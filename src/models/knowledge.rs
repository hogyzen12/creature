// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

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
