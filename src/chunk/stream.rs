use std::future::Future;

use super::data_chunk::{Chunk, ChunkSize, FileInfo};
use super::Memory;

use tokio::io::AsyncSeekExt;
use tokio::time::Instant;

use tokio::task::{self, JoinHandle};
use tokio::{
    fs::File,
    io::{self, AsyncReadExt, BufReader},
};

use tokio_stream::Stream;
pub use tokio_stream::StreamExt;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default)]
struct FilePack {
    metadata: FileInfo,
    buffer: Option<BufReader<File>>,
    read_complete: bool,
}

impl FilePack {
    async fn new(buffer: BufReader<File>, start_position: usize) -> io::Result<FilePack> {
        Ok(FilePack {
            metadata: FileInfo::new(
                buffer.get_ref().metadata().await?.len() as f64,
                start_position,
            ),
            buffer: Some(buffer),
            read_complete: false,
        })
    }

    async fn create_buffer(path: &str) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(path).await?))
    }

    async fn read_chunk(mut self) -> io::Result<(Chunk, Self)> {
        let mut buffer = Vec::new();
        match self.buffer.as_mut() {
            Some(buff) => {
                let timer = Instant::now();
                match buff
                    .take(self.metadata.chunk_info.prev_bytes_per_second.max(1.0) as u64)
                    .read_to_end(&mut buffer)
                    .await
                {
                    Ok(_) => {
                        let timer = timer.elapsed();
                        if buffer.is_empty() {
                            self.read_complete = true;
                        }
                        Ok((
                            Chunk {
                                bytes_per_second: if !timer.is_zero() {
                                    buffer.len() as f64 / timer.as_secs_f64()
                                } else {
                                    self.metadata.chunk_info.prev_bytes_per_second
                                },
                                value: buffer,
                            },
                            self,
                        ))
                    }
                    Err(e) => Err(e),
                }
            }
            None => Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "buffer is empty",
            )),
        }
    }
}

type Task = Option<JoinHandle<io::Result<(Chunk, FilePack)>>>;

/// The `FileStream` provides an asynchronous file stream designed to read data chunks from a file.
///
/// It operates in two modes:
/// 1. **[`Auto Mode`](super::data_chunk::ChunkSize::Auto) (default):** Dynamically determines an optimal chunk size based on the previous read time,
///    adjusting it relative to the available RAM (85% available per iteration, i.e.,
///    if a chunk is too big and the system cannot process it, it is cut down to 85%.).
/// 2. **[`Fixed Size Mode`](super::data_chunk::ChunkSize):** Allows users to manually set the chunk size, with any remaining data carried over
///    to the next iteration as a single chunk.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FileStream {
    memory: Memory,
    file: FilePack,
    current_task: Task,
}

impl FileStream {
    /// Creates a new `FileIter` instance. The default setting is automatic detection of the chunk size
    /// ### Arguments
    /// * `path` - A path to the file.
    /// ## Example
    /// ```
    /// use get_chunk::data_size_format::iec::IECUnit;
    /// use get_chunk::stream::{FileStream, StreamExt};
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///
    ///     let mut file_stream = FileStream::new("file.txt").await?;
    ///     while let Ok(chunk) = file_stream.try_next().await {
    ///         match chunk {
    ///             Some(data) => {
    ///                 // some calculations with chunk
    ///                 // .....
    ///                 println!("{}", IECUnit::auto(data.len() as f64));
    ///             }
    ///             None => {
    ///                 println!("End of file");
    ///                 break;
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn new<S: Into<Box<str>>>(path: S) -> io::Result<FileStream> {
        Ok(FileStream {
            memory: Memory::new(),
            file: FilePack::new(FilePack::create_buffer(&path.into()).await?, 0).await?,
            current_task: None,
        })
    }

    /// Checks if the read operation is complete, returning `true` if the data buffer is empty.
    ///
    /// ---
    /// **⚠️ Warning**\
    /// This method does not guarantee that the entire file has been read. If the contents
    /// of the file are modified or deleted during iterations, this method may still return `true`.
    pub fn is_read_complete(&self) -> bool {
        self.file.read_complete
    }

    /// Returns the size of the file in bytes.
    ///
    /// ---
    /// Use [`data_size_format`](crate::data_size_format) for comfortable reading and for calculating size
    pub fn get_file_size(&self) -> f64 {
        self.file.metadata.size
    }

    /// Defines the mode of dividing the file into chunks, automatic mode or fixed size
    ///
    /// ### Arguments
    /// - [`mode`](crate::ChunkSize): The processing mode to be set.
    pub fn set_mode(mut self, mode: ChunkSize) -> Self {
        self.file.metadata.chunk_info.mode = mode;
        self
    }

    /// Sets the start position for reading the file in bytes.
    ///
    /// ### Arguments
    /// - `position`: The start position in bytes.
    ///
    /// ### Errors
    /// Returns an [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html) indicating success or an [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) if the seek operation fails.
    pub async fn set_start_position_bytes(mut self, position: usize) -> io::Result<Self> {
        self.file.metadata.start_position = position.min(self.file.metadata.size as usize);

        match self.file.buffer.as_mut() {
            Some(buff) => {
                buff.seek(io::SeekFrom::Start(
                    self.file.metadata.start_position as u64,
                ))
                .await?;
                Ok(self)
            }
            None => Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "buffer is empty",
            )),
        }
    }

    /// Sets the start position for reading the file as a percentage of the total file size.
    ///
    /// ### Arguments
    /// - `position_percent`: The start position as a percentage of the total file size.
    ///
    /// ### Errors
    /// Returns an [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html) indicating success or an [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) if the seek operation fails.
    pub async fn set_start_position_percent(mut self, position_percent: f64) -> io::Result<Self> {
        /*
            self.file.metadata.start_position =
            (self.file.metadata.size * (position_percent / 100.0)).min(100.0) as usize;
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        Ok(self)
         */
        self.file.metadata.start_position =
            (self.file.metadata.size * (position_percent / 100.0)).min(100.0) as usize;
        match self.file.buffer.as_mut() {
            Some(buff) => {
                buff.seek(io::SeekFrom::Start(
                    self.file.metadata.start_position as u64,
                ))
                .await?;
                Ok(self)
            }
            None => Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "buffer is empty",
            )),
        }

        // self.file.buffer.as_mut().map(|buff| async {
        //     buff.seek(io::SeekFrom::Start(
        //         self.file.metadata.start_position as u64,
        //     ))
        //     .await?;

        //     println!(
        //         "current seek: {}",
        //         buff.seek(io::SeekFrom::Current(0)).await?
        //     );
        //     Ok::<(), io::Error>(())
        // });
    }

    /// Include the available SWAP (available `RAM` + available `SWAP`)
    pub fn include_available_swap(mut self) -> Self {
        self.memory.swap_check = true;
        self
    }
}

impl Stream for FileStream {
    type Item = io::Result<Vec<u8>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // Оптимальный размер чанка за один вызов `poll_next`
        let this = self.get_mut();
        this.file.metadata.chunk_info.prev_bytes_per_second = ChunkSize::calculate_chunk(
            this.file.metadata.chunk_info.prev_bytes_per_second,
            this.file.metadata.chunk_info.now_bytes_per_second,
            this.file.metadata.size,
            {
                this.memory.update_ram();
                this.memory.ram_available
            },
            this.file.metadata.chunk_info.mode,
        );
        if this.current_task.is_none() {
            this.current_task = Some(task::spawn(std::mem::take(&mut this.file).read_chunk()));
        }
        match this.current_task.as_mut() {
            Some(task) => {
                tokio::pin!(task);
                match task.poll(cx) {
                    std::task::Poll::Ready(task_status) => match task_status {
                        Ok(inner) => match inner {
                            Ok((chunk, filepack)) => {
                                this.current_task = None;
                                this.file = filepack;
                                this.file.metadata.chunk_info.now_bytes_per_second =
                                    chunk.bytes_per_second;
                                if !chunk.value.is_empty() {
                                    std::task::Poll::Ready(Some(Ok(chunk.value)))
                                } else {
                                    std::task::Poll::Ready(None)
                                }
                            }
                            Err(e) => {
                                this.current_task = None;
                                std::task::Poll::Ready(Some(Err(e)))
                            }
                        },
                        Err(e) => {
                            this.current_task = None;
                            std::task::Poll::Ready(Some(Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                e,
                            ))))
                        }
                    },
                    std::task::Poll::Pending => std::task::Poll::Pending,
                }
            }
            None => std::task::Poll::Ready(None),
        }
    }
}
