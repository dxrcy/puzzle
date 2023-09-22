#[macro_use]
mod color_macro;

mod app;
mod grid;

pub use app::App;

pub const TILE_SIZE: f32 = 140.0;
pub const GRID_SIZE: usize = 4;
pub const TEXT_SIZE: f32 = 0.7;

mod color {
    colors!(
        BACKGROUND  (BLACK)
        TILE_FILL   (WHITE)
        TILE_STROKE (BLACK)
        TILE_TEXT   (BLUE)
    );
}
