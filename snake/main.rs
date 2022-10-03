extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
  let (w, h) = (20, 20);

  let mut window: PistonWindow = WindowSettings::new(
    "Snake",
    [to_coord_u32(w), to_coord_u32(h)]
  ).exist_on_esc(true)
    .build()
    .unwrap();

  let mut game = Game::new(w, h);

  while let Some(ev) = window.next() {
    if let Some(Button::KeyBoard(key)) = ev.press_args() {
      game.key_pressed(key);
    }
    window.draw_2d(&ev, |c, g, _| {
      clear(BACKGROUND_COLOR, g);
      game.draw(&c, g);
    });

    ev.update(|arg| {
      game.update(arg.dt);
    });
  }
}
