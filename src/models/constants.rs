// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

pub const MAX_MEMORY_SIZE: usize = 50000;
pub const MAX_THOUGHTS_FOR_PLAN: usize = 42;
pub const NEIGHBOR_DISTANCE_THRESHOLD: f64 = 2.0;
pub const BATCH_SIZE: usize = 5;

// Timing constants
pub const CELL_INIT_DELAY_MS: u64 = 2;
pub const CYCLE_DELAY_MS: u64 = 10;
pub const API_TIMEOUT_SECS: u64 = 300;

// API constants
pub const MAX_TOKENS_GROK: usize = 120000;
pub const MAX_TOKENS_CLAUDE: usize = 8096;
pub const MAX_TOKENS_GEMINI: usize = 8096;
pub const MAX_PROMPT_TOKENS: usize = 6072; // Reserve 1024 for response
pub const TOKEN_PADDING: usize = 50; // Safety margin
