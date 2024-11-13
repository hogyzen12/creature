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
