use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

/// Simple 1d buffer of cells
pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Option<(char, Color)>>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            cells: vec![None; width.max(1) * height.max(1)],
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill_with(|| None);
    }

    /// Sets a single cell to `char` and `color`. No-op if out of bounds.
    pub fn write_char(&mut self, x: usize, y: usize, ch: char, color: Color) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = Some((ch, color));
        }
    }

    /// Writes `s` starting at (`x`, `y`), one column per character.
    /// Characters that land out of bounds are dropped.
    pub fn write(&mut self, x: usize, y: usize, s: &str, color: Color) {
        for (i, ch) in s.chars().enumerate() {
            self.write_char(x + i, y, ch, color);
        }
    }

    pub fn copy_from(&mut self, other: &Buffer) {
        self.cells.copy_from_slice(&other.cells);
    }

    /// Renders the buffer in one pass, overwriting every cell in the terminal
    pub fn display(&self, out: &mut Stdout) -> std::io::Result<()> {
        let flush_run = |out: &mut Stdout, run: &mut String| -> std::io::Result<()> {
            if !run.is_empty() {
                queue!(out, Print(&run))?;
                run.clear();
            }
            Ok(())
        };

        for y in 0..self.height {
            queue!(out, MoveTo(0, y as u16))?;
            let mut current_color: Option<Color> = None;
            let mut run = String::new();

            for x in 0..self.width {
                match self.cells[y * self.width + x] {
                    Some((ch, color)) => {
                        if current_color != Some(color) {
                            flush_run(out, &mut run)?;
                            queue!(out, SetForegroundColor(color))?;
                            current_color = Some(color);
                        }
                        run.push(ch);
                    }
                    None => {
                        if current_color.is_some() {
                            flush_run(out, &mut run)?;
                            queue!(out, ResetColor)?;
                            current_color = None;
                        }
                        run.push(' ');
                    }
                }
            }
            flush_run(out, &mut run)?;
        }
        queue!(out, ResetColor)?;
        out.flush()
    }
}
