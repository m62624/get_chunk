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

            let mut file_iter = FileIter::new(file.path.as_str())?
                .set_start_position_percent(420.0)?
                .set_mode(ChunkSize::Bytes(1));

            assert!(
                file_iter.next().is_none(),
                "Error in set_start_position_t_0"
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

            let mut file_iter = FileIter::new(file.path.as_str())?
                .set_start_position_bytes(420)?
                .set_mode(ChunkSize::Bytes(1));

            assert!(
                file_iter.next().is_none(),
                "Error in set_start_position_t_1"
            );
            Ok(())
        }
    }

    #[test]
    fn get_file_size_t_0() -> io::Result<()> {
        let file = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).into(),
        )?;

        let file_iter = FileIter::new(file.path.as_str())?;
        assert_eq!(
            file_iter.get_file_size(),
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1
        );

        Ok(())
    }

    #[test]
    fn is_read_complete_t_0() -> io::Result<()> {
        let file = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(900.0, IECSize::Kibibyte).into(),
        )?;

        let mut file_iter = FileIter::new(file.path.as_str())?.set_mode(ChunkSize::Percent(50.0));
        file_iter.next();
        assert!(!file_iter.is_read_complete());
        file_iter.next();
        assert!(!file_iter.is_read_complete());
        file_iter.next();
        assert!(file_iter.is_read_complete());
        Ok(())
    }

    mod impl_try_from {
        use std::{fs::File, io::BufReader};

        use super::*;

        ///  impl TryFrom<File> for FileIter<File>
        #[test]
        fn impl_try_from_t_0() -> io::Result<()> {
            let chunk_size = 150.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).into(),
            )?;
            let file_orig = File::open(file_orig.path.as_str())?;
            let file_iter = FileIter::try_from(file_orig)?.set_mode(ChunkSize::Bytes(
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

        ///  impl TryFrom<BufReader<File>> for FileIter<File>
        #[test]
        fn impl_try_from_t_1() -> io::Result<()> {
            let chunk_size = 150.0;
            let file_orig = FileTest::create_file_with_size(
                FILE_TEST,
                IECUnit::new(960.0, IECSize::Kibibyte).into(),
            )?;
            let file_orig = BufReader::new(File::open(file_orig.path.as_str())?);
            let file_iter = FileIter::try_from(file_orig)?.set_mode(ChunkSize::Bytes(
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

        /// impl TryFrom<Vec<u8>> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_2() -> io::Result<()> {
            let bytes: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
            let mut file_iter =
                FileIter::try_from(bytes.as_slice())?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }

        ///  impl TryFrom<Vec<u8>> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_3() -> io::Result<()> {
            let bytes: Vec<u8> =
                [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter = FileIter::try_from(bytes)?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }

        ///  impl TryFrom<&Vec<u8>> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_4() -> io::Result<()> {
            let bytes: Vec<u8> =
                [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec();
            let mut file_iter = FileIter::try_from(&bytes)?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }

        ///   impl TryFrom<io::Cursor<Vec<u8>>> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_5() -> io::Result<()> {
            let bytes: io::Cursor<Vec<u8>> = io::Cursor::new(
                [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec(),
            );
            let mut file_iter = FileIter::try_from(bytes)?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }

        ///  impl TryFrom<BufReader<io::Cursor<Vec<u8>>>> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_6() -> io::Result<()> {
            let bytes: io::Cursor<Vec<u8>> = io::Cursor::new(
                [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].to_vec(),
            );
            let bytes = BufReader::new(bytes);
            let mut file_iter = FileIter::try_from(bytes)?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }

        /// impl TryFrom<&[u8]> for FileIter<io::Cursor<Vec<u8>>>
        #[test]
        fn impl_try_from_t_7() -> io::Result<()> {
            let bytes: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
            let mut file_iter =
                FileIter::try_from(bytes.as_slice())?.set_mode(ChunkSize::Percent(50.0));
            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108, 111, 44]);
            assert_eq!(file_iter.next().unwrap()?, [32, 119, 111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }
    }

    mod chunk_bytes {
        use super::*;

        #[test]
        fn chunk_bytes_t_0() -> io::Result<()> {
            let bytes: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
            let mut file_iter = FileIter::try_from(bytes.as_slice())?.set_mode(ChunkSize::Bytes(4));

            assert_eq!(file_iter.next().unwrap()?, [72, 101, 108, 108]);
            assert_eq!(file_iter.next().unwrap()?, [111, 44, 32, 119]);
            assert_eq!(file_iter.next().unwrap()?, [111, 114, 108, 100]);
            assert_eq!(file_iter.next().unwrap()?, [33]);

            Ok(())
        }
    }
}
