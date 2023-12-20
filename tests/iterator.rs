mod temp_files;
use temp_files::{FileTest, FILE_TEST};

#[cfg(feature = "size_format")]
mod size_format {
    use super::*;
    use get_chunk::data_size_format::ies::{IECSize, IECUnit};
    use get_chunk::iterator::FileIter;
    use get_chunk::ChunkSize;
    use std::io;

    #[test]
    pub fn iter_t_0() -> io::Result<()> {
        let chunk_size = 150.0;
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
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

    #[test]
    pub fn iter_t_1() -> io::Result<()> {
        let chunk_size = 144.0;
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(960.0, IECSize::Kibibyte).get_values().1,
        )?;

        let file_iter = FileIter::new(file_orig.path.as_str())?.set_mode(ChunkSize::Percent(15.0));

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

    #[test]
    pub fn iter_t_2() -> io::Result<()> {
        let file_orig = FileTest::create_file_with_size(
            FILE_TEST,
            IECUnit::new(700.0, IECSize::Kibibyte).get_values().1,
        )?;

        let mut file_from_chunks = FileTest::default();
        for chunk in FileIter::new(file_orig.path.as_str())? {
            chunk.map(|data| file_from_chunks.write_bytes_to_file(&data).ok())?;
        }
        assert_eq!(file_orig, file_from_chunks);
        Ok(())
    }
}
