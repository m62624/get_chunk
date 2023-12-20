use super::data_chunk::{Chunk, ChunkSize, FileInfo};
use super::Memory;

use std::io::Seek;
use std::time::Instant;

use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[cfg_attr(feature = "debug", derive(Debug))]
struct FilePack {
    metadata: FileInfo,
    buffer: BufReader<File>,
    read_complete: bool,
}

impl FilePack {
    fn new(buffer: BufReader<File>, start_position: usize) -> io::Result<FilePack> {
        Ok(FilePack {
            metadata: FileInfo::new(buffer.get_ref().metadata()?.len() as f64, start_position),
            buffer,
            read_complete: false,
        })
    }

    fn create_buffer(path: &str) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(path)?))
    }

    fn read_chunk(&mut self) -> io::Result<Chunk> {
        let mut buffer = Vec::new();
        let timer = Instant::now();
        self.buffer
            .get_mut()
            .take(self.metadata.chunk_info.prev_bytes_per_second.max(1.0) as u64)
            .read_to_end(&mut buffer)?;
        let timer = timer.elapsed();
        // stop
        if buffer.is_empty() {
            self.read_complete = true;
        }
        Ok(Chunk {
            bytes_per_second: if !timer.is_zero() {
                buffer.len() as f64 / timer.as_secs_f64()
            } else {
                self.metadata.chunk_info.prev_bytes_per_second
            },
            value: buffer,
        })
    }
}

/// The `FileIter` provides a synchronous file iterator designed to read data chunks from a file.
///
/// It operates in two modes:
/// 1. **[`Auto Mode`](super::data_chunk::ChunkSize::Auto) (default):** Dynamically determines an optimal chunk size based on the previous read time,
///    adjusting it relative to the available RAM (85% available per iteration, i.e.,
///    if a chunk is too big and the system cannot process it, it is cut down to 85%.).
/// 2. **[`Fixed Size Mode`](super::data_chunk::ChunkSize):** Allows users to manually set the chunk size, with any remaining data carried over
///    to the next iteration as a single chunk.

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct FileIter {
    memory: Memory,
    file: FilePack,
}

impl FileIter {
    /// Creates a new `FileIter` instance.
    /// # Arguments
    /// * `path` - A path to the file.
    /// # Example
    /// ```
    /// use get_chunk::iterator::FileIter;
    /// 
    /// fn main() -> std::io::Result<()> {
    /// 
    ///     let file_iter = FileIter::new("file.txt")?;
    ///     for chunk in file_iter {
    ///         match chunk {
    ///             Ok(chunk) => {
    ///                 // some calculations with chunk
    ///             }
    ///             Err(_) => break,
    ///         }
    ///     }
    /// 
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn new<S: Into<Box<str>>>(path: S) -> io::Result<FileIter> {
        Ok(FileIter {
            memory: Memory::new(),
            file: FilePack::new(FilePack::create_buffer(&path.into())?, 0)?,
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
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        Ok(self)
    }

    pub fn set_start_position_percent(mut self, position_percent: f64) -> io::Result<Self> {
        self.file.metadata.start_position =
            (self.file.metadata.size * (position_percent / 100.0)).min(100.0) as usize;
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        Ok(self)
    }
}

impl Iterator for FileIter {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.file.metadata.chunk_info.prev_bytes_per_second = ChunkSize::calculate_chunk(
            self.file.metadata.chunk_info.prev_bytes_per_second,
            self.file.metadata.chunk_info.now_bytes_per_second,
            self.file.metadata.size,
            {
                self.memory.update_ram();
                self.memory.ram_available
            },
            self.file.metadata.chunk_info.mode,
        );
        match self.file.read_chunk() {
            Ok(chunk) => {
                self.file.metadata.chunk_info.now_bytes_per_second = chunk.bytes_per_second;
                if !chunk.value.is_empty() {
                    Some(Ok(chunk.value))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}
