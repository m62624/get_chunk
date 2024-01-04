use super::data_chunk::{Chunk, ChunkSize, FileInfo};
use super::Memory;

use std::io::Seek;
use std::time::Instant;

use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[cfg_attr(feature = "debug", derive(Debug))]
struct FilePack<R>
where
    R: Read + Seek,
{
    metadata: FileInfo,
    buffer: BufReader<R>,
    read_complete: bool,
}

impl FilePack<File> {
    fn new(buffer: BufReader<File>, start_position: usize) -> io::Result<FilePack<File>> {
        Ok(FilePack {
            metadata: FileInfo::new(buffer.get_ref().metadata()?.len() as f64, start_position),
            buffer,
            read_complete: false,
        })
    }

    fn create_buffer(path: &str) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(path)?))
    }
}

impl FilePack<io::Cursor<Vec<u8>>> {
    fn new(
        buffer: BufReader<io::Cursor<Vec<u8>>>,
        start_position: usize,
    ) -> io::Result<FilePack<io::Cursor<Vec<u8>>>> {
        Ok(FilePack {
            metadata: FileInfo::new(buffer.get_ref().get_ref().len() as f64, start_position),
            buffer,
            read_complete: false,
        })
    }

    pub fn create_buffer(bytes: Vec<u8>) -> io::Result<BufReader<io::Cursor<Vec<u8>>>> {
        Ok(BufReader::new(io::Cursor::new(bytes)))
    }
}

impl<R: Read + Seek> FilePack<R> {
    fn read_chunk(&mut self) -> io::Result<Chunk> {
        let mut buffer = Vec::new();
        let timer = Instant::now();
        self.buffer
            .get_mut()
            .take(self.metadata.chunk_info.prev_bytes_per_second.max(1.0) as u64)
            .read_to_end(&mut buffer)?;
        let timer = timer.elapsed();
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
pub struct FileIter<R: Seek + Read> {
    memory: Memory,
    file: FilePack<R>,
}

impl FileIter<File> {
    /// Creates a new `FileIter` instance. The default setting is automatic detection of the chunk size
    ///
    /// ---
    /// ⚙️ If you prefer not to specify the file path directly in `new`, you can use `TryFrom` with various input types.
    ///
    /// ---
    /// ### Arguments
    /// * `path` - A path to the file.
    /// ## Example
    /// ```
    /// use get_chunk::iterator::FileIter;
    /// use get_chunk::data_size_format::iec::IECUnit;
    ///
    /// fn main() -> std::io::Result<()> {
    ///
    ///     let file_iter = FileIter::new("file.txt")?;
    ///     for chunk in file_iter {
    ///         match chunk {
    ///             Ok(data) => {
    ///               // some calculations with chunk
    ///               //.....
    ///               println!("{}", IECUnit::auto(data.len() as f64));
    ///             }
    ///             Err(_) => break,
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn new<S: Into<Box<str>>>(path: S) -> io::Result<FileIter<File>> {
        Ok(FileIter {
            memory: Memory::new(),
            file: FilePack::<File>::new(FilePack::<File>::create_buffer(&path.into())?, 0)?,
        })
    }
}

impl<R: Seek + Read> FileIter<R> {
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
    pub fn set_start_position_bytes(mut self, position: usize) -> io::Result<Self> {
        self.file.metadata.start_position = position.min(self.file.metadata.size as usize);
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        Ok(self)
    }

    /// Sets the start position for reading the file as a percentage of the total file size.
    ///
    /// ### Arguments
    /// - `position_percent`: The start position as a percentage of the total file size.
    ///
    /// ### Errors
    /// Returns an [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html) indicating success or an [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) if the seek operation fails.
    pub fn set_start_position_percent(mut self, position_percent: f64) -> io::Result<Self> {
        self.file.metadata.start_position =
            (self.file.metadata.size * (position_percent / 100.0)).min(100.0) as usize;
        self.file.buffer.seek(io::SeekFrom::Start(
            self.file.metadata.start_position as u64,
        ))?;
        Ok(self)
    }

    /// Include the available SWAP (available `RAM` + available `SWAP`)
    pub fn include_available_swap(mut self) -> Self {
        self.memory.swap_check = true;
        self
    }
}

impl<R: Seek + Read> Iterator for FileIter<R> {
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

/// Added implementations of conversions from other types
mod impl_try_from {
    use std::borrow::Cow;

    use super::*;

    impl TryFrom<File> for FileIter<File> {
        type Error = io::Error;

        fn try_from(file: File) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<File>::new(BufReader::new(file), 0)?,
            })
        }
    }

    impl TryFrom<BufReader<File>> for FileIter<File> {
        type Error = io::Error;

        fn try_from(buffer: BufReader<File>) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<File>::new(buffer, 0)?,
            })
        }
    }

    impl TryFrom<Vec<u8>> for FileIter<io::Cursor<Vec<u8>>> {
        type Error = io::Error;

        fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<io::Cursor<Vec<u8>>>::new(
                    FilePack::<io::Cursor<Vec<u8>>>::create_buffer(bytes)?,
                    0,
                )?,
            })
        }
    }

    impl TryFrom<&Vec<u8>> for FileIter<io::Cursor<Vec<u8>>> {
        type Error = io::Error;

        fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<io::Cursor<Vec<u8>>>::new(
                    FilePack::<io::Cursor<Vec<u8>>>::create_buffer(bytes.clone())?,
                    0,
                )?,
            })
        }
    }

    impl TryFrom<io::Cursor<Vec<u8>>> for FileIter<io::Cursor<Vec<u8>>> {
        type Error = io::Error;

        fn try_from(buffer: io::Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<io::Cursor<Vec<u8>>>::new(BufReader::new(buffer), 0)?,
            })
        }
    }

    impl TryFrom<BufReader<io::Cursor<Vec<u8>>>> for FileIter<io::Cursor<Vec<u8>>> {
        type Error = io::Error;

        fn try_from(buffer: BufReader<io::Cursor<Vec<u8>>>) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<io::Cursor<Vec<u8>>>::new(buffer, 0)?,
            })
        }
    }

    impl TryFrom<&[u8]> for FileIter<io::Cursor<Vec<u8>>> {
        type Error = io::Error;

        fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
            Ok(FileIter {
                memory: Memory::new(),
                file: FilePack::<io::Cursor<Vec<u8>>>::new(
                    FilePack::<io::Cursor<Vec<u8>>>::create_buffer(bytes.to_vec())?,
                    0,
                )?,
            })
        }
    }

    #[cfg(not(tarpaulin_include))]
    impl TryFrom<&str> for FileIter<File> {
        type Error = io::Error;

        fn try_from(path: &str) -> Result<Self, Self::Error> {
            FileIter::new(path)
        }
    }

    #[cfg(not(tarpaulin_include))]
    impl TryFrom<String> for FileIter<File> {
        type Error = io::Error;

        fn try_from(path: String) -> Result<Self, Self::Error> {
            FileIter::new(path)
        }
    }

    #[cfg(not(tarpaulin_include))]
    impl TryFrom<Cow<'_, str>> for FileIter<File> {
        type Error = io::Error;

        fn try_from(path: Cow<'_, str>) -> Result<Self, Self::Error> {
            FileIter::new(path)
        }
    }
}
