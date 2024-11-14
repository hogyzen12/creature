mod api;
mod models;
mod systems;
mod server;
mod utils;

use serde_json::Value;
use std::sync::{Arc, Mutex};
use clap::{App, Arg};
use crate::models::types::Coordinates;
use crate::models::constants::{BATCH_SIZE, CELL_INIT_DELAY_MS, CYCLE_DELAY_MS};
use crate::systems::colony::Colony;
use rand::Rng;
use std::time::Duration;
use tokio::time;
use tokio::signal::ctrl_c;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc as StdArc;
use futures::future;
use serde_json::json;
use tokio::sync::mpsc::{self, Sender};


const DEFAULT_INITIAL_CELLS: usize = 32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    crate::utils::logging::ensure_data_directories()
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;

    let running = StdArc::new(AtomicBool::new(true));
    let r = running.clone();
    let r2 = running.clone();

    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_tx_clone = shutdown_tx.clone();

    // Handle both Ctrl+C and SIGTERM
    tokio::spawn(async move {
        let mut sigterm = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate()
        ).expect("Failed to set up SIGTERM handler");
        
        tokio::select! {
            _ = ctrl_c() => {
                println!("\nReceived Ctrl+C signal. Sending CREATURE to pasture...");
                for _ in 0..2 {
                    shutdown_tx_clone.send(()).unwrap();
                }
                println!("\nIf the process doesn't exit cleanly, you can force quit with:");
                println!("sudo kill -9 $(pgrep -fl 'creature' | awk '{{print $1}}')");
            }
            _ = sigterm.recv() => {
                println!("\nReceived SIGTERM signal. Sending CREATURE to pasture...");
                for _ in 0..2 {
                    if let Err(e) = shutdown_tx_clone.send(()) {
                        eprintln!("Failed to send shutdown signal: {}", e);
                    }
                }
            }
        }
        r.store(false, Ordering::SeqCst);
    });

    tokio::spawn(async move {
        let mut force_exit = false;
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                if !r2.load(Ordering::SeqCst) {
                    println!("Forcing exit after timeout...");
                    force_exit = true;
                }
            }
        }
        if force_exit {
            std::process::exit(0);
        }
    });

    let api_key = std::env::var("OPENROUTER_API_KEY").map_err(|_| {
        let error_msg = "
╔════════════════════════════════════════════════════════════════╗
║                         ERROR                                   ║
║ OPENROUTER_API_KEY environment variable is not set             ║
║                                                                ║
║ Please set it by running:                                      ║
║ export OPENROUTER_API_KEY='your-api-key'                       ║
║                                                                ║
║ You can get an API key from:                                   ║
║ https://openrouter.ai/keys                                     ║
╚════════════════════════════════════════════════════════════════╝
";
        eprintln!("{}", error_msg);
        std::io::Error::new(std::io::ErrorKind::NotFound, "OPENROUTER_API_KEY not set")
    })?;

    // Set up command line argument parsing
    let matches = App::new("Creature")
        .version("0.1.0")
        .author("BasedAI")
        .about("Adaptive AI Colony Simulation")
        .arg(Arg::with_name("mission")
            .short('m')
            .long("mission")
            .value_name("MISSION")
            .help("Sets the colony's mission")
            .takes_value(true))
        .arg(Arg::with_name("name")
            .short('n')
            .long("name")
            .value_name("NAME")
            .help("Sets the colony's name")
            .takes_value(true))
        .arg(Arg::with_name("state")
            .short('s')
            .long("state")
            .value_name("STATE_FILE")
            .help("Load initial state from file")
            .takes_value(true))
        .arg(Arg::with_name("cells")
            .short('c')
            .long("cells")
            .value_name("COUNT")
            .help("Sets the initial number of cells (default: 32)")
            .takes_value(true))
        .get_matches();

    let initial_cells = matches.value_of("cells")
        .and_then(|c| c.parse().ok())
        .unwrap_or(DEFAULT_INITIAL_CELLS);

    let mission = matches.value_of("mission")
        .unwrap_or("Develop innovative AI collaboration systems with focus on real-time adaptation")
        .to_string();
        
    let colony_name = matches.value_of("name").unwrap_or("Unnamed");
    
    crate::utils::logging::update_stats_line(&format!("{}", colony_name));

    let api_client = api::openrouter::OpenRouterClient::new(api_key.clone())
        .map_err(|e| e as Box<dyn std::error::Error>)?;
    let mut colony = Colony::new(mission, api_client);
    
    // Try loading state from command line arg or default file
    let state_file = matches.value_of("state").unwrap_or("eca_state.json");
    if std::path::Path::new(state_file).exists() {
        match colony.load_state_from_file(state_file) {
            Ok(_) => println!("Loaded colony state from {}", state_file),
            Err(e) => eprintln!("Error loading state from {}: {}", state_file, e)
        }
    } else {
        if let Err(e) = colony.save_state_to_file("eca_state.json") {
            eprintln!("Error creating initial state file: {}", e);
        }
    }
    
    let colony = Arc::new(Mutex::new(colony));
    let colony_ws = Arc::clone(&colony);

    let shutdown_rx_ws = shutdown_tx.subscribe();
    tokio::spawn(async move {
        server::start_server(colony_ws, shutdown_rx_ws).await;
    });

    let simulation_cycles = 100000000; // forever and forever
    let mut current_cycle = 0;

    println!("Initializing colony...");

    let (init_tx, mut init_rx) = mpsc::channel::<Value>(100);
    let init_tx = Arc::new(init_tx);

    // Spawn a separate task to handle initialization events
    let init_task = tokio::spawn(async move {
        while let Some(event) = init_rx.recv().await {
            println!("{}", event["message"].as_str().unwrap_or(""));
        }
    });

    // Create a vector to store initialization futures
    let mut init_futures = Vec::new();

    // Spawn cell initialization tasks
    for cell_index in 0..initial_cells {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let colony = Arc::clone(&colony);
        let init_tx: Arc<Sender<Value>> = Arc::clone(&init_tx);

        let future = tokio::spawn({
            let colony = colony.clone();
            let init_tx = init_tx.clone();
            async move {
                // Generate position coordinates before getting the lock
                let grid_pos = (
                    (cell_index as f64 / 9.0).floor(),
                    ((cell_index % 9) as f64 / 3.0).floor(),
                    (cell_index % 3) as f64
                );
                
                // Generate random offsets before getting the lock
                let x_offset = rand::thread_rng().gen_range(-0.2..0.2);
                let y_offset = rand::thread_rng().gen_range(-0.2..0.2);
                let z_offset = rand::thread_rng().gen_range(-0.2..0.2);
                
                let position = Coordinates {
                    x: grid_pos.2 * 2.0 + x_offset,
                    y: grid_pos.1 * 2.0 + y_offset,
                    z: grid_pos.0 * 2.0 + z_offset,
                    heat: 0.0,
                    emergence_score: 0.0,
                    coherence_score: 0.0,
                    resilience_score: 0.0,
                    intelligence_score: 0.0,
                    efficiency_score: 0.0,
                    integration_score: 0.0,
                };
                
                // Create cell and get its info while holding the lock
                let (cell_id, cell) = {
                    let mut colony_guard = colony.lock().unwrap();
                    let id = colony_guard.add_cell(position.clone());
                    let cell = colony_guard.cells.get(&id).unwrap().clone();
                    (id, cell)
                };

                // Create and send the event after releasing the lock
                let init_event = json!({
                    "type": "initialization",
                    "message": format!(
                        "Initialized cell {} of {}:\n  Ca position: ({:.1}, {:.1}, {:.1})\n  Gradient position: ({:.2}, {:.2}, {:.2})\n  Cell ID: {}\n  Initial energy: {:.1}\n  Heat level: {:.2}\n  Dimensional Scores:\n    Emergence: {:.1}\n    Coherence: {:.1}\n    Resilience: {:.1}\n    Intelligence: {:.1}\n    Efficiency: {:.1}\n    Integration: {:.1}\n",
                        cell_index + 1, INITIAL_CELLS,
                        grid_pos.0, grid_pos.1, grid_pos.2,
                        position.x, position.y, position.z,
                        cell_id,
                        cell.energy,
                        cell.position.heat,
                        cell.position.emergence_score,
                        cell.position.coherence_score,
                        cell.position.resilience_score,
                        cell.position.intelligence_score,
                        cell.position.efficiency_score,
                        cell.position.integration_score
                    )
                });

                if let Err(e) = init_tx.send(init_event).await {
                    eprintln!("Error sending initialization event: {}", e);
                }

                time::sleep(Duration::from_millis(CELL_INIT_DELAY_MS)).await;
                cell_id
            }
        });

        init_futures.push(future);
    }

    // Wait for all initialization tasks to complete with a timeout
    let timeout_duration = Duration::from_secs(INITIAL_CELLS as u64 * (CELL_INIT_DELAY_MS / 1000 + 1));
    match time::timeout(timeout_duration, future::join_all(init_futures)).await {
        Ok(results) => {
            for result in results {
                if let Err(e) = result {
                    eprintln!("Cell initialization error: {}", e);
                }
            }
        }
        Err(_) => {
            eprintln!("Colony initialization timed out!");
            return Err("Initialization timeout".into());
        }
    }

    // Drop the initialization channel
    drop(init_tx);
    
    // Wait for the initialization event handler to complete
    if let Err(e) = init_task.await {
        eprintln!("Error in initialization event handler: {}", e);
    }

    println!("Cellular initialization complete!");

    crate::utils::logging::print_banner();

    'main: while current_cycle < simulation_cycles && running.load(Ordering::SeqCst) {
        let stats = {
            let colony_guard = colony.lock().unwrap();
            (
                colony_guard.cells.len(),
                colony_guard.get_average_energy(),
                colony_guard.get_total_thoughts(),
                colony_guard.get_total_plans(),
                colony_guard.get_mutation_rate(),
                colony_guard.get_cluster_count()
            )
        }; 

        crate::utils::logging::update_stats_line(&format!("{} | Cells: {} | Energy: {:.1} | Thoughts: {} | Plans: {} | Mutation: {:.1}%",
            colony_name, stats.0, stats.1, stats.2, stats.3, stats.4 * 100.0));
            
        println!("Active cells: {}", stats.0);
        println!("Average energy: {:.1}", stats.1);
        println!("Total thoughts: {}", stats.2);
        println!("Total plans: {}", stats.3);
        println!("Mutation rate: {:.1}%", stats.4 * 100.0);
        println!("Cluster count: {}", stats.5);
        
        let cell_ids: Vec<uuid::Uuid>;
        {
            let colony_guard = colony.lock().unwrap();
            cell_ids = colony_guard.cells.keys().copied().collect();
        }

        for batch_idx in (0..cell_ids.len()).step_by(BATCH_SIZE) {
            if !running.load(Ordering::SeqCst) {
                println!("Shutting down simulation...");
                break 'main;
            }
            println!("\nStarting thoughts batch {} of {}", 
                batch_idx / BATCH_SIZE + 1, (cell_ids.len() + BATCH_SIZE - 1) / BATCH_SIZE);
            let batch_end = (batch_idx + BATCH_SIZE).min(cell_ids.len());
            let batch = cell_ids[batch_idx..batch_end].to_vec();
            println!("Preparing to process {} cells...", batch.len());
            if let Err(e) = colony.lock().unwrap().process_cell_batch(&batch).await {
                eprintln!("Error processing cell batch: {}", e);
            }
        }
        
        for batch_idx in (0..cell_ids.len()).step_by(BATCH_SIZE) {
            println!("\nCreating plans batch {} of {}", 
                batch_idx / BATCH_SIZE + 1, (cell_ids.len() + BATCH_SIZE - 1) / BATCH_SIZE);
            let batch_end = (batch_idx + BATCH_SIZE).min(cell_ids.len());
            let batch = cell_ids[batch_idx..batch_end].to_vec();
            if let Err(e) = colony.lock().unwrap().create_plans_batch(&batch, &current_cycle.to_string()).await {
                eprintln!("Error creating plans batch: {}", e);
            }
        }
        
        // Evolution phase
        if let Err(e) = colony.lock().unwrap().evolve_cells().await {
            eprintln!("Error evolving cells: {}", e);
        }
        if let Err(e) = colony.lock().unwrap().handle_cell_reproduction().await {
            eprintln!("Error handling cell reproduction: {}", e);
        }
        if let Err(e) = colony.lock().unwrap().update_mission_progress().await {
            eprintln!("Error updating mission progress: {}", e);
        }
        
        // Memory compression (every other cycle)
        if current_cycle % 2 == 0 {
            if let Err(e) = colony.lock().unwrap().compress_colony_memories().await {
                eprintln!("Error compressing colony memories: {}", e);
            }
        }
        
        {
            let colony_guard = colony.lock().unwrap();
            colony_guard.print_cycle_statistics(current_cycle);
            if let Err(e) = colony_guard.save_state() {
                eprintln!("Error saving state: {}", e);
            }
        }
        current_cycle += 1;
        
        time::sleep(Duration::from_millis(CYCLE_DELAY_MS)).await;
        
        // Display thinking animation
        for frame in 0..12 {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            crate::utils::animations::update_thinking_animation(frame).await;
        }
        println!(); // Clear animation line
    }

    if !running.load(Ordering::SeqCst) {
        println!("Shutting down gracefully...");
    } else {
        println!("\nSimulation complete!");
    }
    
    colony.lock().unwrap().print_statistics();
    
    println!("Cleaning up resources...");
    drop(colony);
    let cleanup_timeout = Duration::from_secs(180);
    let cleanup_deadline = tokio::time::Instant::now() + cleanup_timeout;
    
    while tokio::time::Instant::now() < cleanup_deadline {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    println!("Shutdown complete");
    Ok(())
}
