use sysinfo::{MemoryRefreshKind, RefreshKind, System};

///
/// ## Version: Sync
///
/// This module defines the [FileIter](iterator::FileIter) struct, which represents a synchronous file processing unit.
/// It is designed to work as an iterator, reading chunks of data from a file and providing information
/// about the read chunks. The synchronous version is suitable for scenarios where asynchronous
/// processing is not a requirement.
pub mod iterator;

///
/// ## Version: Async
///
/// This module defines the [FileStream](stream::FileStream) struct, which represents an asynchronous file processing unit.
/// It is designed to work as an iterator, reading chunks of data from a file and providing information
/// about the read chunks. The asynchronous version is suitable for scenarios where asynchronous
/// processing is a requirement.
#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
pub mod stream;

#[cfg_attr(feature = "debug", derive(Debug))]
/// A structure that stores the information needed to determine the `optimal` chunk size
pub struct Memory {
    /// Here we store an object that can store various data about the system
    system_info: System,
    /// Information about the free and total space in RAM.
    ram_available: f64,
    swap_check: bool,
}

impl Memory {
    fn new() -> Self {
        // only RAM tracking
        Self {
            ram_available: 0.0,
            system_info: System::new_with_specifics(
                RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
            ),
            swap_check: false,
        }
    }

    fn update_ram(&mut self) {
        self.system_info
            .refresh_memory_specifics(match self.swap_check {
                true => MemoryRefreshKind::new().with_ram().with_swap(),
                false => MemoryRefreshKind::new().with_ram().without_swap(),
            });
        self.ram_available = if self.swap_check {
            (self.system_info.available_memory() + self.system_info.free_swap()) as f64
        } else {
            self.system_info.available_memory() as f64
        };
    }
}

pub mod data_chunk {

    #[cfg_attr(feature = "debug", derive(Debug))]

    pub struct Chunk {
        pub value: Vec<u8>,
        pub bytes_per_second: f64,
    }

    /// The `ChunkSize` enum represents different modes for determining the chunk size in the file processing module.
    /// Regardless of the specific mode chosen, all modes adhere to the rules of the [Auto](ChunkSize::Auto) mode with RAM constraints.
    #[derive(Debug, Clone, Copy)]
    pub enum ChunkSize {
        /// Automatically determines an optimal chunk size based on previous read times and available RAM,
        /// ensuring it does not exceed 85% of the available RAM per iteration.
        Auto,
        /// Specifies the chunk size as a percentage of the total file size, considering RAM checks.
        Percent(f64),
        /// Allows users to manually set the chunk size in bytes, subject to RAM constraints.
        Bytes(usize),
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    pub struct ChunkInfo {
        pub now_bytes_per_second: f64,
        pub mode: ChunkSize,
        pub prev_bytes_per_second: f64,
    }

    #[cfg_attr(feature = "debug", derive(Debug))]

    pub struct FileInfo {
        pub size: f64,
        pub start_position: usize,
        pub chunk_info: ChunkInfo,
    }

    impl FileInfo {
        pub fn new(size: f64, start_position: usize) -> Self {
            Self {
                size,
                start_position,
                chunk_info: ChunkInfo::default(),
            }
        }
    }

    impl Default for FileInfo {
        fn default() -> Self {
            Self {
                size: 0.0,
                start_position: 0,
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
        pub(crate) fn calculate_chunk(
            prev: f64,
            now: f64,
            size: f64,
            ram: f64,
            mode: ChunkSize,
        ) -> f64 {
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
