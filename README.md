# get_chunk

## About

`get_chunk` is a versatile library for creating file iterators or streams (asynchronous iterators),
specialized in efficient file chunking. It enables users to retrieve chunks of data with each call to `get_chunk`.
**Key Features:**
- **File Chunking:** Divide files, including large ones, into seamless chunks with each "Next" iteration.
- **Automatic Chunking:** Dynamically adjusts chunk sizes for optimal performance, ensuring efficient memory usage.
  Large chunks are limited to 85% of available free RAM.
- **Modes:** Choose between automatic tuning or manually set chunk size based on percentage or byte count.
---
⚠️ **Important Notice:**
Iterators created by `get_chunk` don't store the entire file in memory, especially for large datasets.
Their purpose is to fetch data from files in chunks, maintaining efficiency.
**Key Points:**
- **Limited File Retention:** Creating an iterator for a small file might result in fetching all data, OS-dependent.
  However, this doesn't guarantee file persistence after iterator creation.
- **Deletion Warning:** Deleting a file during iterator or stream iterations will result in an error.
  These structures don't track the last successful position.
- **No File Restoration:** Attempting to restore a deleted file during iterations is not supported.
  These structures don't keep track of the file's original state.
```
