use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Generate a unique filename using timestamp and process id
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let filename = format!("temp_{}.tmp", timestamp);
        let file_path = env::temp_dir().join(filename);
        let file = File::create(&file_path)?;

        Ok(TempFile { file_path, file })
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write_all(data)
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.file_path);
    }
}
