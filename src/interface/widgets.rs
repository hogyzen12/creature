// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, StatefulWidget, Widget},
};

pub struct CellDisplay {
    cells: Vec<Vec<bool>>,
    energy_levels: Vec<Vec<f64>>,
    processing: bool,
}

impl CellDisplay {
    pub fn new(cells: Vec<Vec<bool>>, energy_levels: Vec<Vec<f64>>, processing: bool) -> Self {
        CellDisplay {
            cells,
            energy_levels,
            processing,
        }
    }
}

impl Widget for CellDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cells = self.cells;
        let energy_levels = self.energy_levels;
        
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if y < (area.height - 2) as usize && x < (area.width - 2) as usize {
                    let is_active = cells[y][x];
                    let energy = energy_levels[y][x];
                    
                    let display_char = if is_active { 
                        if self.processing { '◆' } else { '■' }
                    } else { 
                        '·' 
                    };

                    // Enhanced color scheme
                    let color = if is_active {
                        match energy {
                            e if e > 0.8 => Color::Rgb(0, 255, 255),  // Bright cyan
                            e if e > 0.6 => Color::Rgb(0, 200, 255),  // Arc reactor blue
                            e if e > 0.4 => Color::Rgb(0, 150, 255),  // Medium blue
                            _ => Color::Rgb(0, 100, 255),             // Deep blue
                        }
                    } else {
                        Color::Rgb(30, 30, 30) // Darker background
                    };

                    let cell_style = if self.processing {
                        Style::default()
                            .fg(color)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(color)
                    };

                    buf.get_mut(
                        area.x + 1 + x as u16,
                        area.y + 1 + y as u16
                    )
                    .set_char(display_char)
                    .set_style(cell_style);
                }
            }
        }
    }
}

pub struct EnergyBar {
    progress: f64,
}

impl EnergyBar {
    pub fn new(progress: f64) -> Self {
        EnergyBar {
            progress: progress.clamp(0.0, 1.0),
        }
    }
}

impl Widget for EnergyBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width as usize;
        let filled = (self.progress * width as f64) as usize;

        for x in 0..width {
            let symbol = if x < filled { '█' } else { '░' };
            let color = if x < filled {
                match (x as f64 / width as f64) {
                    p if p > 0.8 => Color::Rgb(0, 255, 255),  // Bright cyan
                    p if p > 0.6 => Color::Rgb(0, 200, 200),  // Medium cyan
                    p if p > 0.4 => Color::Rgb(0, 150, 150),  // Dark cyan
                    _ => Color::Rgb(0, 100, 100),             // Very dark cyan
                }
            } else {
                Color::DarkGray
            };

            buf.get_mut(area.x + x as u16, area.y)
                .set_char(symbol)
                .set_style(Style::default().fg(color));
        }
    }
}
