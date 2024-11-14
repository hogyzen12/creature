// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

const THINKING_FRAMES: [&str; 6] = [
    "⠋ Thinking...",
    "⠙ Thinking...",
    "⠹ Thinking...",
    "⠸ Thinking...",
    "⠼ Thinking...",
    "⠴ Thinking..."
];

pub async fn update_thinking_animation(frame: usize) {
    print!("[38;5;39m{}[0m", THINKING_FRAMES[frame % THINKING_FRAMES.len()]);
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(100)).await;
}
