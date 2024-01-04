# Changelog

## [1.2.0] - 2024.01.04

### Fixed
- Resolved issue with byte mode in `FileIter`. It now correctly handles chunk sizes.

### Added
- Implemented `TryFrom` for `FileIter` with support for various input types:
  - `File` --> `FileIter`
  - `BufReader<File>` --> `FileIter`
  - `Vec<u8>` --> `FileIter`
  - `&Vec<u8>` --> `FileIter`
  - `io::Cursor<Vec<u8>>` --> `FileIter`
  - `BufReader<io::Cursor<Vec<u8>>>` --> `FileIter`
  - `&[u8]` --> `FileIter`
  - `&str` --> `FileIter` (equivalent to the `new` method)
  - `String` --> `FileIter` (equivalent to the `new` method)
  - `Cow<'_, str>` --> `FileIter` (equivalent to the `new` method)

- Implemented `TryFrom` for `FileStream` with custom trait to support async operations (use the `try_from_data` method instead of `try_from`):
  - `File` --> `FileIter`
  - `BufReader<File>` --> `FileIter`
  - `Vec<u8>` --> `FileIter`
  - `io::Cursor<Vec<u8>>` --> `FileIter`
  - `BufReader<io::Cursor<Vec<u8>>>` --> `FileIter`
  - `String` --> `FileIter` (equivalent to the `new` method)


## [1.1.0] - 2023.12.23

### Fixed
- Fixed method for setting the start position of `stream` (set_start_position).
- Added fields to `Cargo.toml` to build documentation with all features
```
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### Added
- Added `include_available_swap` method to include free swap partition/file in the calculation 

### Changed
- Now when creating iterator/stream, only free RAM is considered by default. 


## [1.0.0] - 2023.12.21

### Changed
- Introducing the first stable release of the library.
- Improved and stabilized the *API* compared to previous versions.

### Added
- Implemented new enums and units for both *SI* and *IEC* data size representations.
- Introduced the concept of adaptive chunk size adjustment based on the performance of the file reading process.
- Added functions for creating and working with data size units.
- Added support for iterators and streams to enhance data processing capabilities.
