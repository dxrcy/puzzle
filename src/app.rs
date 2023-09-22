use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, DrawMode, DrawParam, Mesh, Rect, TextLayout},
    mint::Point2,
    Context,
};

use crate::{
    color,
    grid::{Grid, GridTile},
    GRID_SIZE, TEXT_SIZE, TILE_SIZE,
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
            // std::thread::sleep(std::time::Duration::from_secs(1));
            self.reset(ctx)
        }

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

        if let Some((x, y)) = self.active_tile {
            let point = Point2 {
                x: x as f32 + 0.5,
                y: y as f32 + 0.5,
            };

            let mesh = graphics::Mesh::new_circle(
                ctx,
                DrawMode::stroke(0.05),
                point,
                0.3,
                0.01,
                color!(255, 180, 180),
            )?;
            canvas.draw(&mesh, param);

            if let Some((new_x, new_y)) = self.grid.find_empty(x, y) {
                let new_point = Point2 {
                    x: new_x as f32 + 0.5,
                    y: new_y as f32 + 0.5,
                };

                let mesh = graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::stroke(0.05),
                    new_point,
                    0.3,
                    0.01,
                    color!(180, 255, 180),
                )?;
                canvas.draw(&mesh, param);
            }
        }

        canvas.finish(ctx)
    }
}
