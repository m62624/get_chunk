mod temp_files;

#[cfg(all(feature = "size_format", feature = "stream"))]
mod size_format {
    use super::*;
    use get_chunk::data_size_format::iec::{IECSize, IECUnit};
    use get_chunk::stream::{FileStream, StreamExt, TryFrom};
    use get_chunk::ChunkSize;
    use std::io;
    use temp_files::{FileTest, FILE_TEST};

    mod set_mode_tests {
        use super::*;

        /// Bytes
        #[tokio::test]
        pub async fn set_mode_t_0() -> io::Result<()> {
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

        /// Percent
        #[tokio::test]
        pub async fn set_mode_t_1() -> io::Result<()> {
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

        /// Auto
        #[tokio::test]
        pub async fn set_mode_t_2() -> io::Result<()> {
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

    mod set_start_position_tests {
        use super::*;
        const TEST_TEXT: &str = "Hello world :D, I'm a test file!";

        /// Percent
        #[tokio::test]
        pub async fn set_start_position_t_0() -> io::Result<()> {
            let file = FileTest::create_with_text(&FILE_TEST, &TEST_TEXT)?;
            let mut file_iter = FileStream::new(file.path.as_str())
                .await?
                .set_start_position_percent(50.0)
                .await?
                .set_mode(ChunkSize::Bytes(1));

            assert_eq!(
                "I",
                String::from_utf8_lossy(&file_iter.next().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Error in set_start_position_t_0")
                })??)
            );

            let mut file_iter = FileStream::new(file.path.as_str())
                .await?
                .set_start_position_percent(420.0)
                .await?
                .set_mode(ChunkSize::Bytes(1));

            assert!(
                file_iter.next().await.is_none(),
                "Error in set_start_position_t_0"
            );

            Ok(())
        }

        /// Bytes
        #[tokio::test]
        pub async fn set_start_position_t_1() -> io::Result<()> {
            let file = FileTest::create_with_text(&FILE_TEST, &TEST_TEXT)?;
            let mut file_iter = FileStream::new(file.path.as_str())
                .await?
                .set_start_position_bytes(6)
                .await?
                .set_mode(ChunkSize::Bytes(1));

            assert_eq!(
                "w",
                String::from_utf8_lossy(&file_iter.next().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Error in set_start_position_t_1")
                })??)
            );

            let mut file_iter = FileStream::new(file.path.as_str())
                .await?
                .set_start_position_bytes(420)
                .await?
                .set_mode(ChunkSize::Bytes(1));

            assert!(
                file_iter.next().await.is_none(),
                "Error in set_start_position_t_1"
            );

            Ok(())
        }
    }

    #[tokio::test]
    async fn get_file_size_t_0() -> io::Result<()> {
        let file = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).into(),
        )?;

        let file_iter = FileStream::new(file.path.as_str()).await?;
        assert_eq!(
            file_iter.get_file_size(),
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1
        );

        Ok(())
    }

    #[tokio::test]
    async fn is_read_complete_t_0() -> io::Result<()> {
        let file = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(900.0, IECSize::Kibibyte).into(),
        )?;

        let mut file_iter = FileStream::new(file.path.as_str())
            .await?
            .set_mode(ChunkSize::Percent(50.0));
        file_iter.next().await;
        assert!(!file_iter.is_read_complete());
        file_iter.next().await;
        assert!(!file_iter.is_read_complete());
        file_iter.next().await;
        assert!(file_iter.is_read_complete());
        Ok(())
    }
    mod impl_try_from {
        use super::*;
        use tokio::fs::File;
        /// impl TryFrom<File> for FileStream<File>
        #[tokio::test]
        async fn impl_try_from_t_0() -> io::Result<()> {
            let chunk_size = 150.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
            )?;
            let file_orig = File::open(file_orig.path.as_str()).await?;
            let file_stream =
                FileStream::try_from_data(file_orig)
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

        /// impl TryFrom<BufReader<File>> for FileStream<File>
        #[tokio::test]
        async fn impl_try_from_t_1() -> io::Result<()> {
            let chunk_size = 150.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
            )?;
            let file_orig = tokio::fs::File::open(file_orig.path.as_str()).await?;
            let file_orig = tokio::io::BufReader::new(file_orig);
            let file_stream =
                FileStream::try_from_data(file_orig)
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

        /// impl TryFrom<Vec<u8>> for FileStream<Cursor<Vec<u8>>>
        #[tokio::test]
        async fn impl_try_from_t_2() -> io::Result<()> {
            let bytes = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter = FileStream::try_from_data(bytes)
                .await?
                .set_mode(ChunkSize::Percent(50.0));
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [72, 101, 108, 108, 111, 44]
            );
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [32, 119, 111, 114, 108, 100]
            );
            assert_eq!(file_iter.next().await.unwrap()?, [33]);

            Ok(())
        }

        /// impl TryFrom<Cursor<Vec<u8>>> for FileStream<Cursor<Vec<u8>>>
        #[tokio::test]
        async fn impl_try_from_t_3() -> io::Result<()> {
            let bytes = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter = FileStream::try_from_data(std::io::Cursor::new(bytes))
                .await?
                .set_mode(ChunkSize::Percent(50.0));
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [72, 101, 108, 108, 111, 44]
            );
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [32, 119, 111, 114, 108, 100]
            );
            assert_eq!(file_iter.next().await.unwrap()?, [33]);

            Ok(())
        }

        ///  impl TryFrom<BufReader<Cursor<Vec<u8>>>> for FileStream<Cursor<Vec<u8>>>
        #[tokio::test]
        async fn impl_try_from_t_4() -> io::Result<()> {
            let bytes = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter =
                FileStream::try_from_data(tokio::io::BufReader::new(std::io::Cursor::new(bytes)))
                    .await?
                    .set_mode(ChunkSize::Percent(50.0));
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [72, 101, 108, 108, 111, 44]
            );
            assert_eq!(
                file_iter.next().await.unwrap()?,
                [32, 119, 111, 114, 108, 100]
            );
            assert_eq!(file_iter.next().await.unwrap()?, [33]);

            Ok(())
        }
    }

    mod chunk_bytes {
        use super::*;

        #[tokio::test]
        async fn chunk_bytes_t_0() -> io::Result<()> {
            let bytes = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter = FileStream::try_from_data(bytes)
                .await?
                .set_mode(ChunkSize::Bytes(4));

            assert_eq!(file_iter.next().await.unwrap()?, [72, 101, 108, 108]);
            assert_eq!(file_iter.next().await.unwrap()?, [111, 44, 32, 119]);
            assert_eq!(file_iter.next().await.unwrap()?, [111, 114, 108, 100]);
            assert_eq!(file_iter.next().await.unwrap()?, [33]);

            Ok(())
        }
    }
}
