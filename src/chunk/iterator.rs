use super::chunk::{Chunk, FileInfo};

use std::time::Instant;

use std::{
    fs::File,
    io::{self, BufReader, Read},
};

#[derive(Debug)]
struct FilePack {
    metadata: FileInfo,
    buffer: Option<BufReader<File>>,
    read_complete: bool,
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

impl FilePack {
    fn new(buffer: BufReader<File>, start_position: usize) -> io::Result<FilePack> {
        Ok(FilePack {
            metadata: FileInfo::new(buffer.get_ref().metadata()?.len() as f64, start_position),
            buffer: Some(buffer),
            read_complete: false,
        })
    }

    fn create_buffer(path: &str) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(path)?))
    }

    fn read_chunk(&mut self) -> io::Result<Chunk> {
        let mut buffer = Vec::new();
        let timer = Instant::now();
        match self.buffer.as_mut() {
            Some(buf) => {
                buf.take(self.metadata.chunk_info.prev_bytes_per_second.max(1.0) as u64)
                    .read_to_end(&mut buffer)?;
            }
            None => return Err(io::Error::new(io::ErrorKind::Other, "buffer is None")),
        }
        let timer = timer.elapsed();
        // stop
        if buffer.is_empty() {
            self.read_complete = true;
        }
        if !timer.is_zero() {
            Ok(Chunk {
                bytes_per_second: (buffer.len() as f64) / timer.as_secs_f64(),
                value: buffer,
            })
        } else {
            Ok(Chunk {
                bytes_per_second: self.metadata.chunk_info.prev_bytes_per_second,
                value: buffer,
            })
        }
    }
}
