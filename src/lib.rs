//! # **About**
//!
//! `get_chunk` is a library designed to create file iterators or streams (asynchronous iterators).
//! It specializes in efficient file chunking, allowing users to retrieve chunks of data each time `get_chunk` is called.
//!
//! **Key Features:**
//! - **File Chunking:** Seamlessly divide files, including large ones, into chunks with each "Next" iteration.
//! - **Automatic chunking:** Dynamically adjusts chunk sizes for optimal performance.
//! Efficiently manages memory by limiting large chunks to 85% of available free RAM.
//! - **Modes:** Choose between automatic or manual tuning based on percentage or number of bytes.
//!
//! ---
//! ***Important Notice:***
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
#[cfg(feature = "size_format")]
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
pub use chunk::stream;
