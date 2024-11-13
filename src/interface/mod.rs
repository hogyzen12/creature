pub mod widgets;
use std::time::{Instant, Duration};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Line,
    text::Span,
    widgets::{Block, Borders, Paragraph, Clear},
    Terminal, Frame,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use uuid::Uuid;
use std::collections::HashMap;

pub use self::widgets::{CellDisplay, EnergyBar};

const NEON_CYAN: Color = Color::Rgb(0, 255, 255);
const DARK_CYAN: Color = Color::Rgb(0, 128, 128);
const SYSTEM_BLUE: Color = Color::Rgb(41, 171, 226);
const NEON_YELLOW: Color = Color::Rgb(255, 255, 0);
const STARK_BLUE: Color = Color::Rgb(0, 255, 255);
const STARK_ACCENT: Color = Color::Rgb(0, 200, 255);
const STARK_HIGHLIGHT: Color = Color::Rgb(255, 200, 0);
const ENERGY_COLOR: Color = Color::Rgb(0, 255, 200);
const WARNING_COLOR: Color = Color::Rgb(255, 100, 0);
const SUCCESS_COLOR: Color = Color::Rgb(0, 255, 150);

#[derive(Clone)]
pub struct SimulationStats {
    pub active_cells: usize,
    pub clusters: usize,
    pub generation: u64,
    pub grid_depth: usize,
    pub energy_level: f64,
    pub thought_count: usize,
    pub plan_count: usize,
    pub mutation_rate: f64,
    pub bandwidth_usage: f64,
    pub updates_per_second: f64,
    pub total_api_calls: usize,
    pub runtime_seconds: f64,
    pub current_batch: usize,
    pub total_batches: usize,
    pub processing_phase: ProcessingPhase,
    pub latest_thoughts: Vec<String>,
    pub performance_metrics: HashMap<String, f64>,
    pub start_time: Instant,
    pub last_update: Instant,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ProcessingPhase {
    ThoughtGeneration,
    PlanCreation,
    Evolution,
    MemoryCompression,
    Active,
}

#[derive(Clone)]
pub struct CellGrid {
    pub cells: Vec<Vec<bool>>,
    pub energy_levels: Vec<Vec<f64>>,
    pub grid_id: Uuid,
}

pub struct Interface {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    simulation_stats: SimulationStats,
    cell_grids: Vec<CellGrid>,
}

impl Interface {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Interface {
            terminal,
            simulation_stats: SimulationStats::default(),
            cell_grids: Vec::new(),
        })
    }

    pub fn should_quit(&self) -> Result<bool, io::Error> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                return Ok(matches!(
                    key,
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    } | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                ));
            }
        }
        Ok(false)
    }

    pub fn update_stats(&mut self, stats: SimulationStats) {
        self.simulation_stats = stats;
    }

    pub fn update_grids(&mut self, grids: Vec<CellGrid>) {
        self.cell_grids = grids;
    }


    pub fn draw(&mut self) -> Result<(), io::Error> {
        // Clone the data we need before the closure
        let stats = self.simulation_stats.clone();
        let grids = self.cell_grids.clone();
        
        self.terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .margin(1)
                .split(f.size());

            // Use static methods instead of self methods
            Self::draw_view_panel(f, "◢ TOP VIEW ◣", grids.get(0), &stats, main_layout[0]);
            Self::draw_view_panel(f, "◢ FRONT VIEW ◣", grids.get(1), &stats, main_layout[1]);
            Self::draw_view_panel(f, "◢ SIDE VIEW ◣", grids.get(2), &stats, main_layout[2]);
            Self::draw_system_logs(f, &stats.latest_thoughts, main_layout[3]);
            Self::draw_status_bar(f, &stats);
        })?;
        Ok(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Simulation complete"
        )))
    }

    fn draw_view_panel(
        f: &mut Frame,
        title: &str,
        grid: Option<&CellGrid>,
        stats: &SimulationStats,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NEON_CYAN))
            .title(Span::styled(
                title,
                Style::default()
                    .fg(NEON_CYAN)
                    .add_modifier(Modifier::BOLD)
            ));

        let inner_area = block.inner(area);
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        if let Some(grid) = grid {
            let cell_display = CellDisplay::new(
                grid.cells.clone(),
                grid.energy_levels.clone(),
                stats.processing_phase == ProcessingPhase::ThoughtGeneration
            );
            f.render_widget(cell_display, inner_area);
        }
    }

    fn draw_system_logs(f: &mut Frame, thoughts: &[String], area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NEON_CYAN))
            .title(Span::styled(
                "◢ SYSTEM LOGS ◣",
                Style::default()
                    .fg(NEON_CYAN)
                    .add_modifier(Modifier::BOLD)
            ));

        let log_messages: Vec<Line> = thoughts
            .iter()
            .map(|thought| {
                Line::from(vec![
                    Span::styled("►", Style::default().fg(NEON_YELLOW)),
                    Span::raw(" "),
                    Span::styled(thought, Style::default().fg(SYSTEM_BLUE))
                ])
            })
            .collect();

        let logs = Paragraph::new(log_messages)
            .block(block)
            .style(Style::default().fg(NEON_CYAN));

        f.render_widget(Clear, area);
        f.render_widget(logs, area);
    }

    fn draw_status_bar(f: &mut Frame, stats: &SimulationStats) {
        let status_text = vec![
            Line::from(vec![
                Span::styled(
                    format!("CELLS: {}", stats.active_cells),
                    Style::default().fg(NEON_CYAN)
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("GEN: {}", stats.generation),
                    Style::default().fg(SYSTEM_BLUE)
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("UPS: {:.1}", stats.updates_per_second),
                    Style::default().fg(NEON_YELLOW)
                ),
                Span::raw(" | "),
                Span::styled(
                    match stats.processing_phase {
                        ProcessingPhase::ThoughtGeneration => "◢ THINKING ◣",
                        ProcessingPhase::PlanCreation => "◢ PLANNING ◣",
                        ProcessingPhase::Evolution => "◢ EVOLVING ◣",
                        ProcessingPhase::MemoryCompression => "◢ COMPRESSING ◣",
                        ProcessingPhase::Active => "◢ ACTIVE ◣",
                    },
                    Style::default()
                        .fg(NEON_CYAN)
                        .add_modifier(Modifier::BOLD)
                ),
            ])
        ];

        let status_bar = Paragraph::new(status_text)
            .style(Style::default().bg(Color::Black));

        let area = Rect {
            x: 0,
            y: f.size().height - 1,
            width: f.size().width,
            height: 1,
        };

        f.render_widget(status_bar, area);
    }

    pub fn cleanup(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }
}

impl Default for SimulationStats {
    fn default() -> Self {
        SimulationStats {
            active_cells: 0,
            clusters: 0,
            generation: 0,
            grid_depth: 0,
            energy_level: 0.0,
            thought_count: 0,
            plan_count: 0,
            mutation_rate: 0.0,
            bandwidth_usage: 0.0,
            updates_per_second: 0.0,
            total_api_calls: 0,
            runtime_seconds: 0.0,
            current_batch: 0,
            total_batches: 0,
            processing_phase: ProcessingPhase::Active,
            latest_thoughts: Vec::new(),
            performance_metrics: HashMap::new(),
            start_time: Instant::now(),
            last_update: Instant::now(),
        }
    }
}
