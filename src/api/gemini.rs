// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use reqwest;
use serde_json::json;
use std::error::Error;
use std::env;

#[derive(Clone)]
pub struct GeminiClient {
    client: reqwest::Client,
    project_id: String,
    location: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let project_id = env::var("GOOGLE_CLOUD_PROJECT")
            .expect("GOOGLE_CLOUD_PROJECT environment variable not set");
        
        Ok(Self {
            client: reqwest::Client::new(),
            project_id,
            location: "us-central1".to_string(),
        })
    }

    pub async fn research_topic(&self, topic: &str, depth: u32) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            r#"Research this topic in depth.
            Topic: {}
            Research depth level: {}
            
            Requirements:
            1. Find latest developments (last 6 months)
            2. Identify key researchers and labs
            3. Link to papers and code repositories
            4. Note technical limitations
            5. Suggest promising directions
            
            Format as a detailed technical analysis.
            Include links to sources."#,
            topic,
            depth
        );

        let url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/gemini-1.5-pro-002:streamGenerateContent",
            self.location, self.project_id, self.location
        );

        let payload = json!({
            "contents": [{
                "role": "user",
                "parts": [{
                    "text": prompt
                }]
            }],
            "generation_config": {
                "temperature": 0.5,
                "topP": 0.95,
                "maxOutputTokens": 8192
            },
            "safety_settings": [
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_NONE"
                },
                {
                    "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                    "threshold": "BLOCK_NONE" 
                },
                {
                    "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", // say what? 
                    "threshold": "BLOCK_NONE"
                },
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_NONE"
                }
            ]
        });

        let timeout_duration = std::time::Duration::from_secs(30);
        
        let response = match tokio::time::timeout(
            timeout_duration,
            self.client
                .post(&url)
                .json(&payload)
                .send()
        ).await {
            Ok(result) => match result {
                Ok(response) => response,
                Err(e) => return Err(format!("Request error: {}", e).into())
            },
            Err(_) => return Err("Request timed out after 30 seconds".into())
        };

        let json_response = match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            response.json::<serde_json::Value>()
        ).await {
            Ok(result) => match result {
                Ok(json) => json,
                Err(e) => return Err(format!("JSON parsing error: {}", e).into())
            },
            Err(_) => return Err("JSON parsing timed out".into())
        };

        let text = json_response
            .get("candidates")
            .and_then(|candidates| candidates.get(0))
            .and_then(|first| first.get("content"))
            .and_then(|content| content.get("parts"))
            .and_then(|parts| parts.get(0))
            .and_then(|part| part.get("text"))
            .and_then(|text| text.as_str())
            .ok_or("Invalid response structure")?;

        Ok(text.to_string())
    }
}
