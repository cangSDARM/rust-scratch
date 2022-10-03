use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAMEOVER_COLOR: Color = [0.90, 0.90, 0.90, 0.20];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 2.0;

pub struct Game {
  snake: Snake,

  food_exist: bool,
  food_coord: (i32, i32),

  width: i32,
  height: i32,

  game_over: bool,
  waiting_time: f64,
}

impl Game {
  pub fn new(width: i32, height: i32) -> Game {
    Game {
      snake: Snake::new(2, 2),
      waiting_time: 0.0,
      food_exist: true,
      food_coord: (6, 4),
      width,
      height,
      game_over: false,
    }
  }

  pub fn key_pressed(&mut self, key: Key) {
    if self.game_over {
      return;
    }

    let dir = match key {
      Key::Up => Some(Direction::Up),
      Key::Down => Some(Direction::Down),
      Key::Left => Some(Direction::Left),
      Key::Right => Some(Direction::Right),
      _: Some(self.snake.head_direction()),
    };

    if dir.unwrap() == self.snake.head_direction().oppsite() {
      return;
    }

    self.update_snake(dir);
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    self.snake.draw(con, g);

    if self.food_exist {
      draw_block(FOOD_COLOR, self.food_coord.0, self.food_coord.1, con, g);
    }

    draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
    draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

    if self.game_over {
      draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
    }
  }

  pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;

    if self.game_over {
      if self.waiting_time > RESTART_TIME {
        self.restart();
      }
      return;
    }

    if !self.food_exist {
      self.add_food();
    }

    if self.waiting_time > MOVING_PERIOD {
      self.update_snake(None);
    }
  }

  fn check_eating(&mut self) {
    let (head_x, head_y) = self.snake.head_position();
    if self.food_exist && self.food_coord.0 == head_x && self.food_coord.1 == head_y {
      self.food_exist = false;
      self.snake.restore_tail();
    }
  }

  fn check_snake_alive(&self, dir: Option<Direction>) -> bool {
    let (next_x, next_y) = self.snake.next_head(dir);
    
    if self.snake.overlap_tail(next_x, next_y) {
      return false;
    }

    next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
  }

  fn add_food(&mut self) {
    let mut rng = thread_rng();
    
    let mut nx = rng.gen_range(1, self.width - 1);
    let mut ny = rng.gen_range(1, self.height - 1);
    while self.snake.overlap_tail(nx, ny) {
      nx = rng.gen_range(1, self.width - 1);
      ny = rng.gen_range(1, self.height - 1);
    }

    self.food_coord = (nx, ny);
    self.food_exist = true;
  }

  fn update_snake(&mut self, dir: Option<Direction>) {
    if self.check_snake_alive(dir) {
      self.snake.move_forward(dir);
      self.check_eating();
    } else {
      self.game_over = true;
    }
    self.waiting_time = 0.0;
  }

  fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food_exist = true;
    self.food_coord = (6, 4);
    self.game_over = false;
  }
}
