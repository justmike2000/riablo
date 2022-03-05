
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
//const STRETCHED_RESOLUTION: (f32, f32) = ((BASE_RESOLUTION.0 / RESOLUTION.0),
//                                       (BASE_RESOLUTION.1 / RESOLUTION.1));
const SCALED_RESOLUTION: (f32, f32) = ((RESOLUTION.0 / BASE_RESOLUTION.0),
                                       (RESOLUTION.1 / BASE_RESOLUTION.1));
const PLAYER_MOVEMENT: (f32, f32) = (5.00, 5.00);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Direction {
    Up: bool,
    Down: bool,
    Left: bool,
    Right: bool,
}

#[derive(Default)]
struct Position {
    x: f32,
    y: f32,
}

impl Direction {

    pub fn update_from_keycode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.Up = true,
            KeyCode::Down => self.Down = true,
            KeyCode::Left => self.Left = true,
            KeyCode::Right => self.Right = true,
            _ => (),
        };
    }
}

struct Player {
    position: Position,
    direction: Direction,
    texture: ImageGeneric<GlBackendSpec>,
}

impl Player {

    fn new(ctx: &mut Context) -> Player {
        Player {
            position: Position::default(),
            direction: Direction::default(),
            texture: Image::new(ctx, 
                "/hero.png".to_string()).unwrap(),
        }
    }

    fn update(&mut self) {
        if self.direction.Up {
            self.position.y -= PLAYER_MOVEMENT.1;

        }
        if self.direction.Down {
            self.position.y += PLAYER_MOVEMENT.1;

        }
        if self.direction.Left {
            self.position.x -= PLAYER_MOVEMENT.0;

        }
        if self.direction.Right {
            self.position.x += PLAYER_MOVEMENT.0;
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        let param = graphics::DrawParam::new()
        .src(graphics::Rect {x: 0.00, y: 0.00, w: 0.25, h: 0.25})
        .dest(Vec2::new(self.position.x * SCALED_RESOLUTION.0, 
                              self.position.y * SCALED_RESOLUTION.1))
        .offset(Vec2::new(0.00, 0.00))
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
        self.player.update();
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
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.player.direction.update_from_keycode(keycode);
        println!("{:?}", self.player.direction);
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