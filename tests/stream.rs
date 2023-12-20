mod temp_files;
use temp_files::{FileTest, FILE_TEST};

#[cfg(all(feature = "size_format", feature = "stream"))]
mod size_format {
    use super::*;
    use get_chunk::data_size_format::ies::{IECSize, IECUnit};
    use get_chunk::stream::{FileStream, StreamExt};
    use get_chunk::ChunkSize;
    use std::io;

    #[tokio::test]
    pub async fn iter_t_0() -> io::Result<()> {
        let chunk_size = 150.0;
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
        )?;

        let file_stream =
            FileStream::new(file_orig.path.as_str())
                .await?
                .set_mode(ChunkSize::Bytes(
                    IECUnit::new(chunk_size, IECSize::Kibibyte).into(),
                ));

        let mut elements = file_stream.collect::<io::Result<Vec<_>>>().await?;
        elements.pop();

        for chunk in elements {
            assert_eq!(
                chunk.len(),
                IECUnit::new(chunk_size, IECSize::Kibibyte).get_values().1 as usize
            );
        }
        Ok(())
    }

    #[tokio::test]
    pub async fn iter_t_1() -> io::Result<()> {
        let chunk_size = 144.0;
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
        )?;

        let file_stream = FileStream::new(file_orig.path.as_str())
            .await?
            .set_mode(ChunkSize::Percent(15.0));

        let mut elements = file_stream.collect::<io::Result<Vec<_>>>().await?;
        elements.pop();

        for chunk in elements {
            assert_eq!(
                chunk.len(),
                IECUnit::new(chunk_size, IECSize::Kibibyte).get_values().1 as usize
            );
        }
        Ok(())
    }

    #[tokio::test]
    pub async fn iter_t_2() -> io::Result<()> {
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(700.0, IECSize::Kibibyte).get_values().1,
        )?;

        let mut file_from_chunks = FileTest::default();

        let mut file_stream = FileStream::new(file_orig.path.as_str()).await?;

        while let Some(chunk) = file_stream.next().await {
            chunk.map(|data| file_from_chunks.write_bytes_to_file(&data).ok())?;
        }

        assert_eq!(file_orig, file_from_chunks);
        Ok(())
    }
}
