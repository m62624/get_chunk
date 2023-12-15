use super::data_size_format::{
    ies_format::{IECSize, IECUnit},
    si_format::{SISize, SIUnit},
};

use std::time::{Duration, Instant};
use sysinfo::RefreshKind;
use sysinfo::{System, SystemExt};

pub mod iterator;
pub mod stream;

#[derive(Debug)]
/// A structure that stores the information needed to determine the `optimal` chunk size
pub struct Memory {
    /// Here we store an object that can store various data about the system
    system_info: System,
    /// Information about the free and total space in RAM.
    ram_available: f64,
}

impl Memory {
    /// Create a new object that stores information about the free and total space on the disk. \
    /// RAM information is also stored here. The information is fixed, before use, update the data via `update_info()`
    fn new() -> Self {
        let system_info = System::new_with_specifics(RefreshKind::new().with_memory());
        // only RAM tracking
        Self {
            ram_available: system_info.available_memory() as f64,
            system_info,
        }
    }
}

mod chunk {

    #[derive(Debug)]
    pub struct Chunk {
        pub value: Vec<u8>,
        pub bytes_per_second: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ChunkSize {
        Auto,
        Percent(f64),
        Bytes(usize),
    }

    #[derive(Debug)]
    pub struct ChunkInfo {
        pub now_bytes_per_second: f64,
        pub mode: ChunkSize,
        pub prev_bytes_per_second: f64,
    }

    #[derive(Debug)]
    pub struct FileInfo {
        pub size: f64,
        pub chunk_info: ChunkInfo,
    }

    impl FileInfo {
        pub fn new(size: f64, start_position: usize) -> Self {
            Self {
                size,
                chunk_info: ChunkInfo::default(),
            }
        }
    }

    impl Default for FileInfo {
        fn default() -> Self {
            Self {
                size: 0.0,
                chunk_info: ChunkInfo::default(),
            }
        }
    }

    impl Default for ChunkInfo {
        fn default() -> Self {
            Self {
                now_bytes_per_second: -1.0,
                mode: ChunkSize::Auto,
                prev_bytes_per_second: -1.0,
            }
        }
    }

    impl ChunkSize {
        pub fn calculate_chunk(prev: f64, now: f64, size: f64, ram: f64, mode: ChunkSize) -> f64 {
            match mode {
                ChunkSize::Auto => {
                    if prev > 0.0 {
                        if now > 0.0 {
                            if now < prev {
                                ChunkSize::decrease_chunk(ram, prev, now)
                            } else {
                                ChunkSize::increase_chunk(ram, prev, now)
                            }
                        } else {
                            prev
                        }
                    } else {
                        ChunkSize::default_chunk_size(size, ram)
                    }
                }
                ChunkSize::Percent(percent) => ChunkSize::percentage_chunk(size, ram, percent),
                ChunkSize::Bytes(bytes) => ChunkSize::bytes_chunk(size, ram, bytes),
            }
        }

        fn increase_chunk(
            ram_available: f64,
            prev_bytes_per_second: f64,
            now_bytes_per_second: f64,
        ) -> f64 {
            (prev_bytes_per_second
                * (1.0
                    + ((now_bytes_per_second - prev_bytes_per_second) / prev_bytes_per_second)
                        .min(0.15)))
            .min(ram_available * 0.85)
            .min(f64::MAX)
        }

        fn decrease_chunk(
            ram_available: f64,
            prev_bytes_per_second: f64,
            now_bytes_per_second: f64,
        ) -> f64 {
            (prev_bytes_per_second
                * (1.0
                    - ((prev_bytes_per_second - now_bytes_per_second) / prev_bytes_per_second)
                        .min(0.45)))
            .min(ram_available * 0.85)
            .min(f64::MAX)
        }

        fn default_chunk_size(file_size: f64, ram_available: f64) -> f64 {
            (file_size * (0.1 / 100.0))
                .min(ram_available * 0.85)
                .min(f64::MAX)
        }

        fn percentage_chunk(file_size: f64, ram_available: f64, percentage: f64) -> f64 {
            (file_size * (percentage.min(100.0).max(0.1) / 100.0)).min(ram_available * 0.85)
        }

        fn bytes_chunk(file_size: f64, ram_available: f64, bytes: usize) -> f64 {
            (file_size * (bytes.min(file_size as usize) as f64 / 100.0)).min(ram_available * 0.85)
        }
    }
}
