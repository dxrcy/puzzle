use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, DrawMode, DrawParam, Mesh, Rect, TextLayout},
    winit::event::VirtualKeyCode,
    Context,
};

use crate::{
    color,
    grid::{Grid, GridTile},
    GRID_SIZE, MARGIN, PADDING, SIZE, TEXT_SIZE, TILE_SIZE,
};

pub struct App {
    grid: Grid,
    active_tile: Option<(usize, usize)>,
    using_keyboard: bool,
}

impl App {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            grid: Grid::new(),
            active_tile: None,
            using_keyboard: false,
        }
    }

    fn reset(&mut self, ctx: &mut Context) {
        *self = Self::new(ctx)
    }

    fn navigate_active_tile(&mut self, dx: isize, dy: isize) {
        let Some(active) = self.active_tile else {
            self.active_tile = Some((0, 0));
            return;
        };

        let x = active.0 as isize + dx;
        let y = active.1 as isize + dy;

        let range = 0..GRID_SIZE as isize;

        if range.contains(&x) && range.contains(&y) {
            self.active_tile = Some((x as usize, y as usize))
        };
    }

    fn move_tile(&mut self) {
        if let Some((x, y)) = self.active_tile {
            self.grid.shift_tiles(x, y);
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if ctx.mouse.button_just_pressed(MouseButton::Left) {
            self.move_tile();
        }

        if self.grid.is_complete() {
            println!("Well done!");
            std::process::exit(0)
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

            if self.using_keyboard {
                let rect = Rect::new(
                    (x as f32 + PADDING + MARGIN) * SIZE,
                    (y as f32 + PADDING + MARGIN) * SIZE,
                    (1.0 - PADDING * 2.0) * SIZE,
                    (1.0 - PADDING * 2.0) * SIZE,
                );

                let mesh = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::stroke(0.05 * SIZE),
                    rect,
                    0.1 * SIZE,
                    color::TILE_ACTIVE_KEYS,
                )?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        canvas.finish(ctx)
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.using_keyboard = false;

        let (x, y) = (x / TILE_SIZE, y / TILE_SIZE);

        let range = 0.0..GRID_SIZE as f32;
        self.active_tile = if range.contains(&x) && range.contains(&y) {
            Some((x as usize, y as usize))
        } else {
            None
        };

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.using_keyboard = true;

        use VirtualKeyCode as Key;
        match input.keycode {
            // Reset game
            Some(Key::R) => self.reset(ctx),

            // Move tiles
            Some(Key::Space | Key::Return) => self.move_tile(),

            // Navigate
            Some(Key::Left | Key::H | Key::A) => self.navigate_active_tile(-1, 0),
            Some(Key::Down | Key::J | Key::S) => self.navigate_active_tile(0, 1),
            Some(Key::Up | Key::K | Key::W) => self.navigate_active_tile(0, -1),
            Some(Key::Right | Key::L | Key::D) => self.navigate_active_tile(1, 0),

            _ => (),
        }

        Ok(())
    }
}
