# Changelog

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
