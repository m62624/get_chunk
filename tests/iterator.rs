mod temp_files;
use temp_files::{FileTest, FILE_TEST};

#[cfg(feature = "size_format")]
mod size_format {
    use super::*;
    use get_chunk::data_size_format::iec::{IECSize, IECUnit};
    use get_chunk::iterator::FileIter;
    use get_chunk::ChunkSize;
    use std::io;

    mod set_mode_tests {
        use super::*;

        /// Bytes
        #[test]
        pub fn set_mode_t_0() -> io::Result<()> {
            let chunk_size = 150.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).into(),
            )?;

            let file_iter = FileIter::new(file_orig.path.as_str())?.set_mode(ChunkSize::Bytes(
                IECUnit::new(chunk_size, IECSize::Kibibyte).into(),
            ));

            let mut elements = file_iter.collect::<io::Result<Vec<_>>>()?;
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
        #[test]
        pub fn set_mode_t_1() -> io::Result<()> {
            let chunk_size = 144.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).into(),
            )?;

            let file_iter =
                FileIter::new(file_orig.path.as_str())?.set_mode(ChunkSize::Percent(15.0));

            let mut elements = file_iter.collect::<io::Result<Vec<_>>>()?;
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
        #[test]
        pub fn set_mode_t_2() -> io::Result<()> {
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(700.0, IECSize::Kibibyte).into(),
            )?;

            let mut file_from_chunks = FileTest::default();
            for chunk in FileIter::new(file_orig.path.as_str())? {
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
        #[test]
        pub fn set_start_position_t_0() -> io::Result<()> {
            let file = FileTest::create_with_text(&FILE_TEST, &TEST_TEXT)?;
            let mut file_iter = FileIter::new(file.path.as_str())?
                .set_start_position_percent(50.0)?
                .set_mode(ChunkSize::Bytes(1));

            assert_eq!(
                "I",
                String::from_utf8_lossy(&file_iter.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Error in set_start_position_t_0")
                })??)
            );

            Ok(())
        }

        /// Bytes
        #[test]
        pub fn set_start_position_t_1() -> io::Result<()> {
            let file = FileTest::create_with_text(&FILE_TEST, &TEST_TEXT)?;
            let mut file_iter = FileIter::new(file.path.as_str())?
                .set_start_position_bytes(6)?
                .set_mode(ChunkSize::Bytes(1));

            assert_eq!(
                "w",
                String::from_utf8_lossy(&file_iter.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Error in set_start_position_t_1")
                })??)
            );

            Ok(())
        }
    }
}
