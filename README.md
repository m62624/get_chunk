# get_chunk

## About

`get_chunk` is a library for creating file iterators or streams (asynchronous iterators),
specialized in efficient file chunking. It enables users to retrieve chunks of data with each call to `get_chunk`.

**Key Features:**
- **File Chunking:** Divide files, including large ones, into seamless chunks with each "Next" iteration.
- **Automatic Chunking:** Dynamically adjusts chunk sizes for optimal performance, ensuring efficient memory usage.
  Large chunks are limited to 85% of available free *RAM*.
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

### Iterator version

---
#### Example
```rust
use get_chunk::iterator::FileIter;
// Note: requires a `size_format` attribute.
use get_chunk::data_size_format::iec::IECUnit;

fn main() -> std::io::Result<()> {

    let file_iter = FileIter::new("file.txt")?;
    for chunk in file_iter {
        match chunk {
            Ok(data) => {
              // some calculations with chunk
              //.....
              println!("{}", IECUnit::auto(data.len() as f64));
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}
```

### Stream version

#### Example
```rust

// Note: requires the `size_format` and `stream` attributes.
use get_chunk::data_size_format::iec::IECUnit;
use get_chunk::stream::{FileStream, StreamExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let mut file_stream = FileStream::new("file.txt").await?;
    while let Ok(chunk) = file_stream.try_next().await {
        match chunk {
            Some(data) => {
                // some calculations with chunk
                // .....
                println!("{}", IECUnit::auto(data.len() as f64));
            }
            None => {
                println!("End of file");
            }
        }
    }

    Ok(())
}
```

# How it works

The `calculate_chunk` function in the `ChunkSize` enum determines the optimal chunk size based on various parameters. Here's a breakdown of how the size is calculated:

1. **Auto Mode:**
   - If the previous read time (`prev`) is greater than zero:
     - If the current read time (`now`) is also greater than zero:
       - If `now` is less than `prev`, decrease the chunk size using `decrease_chunk` method.
       - If `now` is greater than or equal to `prev`, increase the chunk size using `increase_chunk` method.
     - If `now` is zero or negative, maintain the previous chunk size (`prev`).
   - If the previous read time is zero or negative, use the default chunk size based on the file size and available *RAM*.

2. **Percent Mode:**
   - Calculate the chunk size as a percentage of the total file size using the `percentage_chunk` method. The percentage is capped between 0.1% and 100%.

3. **Bytes Mode:**
   - Calculate the chunk size based on the specified number of bytes using the `bytes_chunk` method. The size is capped by the file size and available *RAM*.

### Key Formulas:

- **Increase Chunk Size:**

```rust
(prev * (1.0 + ((now - prev) / prev).min(0.15))).min(ram_available * 0.85).min(f64::MAX)
```

- **Decrease Chunk Size:**

```rust
(prev * (1.0 - ((prev - now) / prev).min(0.45))).min(ram_available * 0.85).min(f64::MAX)
```

- **Default Chunk Size:**

```rust
(file_size * (0.1 / 100.0)).min(ram_available * 0.85).min(f64::MAX)
```

- **Percentage Chunk Size:**

```rust
(file_size * (percentage.min(100.0).max(0.1) / 100.0)).min(ram_available * 0.85)
```

- **Bytes Chunk Size:**

```rust
(file_size * (bytes.min(file_size as usize) as f64 / 100.0)).min(ram_available * 0.85)
```
