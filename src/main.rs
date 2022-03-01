
use oorandom::Rand32;

use glam::*;
use ggez::{event, timer, Context, GameResult, graphics};
use ggez::input::mouse::MouseButton;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{GlBackendSpec, Image, draw,
                     ImageGeneric, clear, present};

use std::path;
use std::env;

const RESOLUTION: (f32, f32) = (1920.0, 1080.0);

const BASE_RESOLUTION: (f32, f32) = (800.0, 600.0);
const STRETCHED_RESOLUTION: (f32, f32) = ((BASE_RESOLUTION.0 / RESOLUTION.0),
                                       (BASE_RESOLUTION.1 / RESOLUTION.1));
const SCALED_RESOLUTION: (f32, f32) = ((RESOLUTION.0 / BASE_RESOLUTION.0),
                                       (RESOLUTION.1 / BASE_RESOLUTION.1));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Player {
    texture: ImageGeneric<GlBackendSpec>,
}

impl Player {

    fn new(ctx: &mut Context) -> Player {
        Player {
            texture: Image::new(ctx, 
                "/hero.png".to_string()).unwrap(),
        }
    }

    fn update(&mut self) {
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        let param = graphics::DrawParam::new()
        .src(graphics::Rect {x: 0.00, y: 0.00, w: 0.25, h: 0.25})
        .dest(Vec2::new(RESOLUTION.0 / 2.00, 
                              RESOLUTION.1 / 2.00))
        .offset(Vec2::new(0.60, 0.50))
        // Scale image based on resolution
        .scale(Vec2::new(RESOLUTION.0 / BASE_RESOLUTION.0,
                                RESOLUTION.1 / BASE_RESOLUTION.1));
        draw(ctx, &self.texture, param)?;
        Ok(())
    }
}

struct GameState {
    player: Player,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {

        GameState {
            player: Player::new(ctx)
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        self.player.draw(ctx)?;
        present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let stretched_x = STRETCHED_RESOLUTION.0 * x;
        let stretched_y = STRETCHED_RESOLUTION.0 * y;
        println!("{}, {}", stretched_x, stretched_y);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
    }
}

fn main() -> GameResult {

    let window_setup = ggez::conf::WindowSetup::default().title("Riablo");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("textures");
        path
    } else {
        path::PathBuf::from("./textures")
    };

    let (mut ctx, events_loop) = ggez::ContextBuilder::new("player", "Mitt Miles")
        .window_setup(window_setup)
        .window_mode(ggez::conf::WindowMode::default().dimensions(RESOLUTION.0, RESOLUTION.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = GameState::new(&mut ctx);
    event::run(ctx, events_loop, state)
}