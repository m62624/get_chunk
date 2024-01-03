use super::data_chunk::{Chunk, ChunkSize, FileInfo};
use super::Memory;
use async_convert::{async_trait, TryFrom};
use std::future::Future;

use std::io::Cursor;
use tokio::time::Instant;

use tokio::task::{self, JoinHandle};
use tokio::{
    fs::File,
    io::{self, AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, BufReader},
};

use tokio_stream::Stream;
pub use tokio_stream::StreamExt;

#[cfg_attr(feature = "debug", derive(Debug))]
struct FilePack<R>
where
    R: AsyncRead + Unpin + Send,
{
    metadata: FileInfo,
    buffer: Option<BufReader<R>>,
    read_complete: bool,
}

impl<R> Default for FilePack<R>
where
    R: AsyncRead + Unpin + Send,
{
    fn default() -> Self {
        FilePack {
            metadata: FileInfo::default(),
            buffer: None,
            read_complete: false,
        }
    }
}

impl FilePack<File> {
    async fn new(buffer: BufReader<File>, start_position: usize) -> io::Result<FilePack<File>> {
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
}

impl FilePack<Cursor<Vec<u8>>> {
    async fn new(
        buffer: BufReader<Cursor<Vec<u8>>>,
        start_position: usize,
    ) -> io::Result<FilePack<Cursor<Vec<u8>>>> {
        Ok(FilePack {
            metadata: FileInfo::new(buffer.get_ref().get_ref().len() as f64, start_position),
            buffer: Some(buffer),
            read_complete: false,
        })
    }

    async fn create_buffer(bytes: Vec<u8>) -> io::Result<BufReader<Cursor<Vec<u8>>>> {
        Ok(BufReader::new(Cursor::new(bytes)))
    }
}

impl<R: AsyncRead + Unpin + Send> FilePack<R> {
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

/// The `FileStream` provides an asynchronous file stream designed to read data chunks from a file.
///
/// It operates in two modes:
/// 1. **[`Auto Mode`](super::data_chunk::ChunkSize::Auto) (default):** Dynamically determines an optimal chunk size based on the previous read time,
///    adjusting it relative to the available RAM (85% available per iteration, i.e.,
///    if a chunk is too big and the system cannot process it, it is cut down to 85%.).
/// 2. **[`Fixed Size Mode`](super::data_chunk::ChunkSize):** Allows users to manually set the chunk size, with any remaining data carried over
///    to the next iteration as a single chunk.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FileStream<R>
where
    R: AsyncRead + Unpin + Send,
{
    memory: Memory,
    file: FilePack<R>,
    current_task: Option<JoinHandle<io::Result<(Chunk, FilePack<R>)>>>,
    // current_task: Option<JoinHandle<io::Result<(Chunk, FilePack<R>>>)>,
}

impl FileStream<File> {
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
    pub async fn new<S: Into<Box<str>>>(path: S) -> io::Result<FileStream<File>> {
        Ok(FileStream {
            memory: Memory::new(),
            file: FilePack::<File>::new(FilePack::<File>::create_buffer(&path.into()).await?, 0)
                .await?,
            current_task: None,
        })
    }
}

// #[async_trait]
// impl TryFrom<

// impl FileStream<Cursor<Vec<u8>>> {
//     pub async fn from_bytes(bytes: Vec<u8>) -> io::Result<FileStream<Cursor<Vec<u8>>>> {
//         Ok(FileStream {
//             memory: Memory::new(),
//             file: FilePack::<Cursor<Vec<u8>>>::new(
//                 FilePack::<Cursor<Vec<u8>>>::create_buffer(bytes).await?,
//                 0,
//             )
//             .await?,
//             current_task: None,
//         })
//     }
// }

impl<R: AsyncRead + AsyncSeek + Unpin + Send> FileStream<R> {
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
    }

    /// Include the available SWAP (available `RAM` + available `SWAP`)
    pub fn include_available_swap(mut self) -> Self {
        self.memory.swap_check = true;
        self
    }
}

impl<R: AsyncRead + AsyncSeek + Unpin + Send + 'static> Stream for FileStream<R> {
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
            // let file = Option::take(this.file);
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

mod impl_try_from {
    use super::*;

    #[async_trait]
    impl TryFrom<File> for FileStream<File> {
        type Error = io::Error;

        async fn try_from(file: File) -> Result<Self, Self::Error> {
            Ok(FileStream {
                memory: Memory::new(),
                file: FilePack::<File>::new(BufReader::new(file), 0).await?,
                current_task: None,
            })
        }
    }

    #[async_trait]
    impl TryFrom<BufReader<File>> for FileStream<File> {
        type Error = io::Error;

        async fn try_from(buffer: BufReader<File>) -> Result<Self, Self::Error> {
            Ok(FileStream {
                memory: Memory::new(),
                file: FilePack::<File>::new(buffer, 0).await?,
                current_task: None,
            })
        }
    }

    #[async_trait]
    impl TryFrom<Vec<u8>> for FileStream<Cursor<Vec<u8>>> {
        type Error = io::Error;

        async fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(FileStream {
                memory: Memory::new(),
                file: FilePack::<Cursor<Vec<u8>>>::new(
                    FilePack::<Cursor<Vec<u8>>>::create_buffer(bytes).await?,
                    0,
                )
                .await?,
                current_task: None,
            })
        }
    }

    #[async_trait]
    impl TryFrom<Cursor<Vec<u8>>> for FileStream<Cursor<Vec<u8>>> {
        type Error = io::Error;

        async fn try_from(buffer: Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
            Ok(FileStream {
                memory: Memory::new(),
                file: FilePack::<Cursor<Vec<u8>>>::new(BufReader::new(buffer), 0).await?,
                current_task: None,
            })
        }
    }

    #[async_trait]
    impl TryFrom<BufReader<Cursor<Vec<u8>>>> for FileStream<Cursor<Vec<u8>>> {
        type Error = io::Error;

        async fn try_from(buffer: BufReader<Cursor<Vec<u8>>>) -> Result<Self, Self::Error> {
            Ok(FileStream {
                memory: Memory::new(),
                file: FilePack::<Cursor<Vec<u8>>>::new(buffer, 0).await?,
                current_task: None,
            })
        }
    }

    #[async_trait]
    impl TryFrom<String> for FileStream<File> {
        type Error = io::Error;

        async fn try_from(path: String) -> Result<Self, Self::Error> {
            FileStream::new(path).await
        }
    }
}
