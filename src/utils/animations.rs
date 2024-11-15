// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationStyle {
    Spinner,  // Classic spinner animation
    Progress  // Simple progress indicator
}

pub struct AnimationConfig {
    pub style: AnimationStyle,
    pub message: String,
    pub delay: Duration,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            style: AnimationStyle::Spinner,
            message: "Processing".to_string(),
            delay: Duration::from_millis(100),
        }
    }
}

pub struct ThinkingAnimation {
    frames: Vec<String>,
    config: AnimationConfig,
}

impl ThinkingAnimation {
    pub fn new(config: AnimationConfig) -> Self {
        let frames = match config.style {
            AnimationStyle::Spinner => vec![
                "⠋".to_string(),
                "⠙".to_string(),
                "⠹".to_string(),
                "⠸".to_string(),
                "⠼".to_string(),
                "⠴".to_string(),
                "⠦".to_string(),
                "⠧".to_string(),
                "⠇".to_string(),
                "⠏".to_string()
            ],
            AnimationStyle::Progress => vec![
                "[    ]".to_string(),
                "[=   ]".to_string(),
                "[==  ]".to_string(),
                "[=== ]".to_string(),
                "[====]".to_string(),
            ],
        };
        
        Self {
            frames,
            config,
        }
    }

    pub async fn update(&self, frame: usize) -> io::Result<()> {
        let frame_idx = frame % self.frames.len();
        let current_frame = &self.frames[frame_idx];
        
        print!("\r\x1b[36m{} {}\x1b[0m", current_frame, self.config.message);
        io::stdout().flush()?;
        sleep(self.config.delay).await;
        Ok(())
    }

    pub async fn run(&self) -> io::Result<()> {
        for i in 0..50 {  // Run for 50 frames by default
            self.update(i).await?;
        }
        println!("\r\x1b[K");
        Ok(())
    }
}

pub async fn update_thinking_animation(frame: usize) {
    let config = AnimationConfig {
        style: AnimationStyle::Spinner,
        message: "Thinking".to_string(),
        delay: Duration::from_millis(100),
    };
    
    let animation = ThinkingAnimation::new(config);
    let _ = animation.update(frame).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_animation_styles() {
        let styles = vec![
            AnimationStyle::Spinner,
            AnimationStyle::Progress,
        ];

        for style in styles {
            let config = AnimationConfig {
                style,
                ..Default::default()
            };
            let animation = ThinkingAnimation::new(config);
            assert!(!animation.frames.is_empty());
        }
    }
}
