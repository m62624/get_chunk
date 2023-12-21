#![cfg_attr(docsrs, feature(doc_cfg))]
//! # **About**
//!
//! `get_chunk` is a library designed to create file iterators or streams (asynchronous iterators).
//! The main task, the ability to retrieve chunk data especially from large files.
//!
//! **Key Features:**
//! - **File Chunking:** Seamlessly divide files, including large ones, into chunks with each "Next" iteration.
//! - **Modes:** Choose between automatic or manual tuning based on percentage or number of bytes.
//!
//! - **Automatic chunking:** Each "Next" iteration dynamically determines an optimal chunk size, facilitating efficient handling of even large files.
//! > ⚠️ The [algorithm](#how-it-works) adjusts chunk sizes for optimal performance after the "Next" call,
//! taking into account available RAM. However, crucially, this adjustment occurs only after
//! the current chunk is sent and before the subsequent "Next" call.
//! > It's important to note a potential scenario: Suppose a chunk is 15GB, and there's initially 16GB of free RAM.
//! If, between the current and next "Next" calls, 2GB of RAM becomes unexpectedly occupied,
//! the current 15GB chunk will still be processed. This situation introduces a risk,
//! as the system might either reclaim resources (resulting in io::Error) or lead to a code crash.
//!
//! ---
//! ⚠️ ***Important Notice:***
//!
//! Iterators created by `get_chunk` do not store the entire file in memory, especially for large datasets.
//! Their purpose is to fetch data from files, even when dealing with substantial sizes, by reading in chunks.
//!
//! **Key Points:**
//! - **Limited File Retention:** Creating an iterator for a small file might result in obtaining all data, depending on the OS.
//!   However, this doesn't guarantee file persistence after iterator creation.
//! - **Deletion Warning:** Deleting a file during iterator or stream iterations will result in an error.
//!   These structures do not track the last successful position.
//! - **No File Restoration:** Attempting to restore a deleted file during iterations is not supported.
//!   These structures do not keep track of the file's original state.
//!
//! ---
//!
//! # How it works
//!
//! The `calculate_chunk` function in the `ChunkSize` enum determines the optimal chunk size based on various parameters. Here's a breakdown of how the size is calculated:
//!
//! The variables `prev` and `now` represent the previous and current read time, respectively.
//!
//! **prev:**
//!
//! *Definition:* `prev` represents the time taken to read a piece of data in the previous iteration.
//!
//! **now:**
//!
//! *Definition:* `now` represents the current time taken to read the data fragment in the current iteration.
//!
//! 1. **Auto Mode:**
//!    - If the previous read time (`prev`) is greater than zero:
//!      - If the current read time (`now`) is also greater than zero:
//!        - If `now` is less than `prev`, decrease the chunk size using `decrease_chunk` method.
//!        - If `now` is greater than or equal to `prev`, increase the chunk size using `increase_chunk` method.
//!      - If `now` is zero or negative, maintain the previous chunk size (`prev`).
//!    - If the previous read time is zero or negative, use the default chunk size based on the file size and available *RAM*.
//!
//! 2. **Percent Mode:**
//!    - Calculate the chunk size as a percentage of the total file size using the `percentage_chunk` method. The percentage is capped between 0.1% and 100%.
//!
//! 3. **Bytes Mode:**
//!    - Calculate the chunk size based on the specified number of bytes using the `bytes_chunk` method. The size is capped by the file size and available *RAM*.
//!
//! ### Key Formulas:
//!
//! - **Increase Chunk Size:**
//!
//! ```rust
//! (prev * (1.0 + ((now - prev) / prev).min(0.15))).min(ram_available * 0.85).min(f64::MAX)
//! ```
//!
//! - **Decrease Chunk Size:**
//!
//! ```rust
//! (prev * (1.0 - ((prev - now) / prev).min(0.45))).min(ram_available * 0.85).min(f64::MAX)
//! ```
//!
//! - **Default Chunk Size:**
//!
//! ```rust
//! (file_size * (0.1 / 100.0)).min(ram_available * 0.85).min(f64::MAX)
//! ```
//!
//! - **Percentage Chunk Size:**
//!
//! ```rust
//! (file_size * (percentage.min(100.0).max(0.1) / 100.0)).min(ram_available * 0.85)
//! ```
//!
//! - **Bytes Chunk Size:**
//!
//! ```rust
//! (file_size * (bytes.min(file_size as usize) as f64 / 100.0)).min(ram_available * 0.85)
//! ```
//!

mod chunk;

pub use chunk::data_chunk::ChunkSize;

/// The module is responsible for the size of the data
///
/// ---
/// Not activated by default `Cargo.toml` must be modified for activations
/// ```
/// get_chunk = { version = "x.y.z", features = [
///     "size_format"
/// ] }
/// ```
/// ## Data Size Units for Convenient Size Specification
///
/// This module provides structures for dealing with data sizes in both the **SI** format (**1000**) and the **IEC** format (**1024**).
///
/// It includes constants for different size thresholds (e.g., kilobytes, megabytes), data structures representing various units of data size (`SIUnit` and `IECUnit`),
/// and methods for convenient conversion and display of data sizes in human-readable formats.
///
/// ### SI Units and Sizes
///
/// - [`SIUnit`](data_size_format::si::SIUnit): Represents different units of data size in the SI format.
/// - [`SISize`](data_size_format::si::SISize): Enum for SI data size categories (e.g., Byte, Kilobyte).
///
/// ### IEC Units and Sizes
///
/// - [`IECUnit`](data_size_format::iec::IECUnit): Represents different units of data size in the IEC format.
/// - [`IECSize`](data_size_format::iec::IECSize): Enum for IEC data size categories (e.g., Byte, Kibibyte).
///
/// ### Conversion between SI and IEC
///
/// The modules provide conversion functions (`From` implementations) between SI and IEC units, enabling seamless interoperability.
///
/// **Note:** These units are intended for convenient size specification and do not store the entire file in memory.
/// Their purpose is to fetch data from files in human-readable formats during iterations or streams, especially for large datasets.
#[cfg(feature = "size_format")]
#[cfg_attr(docsrs, doc(cfg(feature = "size_format")))]
pub mod data_size_format;

///  The module is responsible for retrieval of chunks from a file
pub use chunk::iterator;

/// The module is responsible for **async** retrieval of chunks from a file
///
/// ---
/// Not activated by default `Cargo.toml` must be modified for activations
/// ```
/// get_chunk = { version = "x.y.z", features = [
///     "stream"
/// ] }
/// ```

#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
pub use chunk::stream;
