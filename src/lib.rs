#[macro_use]
mod color_macro;

mod app;
mod grid;

pub use app::App;

pub const TILE_SIZE: f32 = 140.0;
pub const GRID_SIZE: usize = 4;
pub const TEXT_SIZE: f32 = 0.7;
pub const PADDING: f32 = 0.05;
pub const MARGIN: f32 = 0.05;
pub const SIZE: f32 = TILE_SIZE * (GRID_SIZE as f32 - MARGIN * 2.0) / GRID_SIZE as f32;

mod color {
    colors!(
        BACKGROUND  (0x501616)
        TILE        (0xffefe4)
        TILE_ACTIVE (0xe5d0c2)
        TEXT        (0xb69587)
        EMPTY_CELL  (0x471414)
    );
}
