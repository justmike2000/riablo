
//use oorandom::Rand32;

use glam::*;
use ggez::{event, Context, GameResult, graphics};
//use ggez::input::mouse::MouseButton;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{GlBackendSpec, Image, draw, Rect,
                     ImageGeneric, clear, present};

use std::path;
use std::env;

const BASE_RESOLUTION: (f32, f32) = (800.0, 600.0);
//const STRETCHED_RESOLUTION: (f32, f32) = ((BASE_RESOLUTION.0 / RESOLUTION.0),
//                                       (BASE_RESOLUTION.1 / RESOLUTION.1));
const PLAYER_MOVEMENT: (f32, f32) = (5.00, 5.00);


fn get_scaled_resolution(coords: (f32, f32)) -> (f32, f32) {
    ((coords.0 / BASE_RESOLUTION.0).round(),
    (coords.1 / BASE_RESOLUTION.1).round())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Direction {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Default)]
struct Position {
    x: f32,
    y: f32,
}

impl Direction {

    pub fn update_from_keycode(&mut self, key: KeyCode, down: bool) {
        match key {
            KeyCode::Up => self.up = down,
            KeyCode::Down => self.down = down,
            KeyCode::Left => self.left = down,
            KeyCode::Right => self.right = down,
            _ => (),
        };
    }
}

struct Player {
    resolution: (f32, f32),
    position: Position,
    direction: Direction,
    texture: ImageGeneric<GlBackendSpec>,
}

impl Player {

    fn new(ctx: &mut Context, resolution: (f32, f32)) -> Player {
        Player {
            position: Position::default(),
            direction: Direction::default(),
            texture: Image::new(ctx, 
            "/hero.png".to_string()).unwrap(),
            resolution,
        }
    }

    fn update(&mut self) {
        if self.direction.up {
            self.position.y -= PLAYER_MOVEMENT.1;

        }
        if self.direction.down {
            self.position.y += PLAYER_MOVEMENT.1;

        }
        if self.direction.left {
            self.position.x -= PLAYER_MOVEMENT.0;

        }
        if self.direction.right {
            self.position.x += PLAYER_MOVEMENT.0;
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        let param = graphics::DrawParam::new()
        .src(graphics::Rect {x: 0.00, y: 0.00, w: 0.25, h: 0.25})
        .dest(Vec2::new(self.position.x * get_scaled_resolution(self.resolution).0, 
                              self.position.y * get_scaled_resolution(self.resolution).1))
        .offset(Vec2::new(0.00, 0.00))
        // Scale image based on resolution
        .scale(Vec2::new(get_scaled_resolution(self.resolution).0,
                                get_scaled_resolution(self.resolution).1));
        draw(ctx, &self.texture, param)?;
        Ok(())
    }
}

struct GameState {
    player: Player,
    resolution: (f32, f32),
}

impl GameState {
    pub fn new(ctx: &mut Context, resolution: (f32, f32)) -> Self {

        GameState {
            player: Player::new(ctx, resolution),
            resolution
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear background
        clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let green_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0,
                0.0, 
                BASE_RESOLUTION.0 * get_scaled_resolution(self.resolution).0, 
                BASE_RESOLUTION.1 * get_scaled_resolution(self.resolution).1),
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &green_rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        // Draw Player
        self.player.draw(ctx)?;
        present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    //fn mouse_button_down_event(
    //    &mut self,
    //    _ctx: &mut Context,
    //    button: MouseButton,
    //    x: f32,
    //    y: f32,
    //) {
    //}

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.player.direction.update_from_keycode(keycode, true);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
    ) {
        self.player.direction.update_from_keycode(keycode, false);
    }
}

fn main() -> GameResult {

    let resolution: (f32, f32) = (1920.0, 1080.0);

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
        .window_mode(ggez::conf::WindowMode::default().dimensions(resolution.0, resolution.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = GameState::new(&mut ctx, resolution);
    event::run(ctx, events_loop, state)
}