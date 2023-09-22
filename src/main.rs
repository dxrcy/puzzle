use ggez::conf::WindowMode;
use ggez::event;
use ggez::ContextBuilder;
use ggez::GameResult;

use puzzle::App;
use puzzle::GRID_SIZE;
use puzzle::TILE_SIZE;

fn main() -> GameResult {
    let window_mode = WindowMode::default()
        .dimensions(TILE_SIZE * GRID_SIZE as f32, TILE_SIZE * GRID_SIZE as f32)
        .borderless(true);

    // Create app context
    let (mut ctx, event_loop) = ContextBuilder::new("15puzzle", "darcy")
        .window_mode(window_mode)
        .build()?;

    // Change window properties
    ctx.gfx.set_window_title("15 Puzzle");

    // Create app state
    let app = App::new(&mut ctx);

    // Run game loop
    event::run(ctx, event_loop, app);
}
