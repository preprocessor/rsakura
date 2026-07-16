use std::fs;
use std::io::Stdout;
use std::path::PathBuf;

use crossterm::style::Color;

use crate::art;
use crate::buffer::Buffer;
use crate::cli::Cli;

const TAU: f64 = std::f64::consts::PI * 2.0;

#[derive(Clone, Copy, PartialEq)]
enum LeafState {
    Attached,
    Falling,
    Resting,
}

#[derive(Clone, Copy)]
struct Leaf {
    x: f64,
    y: f64,
    dx: f64,
    phase: f64,
    state: LeafState,
    ch: char,
}

pub struct Scene {
    width: usize,
    height: usize,
    dynamic: Buffer,
    tree: Buffer,
    leaves: Vec<Leaf>,
    ground_y: f64,
    args: Cli,
    art_path: Option<PathBuf>,
}

impl Scene {
    pub fn new(width: usize, height: usize, args: Cli) -> Self {
        let mut tree = Buffer::new(width, height);
        let mut leaves = Vec::new();
        let ground_y = load_tree_art(
            args.art_path.clone(),
            width,
            height,
            &mut tree,
            &mut leaves,
            args.art_color,
        );

        Scene {
            width,
            height,
            dynamic: Buffer::new(width, height),
            tree,
            leaves,
            ground_y,
            art_path: args.art_path.clone(),
            args: args.clone(),
        }
    }

    /// Makes leaves fall
    pub fn gust(&mut self) {
        for leaf in self.leaves.iter_mut() {
            if leaf.state == LeafState::Attached && rand::random_range(0.0..1.0) < 0.08 {
                leaf.state = LeafState::Falling;
                leaf.dx = rand::random_range(-0.35..0.35);
                leaf.phase = rand::random_range(0.0..TAU);
            }
        }
    }

    /// Advances leaf physics by one frame.
    pub fn tick(&mut self) {
        for leaf in self.leaves.iter_mut() {
            match leaf.state {
                LeafState::Falling => {
                    leaf.y += 0.1 * self.args.speed_factor;
                    leaf.phase += 0.08;
                    leaf.x += leaf.dx + leaf.phase.sin() * 0.2;
                    if leaf.y >= self.ground_y {
                        leaf.y = self.ground_y;
                        leaf.state = LeafState::Resting;
                    }
                }
                LeafState::Attached if self.args.sway_amplitude > 0.0 => leaf.phase += 0.02,
                _ => {}
            }
        }
    }

    /// Reallocates the buffers and rescales the tree art for a new terminal size.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.dynamic = Buffer::new(width, height);
        self.tree = Buffer::new(width, height);
        self.ground_y = load_tree_art(
            self.art_path.clone(),
            width,
            height,
            &mut self.tree,
            &mut self.leaves,
            self.args.art_color,
        );
    }

    /// Composes tree, leaves, and quit hint into the dynamic buffer and draws it to the terminal.
    pub fn render(&mut self, out: &mut Stdout) -> std::io::Result<()> {
        self.dynamic.copy_from(&self.tree);

        for leaf in &self.leaves {
            let mut draw_x = leaf.x;
            if leaf.state == LeafState::Attached && self.args.sway_amplitude > 0.0 {
                draw_x += leaf.phase.sin() * self.args.sway_amplitude;
            }
            let ix = draw_x.round() as usize;
            let iy = leaf.y.round() as usize;
            if ix < self.width && iy < self.height {
                self.dynamic
                    .write_char(ix, iy, leaf.ch, self.args.leaf_color);
            }
        }

        // self.dynamic.write(1, 1, "Q/Esc to quit", Color::White);
        self.dynamic.display(out)
    }
}

/// Scales the art to fit the terminal, draws it into `tree`, and scatters `leaves` across the canopy.
/// Returns the row leaves should rest on.
fn load_tree_art(
    path: Option<PathBuf>,
    width: usize,
    height: usize,
    tree: &mut Buffer,
    leaves: &mut Vec<Leaf>,
    art_color: Color,
) -> f64 {
    let default_ground = height as f64;

    let art_text = if let Some(text) = path.and_then(|p| fs::read_to_string(p).ok()) {
        text
    } else if !art::ART_DATA.is_empty() {
        art::ART_DATA.to_string()
    } else {
        return default_ground;
    };

    tree.clear();
    leaves.clear();

    // 2d vec of the artwork
    let lines: Vec<Vec<char>> = art_text.lines().map(|l| l.chars().collect()).collect();
    let art_width = lines.iter().map(|l| l.len()).max().unwrap_or(0) as f64;
    let art_height = lines.len() as f64;
    if art_width == 0. || art_height == 0. {
        return default_ground;
    }

    let target_width = width.max(1) as f64;
    let target_height = height.max(1) as f64;

    let step_x = (art_width / target_width).max(1.0);
    let step_y = (art_height / target_height).max(1.0);

    let scaled_width = (art_width / step_x).ceil() as usize;
    let scaled_height = (art_height / step_y).ceil() as usize;

    let offset_x = width.saturating_sub(scaled_width) / 2;
    let offset_y = height.saturating_sub(scaled_height + 1);

    let mut lowest_row = 0;

    for dy in 0..scaled_height {
        let sy = dy as f64 * step_y;
        let Some(line) = lines.get(sy as usize) else {
            continue;
        };

        for dx in 0..scaled_width {
            let sx = dx as f64 * step_x;
            let Some(&ch) = line.get(sx as usize) else {
                continue;
            };
            if ch == ' ' {
                continue;
            }

            let bx = offset_x + dx;
            let by = offset_y + dy;
            if bx >= width || by >= height {
                continue;
            }

            tree.write_char(bx, by, ch, art_color);
            if by > lowest_row {
                lowest_row = by;
            }
            if sy < art_height * 0.45 && rand::random_range(0.0..1.0) < 0.06 {
                leaves.push(Leaf {
                    x: bx as f64,
                    y: by as f64,
                    dx: 0.0,
                    phase: rand::random_range(0.0..TAU),
                    state: LeafState::Attached,
                    ch: '*',
                });
            }
        }
    }

    height.min(lowest_row) as f64
}
