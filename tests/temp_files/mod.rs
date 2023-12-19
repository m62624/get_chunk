const FILE_TEST: &str = "./test_file";
const CHUNK_SIZE: usize = 100000; // Размер порции (в байтах)

use home::home_dir;
use rand::RngCore;
use sha2::Digest;
use sha2::Sha256;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{self, Read, Write};
use std::process;
use uuid::Uuid;

#[derive(Debug)]
pub struct FileTest {
    pub file_path: String,
    pub hash_data: String,
}

impl FileTest {
    pub fn create_empty_file(file_path: &str, size_bytes: f64) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        // Устанавливаем размер файла с помощью seek
        file.seek(SeekFrom::Start(size_bytes as u64))?;

        // Записываем пустой байт в конец файла, чтобы он был создан на диске
        file.write_all(&[0])?;

        Ok(())
    }
    pub fn write_bytes_to_file(&mut self, bytes: &[u8]) -> io::Result<()> {
        // Открываем файл для записи (создание или дополнение)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.file_path.clone())?;

        // Записываем байты в файл
        file.write_all(bytes)?;
        self.update_from_default()?;
        Ok(())
    }

    fn update_from_default(&mut self) -> io::Result<()> {
        self.hash_data = Self::calculate_hash_data(&self.file_path)?;
        Ok(())
    }

    pub fn expand_path(input_path: String) -> String {
        if input_path.starts_with('~') {
            home_dir()
                .map(|home| {
                    let path = if input_path.len() > 2 {
                        home.join(&input_path[2..])
                    } else {
                        home.join(&input_path[1..])
                    };
                    path.display().to_string()
                })
                .unwrap_or_else(|| input_path)
        } else {
            input_path
        }
    }

    fn add_uuid_to_string(input: &str) -> String {
        let uuid = Uuid::new_v4();
        format!("{}-{}", input, uuid)
    }

    fn calculate_hash_data(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;

        let mut hasher = Sha256::new();
        let mut buffer = Vec::new();

        loop {
            buffer.clear(); // Очищаем буфер перед каждым чтением

            let bytes_read = file.read_to_end(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    pub fn create_file_with_size<S: AsRef<str>>(
        path: S,
        size_bytes: f64,
    ) -> Result<FileTest, std::io::Error> {
        let path = path.as_ref();
        let mut file = File::create(path)?;

        let mut rng = rand::thread_rng();
        let mut buffer = vec![0; CHUNK_SIZE];

        for _ in 0..(size_bytes as usize / CHUNK_SIZE) {
            rng.fill_bytes(&mut buffer);
            file.write_all(&buffer)?;
        }

        // Обработка последней порции, если размер не кратен CHUNK_SIZE
        let remaining_bytes = size_bytes as usize % CHUNK_SIZE;
        if remaining_bytes > 0 {
            rng.fill_bytes(&mut buffer[..remaining_bytes]);
            file.write_all(&buffer[..remaining_bytes])?;
        }

        file.flush()?;
        Ok(FileTest {
            file_path: path.to_string(),
            hash_data: Self::calculate_hash_data(path.as_ref())?,
        })
    }
}

impl PartialEq for FileTest {
    fn eq(&self, other: &Self) -> bool {
        self.hash_data == other.hash_data
    }
}

impl Default for FileTest {
    fn default() -> Self {
        Self {
            file_path: Self::add_uuid_to_string(&Self::expand_path(FILE_TEST.to_string())),
            hash_data: Default::default(),
        }
    }
}

impl Drop for FileTest {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.file_path) {
            process::exit(e.raw_os_error().unwrap_or(1));
        }
    }
}
