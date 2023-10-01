use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, DrawMode, DrawParam, Mesh, Rect, TextLayout},
    mint::Point2,
    Context, winit::event::VirtualKeyCode,
};

use crate::{
    color,
    grid::{Grid, GridTile},
    GRID_SIZE, MARGIN, PADDING, SIZE, TEXT_SIZE, TILE_SIZE,
};

pub struct App {
    grid: Grid,
    active_tile: Option<(usize, usize)>,
}

impl App {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            grid: Grid::new(),
            active_tile: None,
        }
    }

    pub fn reset(&mut self, ctx: &mut Context) {
        *self = Self::new(ctx)
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let Point2 { x, y } = ctx.mouse.position();

        let (x, y) = (x as f32 / TILE_SIZE, y as f32 / TILE_SIZE);

        let range = 0.0..GRID_SIZE as f32;
        self.active_tile = if range.contains(&x) && range.contains(&y) {
            Some((x as usize, y as usize))
        } else {
            None
        };

        if ctx.mouse.button_just_pressed(MouseButton::Left) {
            if let Some((x, y)) = self.active_tile {
                self.grid.shift_tiles(x, y);
            }
        }

        if self.grid.is_complete() {
            println!("Well done!");
            std::process::exit(0)
        }

        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::R) {
            self.reset(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, color::BACKGROUND);

        for GridTile { x, y, tile } in self.grid.iter() {
            let rect = Rect::new(
                (x as f32 + PADDING + MARGIN) * SIZE,
                (y as f32 + PADDING + MARGIN) * SIZE,
                (1.0 - PADDING * 2.0) * SIZE,
                (1.0 - PADDING * 2.0) * SIZE,
            );

            if let Some(tile) = tile {
                let tile_color = if self.active_tile == Some((x, y)) {
                    color::TILE_ACTIVE
                } else {
                    color::TILE
                };

                // Draw rectangle
                let mesh = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    0.1 * SIZE,
                    tile_color,
                )?;
                canvas.draw(&mesh, DrawParam::default());

                let mut text = graphics::Text::new(tile.to_string());
                text.set_scale(SIZE * TEXT_SIZE);
                text.set_layout(TextLayout::center());
                // Set position to center of rectangle
                // This must use fresh DrawParam, to not blur the text
                let text_param = DrawParam::default()
                    .dest([
                        (x as f32 + 0.5 + MARGIN) * SIZE,
                        (y as f32 + 0.53 + MARGIN) * SIZE,
                    ])
                    .color(color::TEXT);
                canvas.draw(&text, text_param);
            }
        }

        if let Some((x, y)) = self.active_tile {
            if let Some((new_x, new_y)) = self.grid.find_empty(x, y) {
                let rect = Rect::new(
                    (new_x as f32 + PADDING + MARGIN) * SIZE,
                    (new_y as f32 + PADDING + MARGIN) * SIZE,
                    (1.0 - PADDING * 2.0) * SIZE,
                    (1.0 - PADDING * 2.0) * SIZE,
                );

                let mesh = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    0.1 * SIZE,
                    color::EMPTY_CELL,
                )?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        canvas.finish(ctx)
    }
}
