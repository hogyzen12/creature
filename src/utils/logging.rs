// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

use chrono::Local;
use colored::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Write;

static STATS_LINE: AtomicUsize = AtomicUsize::new(0);

pub fn update_stats_line(stats: &str, mission: Option<&str>) {
    let current_line = STATS_LINE.load(Ordering::Relaxed);
    
    // Clear and update stats line
    print!("7"); // Save cursor
    print!("[{};0H", current_line); // Move to stats line
    print!("[2K"); // Clear line
    print!("[0;94mCREATURE: {}[0m", stats);
    
    // Clear and update mission line
    if let Some(mission) = mission {
        print!("[{};0H", current_line + 1); // Move to mission line
        print!("[2K"); // Clear line
        print!("[0;94mMISSION: {}[0m", mission);
    }
    
    print!("8"); // Restore cursor
    let _ = std::io::stdout().flush(); // Ensure output is flushed
}

pub fn print_banner(mission: &str, name: &str) -> usize {
    println!("{}", format!(r#"
    ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
 ██████╗██████╗ ███████╗ █████╗ ████████╗██╗   ██╗██████╗ ███████╗▀▀
██╔════╝██╔══██╗██╔════╝██╔══██╗╚══██╔══╝██║   ██║██╔══██╗██╔════╝░░
██║  ▄▄███████╗█████╗  ███████║   ██║   ██║   ██║██████╔╝█████╗  ▄▄
██║ ░░░██╔══██╗██╔══╝  ██╔══██║   ██║   ██║   ██║██╔══██╗██╔══╝  ░░
╚██████╗██║  ██║███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║███████╗▄▄
 ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝░░
                                                     by BasedAI
     ┌──────────────────┐  ╱|、        [2024-ALPHA-BUILD]        
     │  ◉_◉  W A K E   │ (˚ˎ。7       <based_ascension>
     │  U P  N E O . . │  |、˜〵      /system.mind.based/
     └──────────────────┘   じしˍ,)ノ  [based::maximized]

    MISSION: {}
    NAME: {}"#, mission, name).cyan());

    let line_num = r#"
 ██████╗██████╗ ███████╗ █████╗ ████████╗██╗   ██╗██████╗ ███████╗
██╔════╝██╔══██╗██╔════╝██╔══██╗╚══██╔══╝██║   ██║██╔══██╗██╔════╝
██║     ██████╔╝█████╗  ███████║   ██║   ██║   ██║██████╔╝█████╗  
██║     ██╔══██╗██╔══╝  ██╔══██║   ██║   ██║   ██║██╔══██╗██╔══╝  
╚██████╗██║  ██║███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║███████╗
 ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝
                                                     by BasedAI
"#.lines().count() + 2;
    STATS_LINE.store(line_num, Ordering::Relaxed);
    
    println!("
");
    line_num
}

pub fn log_header(title: &str) {
    println!("
╔═══════════════[>SYSTEM<]════════════════╗");
    println!("║ [CREATURE] {} ║", title.bright_cyan().bold());
    println!("╠══════════════[>NEURAL.LINK.ACTIVE<]═══════════════╣");
}

pub fn log_section(title: &str) {
    println!("║ [//{:0>4x}] {} ║", rand::random::<u16>(), title.cyan());
    println!("╠═══════════════[>STREAM<]════════════════╣");
}

pub fn log_metric(label: &str, value: impl std::fmt::Display) {
    println!("║ <{:_<24}> │ {:>35} ║", 
        label.bright_cyan(),
        format!("[{}]", value.to_string()).bright_white()
    );
}

pub fn log_detail(message: &str) {
    println!("| {}", message.white());
}

pub fn log_success(message: &str) {
    println!("║ [//:CRTR] >> {}", message.bright_cyan());
}

pub fn log_warning(message: &str) {
    println!("║ [WARN//DETECTED] >> {}", message.bright_yellow());
}

pub fn log_error(message: &str) {
    println!("║ [ERR//CRITICAL] >> {}", message.bright_red());
}

pub fn log_info(message: &str) {
    println!("║ [INFO//STREAM] >> {}", message.cyan());
}

pub fn log_timestamp(prefix: &str) {
    println!("║ [T:{}] <LINK> {}", 
        Local::now().format("%H:%M:%S.%3f").to_string().bright_cyan(),
        prefix
    );
}

pub fn log_footer() {
    println!("╚════════════════[>STREAM.TERMINATED<]═══════════════╝
");
}

pub fn log_memory_usage(label: &str, bytes: usize) {
    let (size, unit) = if bytes >= 1_000_000 {
        (bytes as f64 / 1_000_000.0, "MB")
    } else if bytes >= 1_000 {
        (bytes as f64 / 1_000.0, "KB")
    } else {
        (bytes as f64, "B")
    };
    
    log_metric(label, format!("{:.2} {}", size, unit));
}
pub fn log_section_header(title: &str) {
    println!("╔═══════════════[>CELL.LINK<]══════════════╗");
    println!("║ {:_<73} ║", format!("[{}]", title).bright_white().bold());
    println!("╠════════════════[>CELL.ACTIVE<]════════════════╣");
}

pub fn log_section_footer() {
    println!("╚═══════════════[>CELL.DISCONNECT<]══════════════╝
");
}

pub fn ensure_data_directories() -> std::io::Result<()> {
    let paths = ["data/thoughts", "data/plans"];
    for path in paths {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn log_thought_to_file(cell_id: &uuid::Uuid, thought: &crate::models::types::Thought) -> std::io::Result<()> {
    let thought_path = format!("data/thoughts/{}.json", cell_id);
    let mut thoughts: Vec<crate::models::types::Thought> = if std::path::Path::new(&thought_path).exists() {
        let content = std::fs::read_to_string(&thought_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };
    
    thoughts.push(thought.clone());
    std::fs::write(thought_path, serde_json::to_string_pretty(&thoughts)?)?;
    Ok(())
}

pub fn log_dimensional_metric(label: &str, value: f64, percentage: f64) {
    println!("|   - {:<20} {:<8.2} [{:>3.0}%]                               |",
        label.bright_white(),
        value.to_string().yellow(),
        percentage
    );
}

pub fn log_simple_metric(label: &str, value: impl std::fmt::Display) {
    println!("|   {:<25} {:<45} |",
        label.bright_white(),
        value.to_string().yellow()
    );
}
