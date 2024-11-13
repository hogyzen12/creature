use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct KnowledgeBase {
    pub compressed_content: String,
    pub last_updated: DateTime<Utc>,
    pub source_files: Vec<String>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            compressed_content: String::new(),
            last_updated: Utc::now(),
            source_files: Vec::new(),
        }
    }

    pub fn load_files(path: &str) -> Result<Vec<(String, String)>, std::io::Error> {
        let mut files = Vec::new();
        let kb_path = Path::new(path);

        if !kb_path.exists() {
            fs::create_dir_all(kb_path)?;
            return Ok(files);
        }

        for entry in fs::read_dir(kb_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" || ext == "md" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            files.push((
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string(),
                                content
                            ));
                        }
                    }
                }
            }
        }

        Ok(files)
    }
}
