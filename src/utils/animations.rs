// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationStyle {
    Classic,
    Braille,
    Matrix,
    Neural,
    Binary,
    Quantum,
    Circuit,
    DNA,
    Custom(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorStyle {
    White,
    Cyan,
    CyanGradient,
    None,
}

pub struct AnimationConfig {
    style: AnimationStyle,
    color: ColorStyle,
    message: String,
    delay: Duration,
    width: usize,
    frame_count: usize,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            style: AnimationStyle::Classic,
            color: ColorStyle::Cyan,
            message: "Processing".to_string(),
            delay: Duration::from_millis(100),
            width: 20,
            frame_count: 50,
        }
    }
}

pub struct ThinkingAnimation {
    frames: Vec<String>,
    config: AnimationConfig,
    color_map: HashMap<usize, String>,
}

impl ThinkingAnimation {
    pub fn new(config: AnimationConfig) -> Self {
        let frames = Self::generate_frames(&config.style, config.width);
        let color_map = Self::generate_color_map(&config.color, frames.len());
        
        Self {
            frames,
            config,
            color_map,
        }
    }

    fn generate_frames(style: &AnimationStyle, width: usize) -> Vec<String> {
        match style {
            AnimationStyle::Classic => vec![
                "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
            ].into_iter().map(String::from).collect(),

            AnimationStyle::Braille => {
                let braille = "⠁⠂⠄⠆⠈⠐⠠⡀⢀⣀⣁⣂⣄⣆⣈⣐⣠⣰⣲⣴⣶⣾⣿";
                braille.chars().map(|c| c.to_string()).collect()
            },

            AnimationStyle::Matrix => {
                let mut frames = Vec::new();
                for _ in 0..10 {
                    let frame: String = (0..width)
                        .map(|_| if thread_rng().gen_bool(0.5) { "1" } else { "0" })
                        .collect();
                    frames.push(frame);
                }
                frames
            },

            AnimationStyle::Neural => {
                let mut frames = Vec::new();
                let neurons = ["○", "◎", "●", "◉"];
                let connections = ["-", "=", "≡", "≣"];
                
                for i in 0..8 {
                    let mut frame = String::new();
                    for j in 0..width {
                        if j % 2 == 0 {
                            frame.push_str(neurons[j % neurons.len()]);
                        } else {
                            frame.push_str(connections[i % connections.len()]);
                        }
                    }
                    frames.push(frame);
                }
                frames
            },

            AnimationStyle::Binary => {
                let mut frames = Vec::new();
                for _ in 0..8 {
                    let frame: String = (0..width)
                        .map(|_| if thread_rng().gen_bool(0.5) { "1" } else { "0" })
                        .collect();
                    frames.push(frame);
                }
                frames
            },

            AnimationStyle::Quantum => {
                let quantum_symbols = ["⟩", "⟨", "⟷", "⟶", "⟵", "⟺", "⟹", "⟸", "⟿", "⟾"];
                quantum_symbols.iter()
                    .map(|&s| {
                        let mut frame = String::new();
                        for _ in 0..width/2 {
                            frame.push_str(s);
                        }
                        frame
                    })
                    .collect()
            },

            AnimationStyle::Circuit => {
                let circuit_symbols = ["┌", "┐", "└", "┘", "├", "┤", "┬", "┴", "─", "│"];
                let mut frames = Vec::new();
                for _ in 0..10 {
                    let frame: String = (0..width)
                        .map(|_| circuit_symbols[thread_rng().gen_range(0..circuit_symbols.len())])
                        .collect();
                    frames.push(frame);
                }
                frames
            },

            AnimationStyle::DNA => {
                let dna_pairs = ["AT", "TA", "GC", "CG"];
                let mut frames = Vec::new();
                for _ in 0..8 {
                    let frame: String = (0..width/2)
                        .map(|_| dna_pairs[thread_rng().gen_range(0..dna_pairs.len())])
                        .collect();
                    frames.push(frame);
                }
                frames
            },

            AnimationStyle::Custom(custom_frames) => custom_frames.clone(),
        }
    }

    fn generate_color_map(color_style: &ColorStyle, frame_count: usize) -> HashMap<usize, String> {
        let mut map = HashMap::new();
        match color_style {
            ColorStyle::White => {
                let color = "\x1b[37m".to_string(); // White
                for i in 0..frame_count {
                    map.insert(i, color.clone());
                }
            },
            ColorStyle::Cyan => {
                let color = "\x1b[36m".to_string(); // Cyan
                for i in 0..frame_count {
                    map.insert(i, color.clone());
                }
            },
            ColorStyle::CyanGradient => {
                // Cyan gradient colors from dark to light
                let cyan_colors = [
                    "\x1b[38;5;23m",  // Dark cyan
                    "\x1b[38;5;30m",  // Darker cyan
                    "\x1b[38;5;37m",  // Medium cyan
                    "\x1b[38;5;44m",  // Light cyan
                    "\x1b[38;5;51m",  // Bright cyan
                ];
                for i in 0..frame_count {
                    let color_idx = (i * cyan_colors.len()) / frame_count;
                    map.insert(i, cyan_colors[color_idx].to_string());
                }
            },
            ColorStyle::None => {
                for i in 0..frame_count {
                    map.insert(i, String::new());
                }
            },
        }
        map
    }

    pub async fn update(&self, frame: usize) -> io::Result<()> {
        let frame_idx = frame % self.frames.len();
        let current_frame = &self.frames[frame_idx];
        let color = self.color_map.get(&frame_idx).cloned().unwrap_or_default();
        
        print!("\r");
        print!("{}{} {}\x1b[0m", color, current_frame, self.config.message);
        io::stdout().flush()?;
        sleep(self.config.delay).await;
        Ok(())
    }

    pub async fn run(&self) -> io::Result<()> {
        for i in 0..self.config.frame_count {
            self.update(i).await?;
        }
        println!("\r\x1b[K");
        Ok(())
    }
}

pub async fn update_thinking_animation(frame: usize) {
    let config = AnimationConfig {
        style: AnimationStyle::Classic,
        color: ColorStyle::Cyan,
        message: "Thinking".to_string(),
        delay: Duration::from_millis(100),
        width: 20,
        frame_count: 50,
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
            AnimationStyle::Classic,
            AnimationStyle::Braille,
            AnimationStyle::Matrix,
            AnimationStyle::Neural,
            AnimationStyle::Binary,
            AnimationStyle::Quantum,
            AnimationStyle::Circuit,
            AnimationStyle::DNA,
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

    #[tokio::test]
    async fn test_color_styles() {
        let colors = vec![
            ColorStyle::White,
            ColorStyle::Cyan,
            ColorStyle::CyanGradient,
            ColorStyle::None,
        ];

        for color in colors {
            let config = AnimationConfig {
                color,
                ..Default::default()
            };
            let animation = ThinkingAnimation::new(config);
            assert!(!animation.color_map.is_empty());
        }
    }
}
