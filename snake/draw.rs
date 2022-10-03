use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;


const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
  (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
  to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
  let gui_x = to_coord(x);
  let gui_y = to_coord(y);

  rectangle(
    color,
    [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
    con.transform,
    g,
  );
}

pub fn draw_rectangle(
  color: Color,
  x: i32,
  y: i32,
  size: (i32, i32),
  con: &Context,
  g: &mut G2d,
) {
  let gui_x = to_coord(x);
  let gui_y = to_coord(y);

  rectangle(
    color,
    [gui_x, gui_y, BLOCK_SIZE * (size.0 as f64), BLOCK_SIZE * (size.1 as f64)],
    con.transform,
    g,
  );
}
