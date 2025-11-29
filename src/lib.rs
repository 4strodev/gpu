use env_logger::{Builder, Target};
use log::LevelFilter;
use winit::event_loop::EventLoop;

use crate::app::App;

mod app;
mod state;

pub fn run() -> anyhow::Result<()> {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
