use element::{elements, ElementType};
use ggez::{
    event,
    graphics::{self, set_window_title, Color, Drawable},
    mint::{Point2, Vector2},
    timer, ContextBuilder, GameError, GameResult, input::mouse,
};

mod util;
mod global;
mod element;
mod world;

struct MainState {
    grid: world::grid::Grid,
    frame: u8,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            grid: world::grid::Grid::new(),
            frame: 0,
        })
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        self.frame = (self.frame + 1) % u8::MAX;

        if mouse::button_pressed(ctx, event::MouseButton::Left) || mouse::button_pressed(ctx, event::MouseButton::Right) {
            let element = match mouse::button_pressed(ctx, event::MouseButton::Right) {
                true => elements::WATER,
                _ => elements::SAND
            };

            let pos = mouse::position(ctx);
            for i in 0..128 {
                for j in 0..128 {
                    let x = (pos.x / global::CELL_SIZE as f32) as i32 + (i - 64);
                    let y = (pos.y / global::CELL_SIZE as f32) as i32 + (j - 64);

                    if self.grid.is_empty_and_within_bounds(x, y) {
                        self.grid.set_element(x, y, element);
                    }
                }
            }
        }

        for y in (0..self.grid.height()).rev() {
            for x in 0..self.grid.width() {
                {
                    let mut element = self.grid.get_element_mut(x, y);
                    if element.element_type == ElementType::Empty || element.frame == self.frame {
                        continue;
                    }

                    element.frame = self.frame;
                }

                self.grid.update_element(x, y);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::BLACK);

        let fps = timer::fps(ctx).floor().to_string();
        set_window_title(ctx, format!("FPS: {}", fps).as_str());

        let mut buffer =
            util::Buffer::new(self.grid.width() as usize, self.grid.height() as usize);
        for x in 0..self.grid.width() {
            for y in 0..self.grid.height() {
                if self.grid.is_empty(x, y) {
                    continue;
                }

                let element = self.grid.get_element(x, y);
                buffer.set_pixel_color(x as usize, y as usize, element.color.into())
            }
        }

        let img_params = graphics::DrawParam::new()
            .dest(Point2 { x: 0.0, y: 0.0 })
            .scale(Vector2 {
                x: global::CELL_SIZE as f32,
                y: global::CELL_SIZE as f32,
            });

        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        graphics::Image::from_rgba8(
            ctx,
            buffer.width().try_into().unwrap(),
            buffer.height().try_into().unwrap(),
            buffer.data.as_slice(),
        )?
        .draw(ctx, img_params)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("Game", "Me");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;

    event::run(ctx, event_loop, state)
}