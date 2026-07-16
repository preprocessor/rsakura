mod art;
mod buffer;
mod cli;
mod scene;

use std::io::stdout;
use std::time::{Duration, Instant};

use clap::Parser;
use cli::Cli;
use scene::Scene;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main_loop(out: &mut std::io::Stdout, mut scene: Scene) -> std::io::Result<()> {
    let mut last_gust = Instant::now()
        .checked_sub(Duration::from_secs(4))
        .unwrap_or_else(Instant::now);
    let mut gust_interval = Duration::from_secs_f64(rand::random_range(1.2..3.0));

    loop {
        // input / resize
        // TODO add controls to adjust speed + aplitude
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event)
                    if matches!(key_event.kind, KeyEventKind::Press | KeyEventKind::Repeat) =>
                {
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            break;
                        }
                        _ => {}
                    }
                }
                Event::Resize(width, height) => {
                    scene.resize(width as usize, height as usize);
                    execute!(out, Clear(ClearType::All))?;
                }
                _ => {}
            }
        }

        // wind gusts
        let now = Instant::now();
        if now.duration_since(last_gust) >= gust_interval {
            last_gust = now;
            gust_interval = Duration::from_secs_f64(rand::random_range(2.0..5.0));
            scene.gust();
        }
        // tick physics
        scene.tick();
        // render scene
        scene.render(out)?;
        // 60 fps
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let (width, height) = terminal::size()?;
    let (width, height) = (width as usize, height as usize);
    let scene = Scene::new(width, height, args);

    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    let result = main_loop(&mut out, scene);

    execute!(out, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}
