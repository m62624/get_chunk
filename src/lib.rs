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
