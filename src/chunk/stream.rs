use std::future::Future;

use super::chunk::{Chunk, ChunkSize, FileInfo};
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

#[derive(Debug)]
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
                        return Ok((
                            Chunk {
                                bytes_per_second: if !timer.is_zero() {
                                    buffer.len() as f64 / timer.as_secs_f64()
                                } else {
                                    self.metadata.chunk_info.prev_bytes_per_second
                                },
                                value: buffer,
                            },
                            self,
                        ));
                    }
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "buffer is empty",
                ))
            }
        }
    }
}

impl Default for FilePack {
    fn default() -> Self {
        Self {
            metadata: FileInfo::default(),
            buffer: None,
            read_complete: false,
        }
    }
}

type Task = Option<JoinHandle<io::Result<(Chunk, FilePack)>>>;
pub struct FileStream {
    memory: Memory,
    file: FilePack,
    current_task: Task,
}

impl FileStream {
    pub async fn new<S: Into<Box<str>>>(path: S) -> io::Result<FileStream> {
        Ok(FileStream {
            memory: Memory::new(),
            file: FilePack::new(FilePack::create_buffer(&path.into()).await?, 0).await?,
            current_task: None,
        })
    }

    pub fn is_read_complete(&self) -> bool {
        self.file.read_complete
    }

    pub fn get_file_size(&self) -> f64 {
        self.file.metadata.size
    }

    pub fn set_mode(mut self, mode: ChunkSize) -> Self {
        self.file.metadata.chunk_info.mode = mode;
        self
    }

    pub fn set_start_position_bytes(mut self, position: usize) -> io::Result<Self> {
        self.file.metadata.start_position = position.min(self.file.metadata.size as usize);
        self.file.buffer.as_mut().map(|buff| async {
            buff.seek(io::SeekFrom::Start(
                self.file.metadata.start_position as u64,
            ))
            .await?;
            Ok::<(), io::Error>(())
        });
        Ok(self)
    }

    pub fn set_start_position_percent(mut self, position_percent: f64) -> io::Result<Self> {
        self.file.metadata.start_position =
            (self.file.metadata.size as f64 * (position_percent / 100.0)).min(100.0) as usize;
        self.file.buffer.as_mut().map(|buff| async {
            buff.seek(io::SeekFrom::Start(
                self.file.metadata.start_position as u64,
            ))
            .await?;
            Ok::<(), io::Error>(())
        });
        Ok(self)
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
