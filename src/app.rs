use ggez::{
    event::EventHandler,
    graphics::{self, DrawMode, DrawParam, Mesh, Rect, TextLayout},
    Context,
};

use crate::{
    color,
    grid::{Grid, GridTile},
    TEXT_SIZE, TILE_SIZE,
};

pub struct App {
    grid: Grid,
}

impl App {
    pub fn new(_ctx: &mut Context) -> Self {
        Self { grid: Grid::new() }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, color::BACKGROUND);

        // Scale to tile size
        let param = DrawParam::default().scale([TILE_SIZE, TILE_SIZE]);

        for GridTile { x, y, tile } in self.grid.iter() {
            let rect = Rect::new(x as f32, y as f32, 1.0, 1.0);

            if let Some(tile) = tile {
                // Draw rectangle
                let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color::TILE_FILL)?;
                canvas.draw(&mesh, param);
                let mesh =
                    Mesh::new_rectangle(ctx, DrawMode::stroke(0.1), rect, color::TILE_STROKE)?;
                canvas.draw(&mesh, param);

                let mut text = graphics::Text::new(tile.to_string());
                text.set_scale(TILE_SIZE * TEXT_SIZE);
                text.set_layout(TextLayout::center());
                // Set position to center of rectangle
                // This must use fresh DrawParam, to not blur the text
                let text_param = DrawParam::default()
                    .dest([(x as f32 + 0.5) * TILE_SIZE, (y as f32 + 0.5) * TILE_SIZE])
                    .color(color::TILE_TEXT);
                canvas.draw(&text, text_param);
            }
        }

        canvas.finish(ctx)
    }
}
