use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        if let Some(d) = dir {
            moving_dir = d
        };

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_new() {
        let snake = Snake::new(2, 2);
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.head_position(), (4, 2));
        assert_eq!(snake.head_direction(), Direction::Right);
    }

    #[test]
    fn test_snake_move_forward() {
        let mut snake = Snake::new(2, 2);
        snake.move_forward(Some(Direction::Right));
        assert_eq!(snake.head_position(), (5, 2));
        assert_eq!(snake.body.len(), 3);

        snake.move_forward(Some(Direction::Up));
        assert_eq!(snake.head_position(), (5, 1));
        assert_eq!(snake.body.len(), 3);

        snake.move_forward(None);
        assert_eq!(snake.head_position(), (5, 0));
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_snake_next_head() {
        let snake = Snake::new(2, 2);
        assert_eq!(snake.next_head(Some(Direction::Up)), (4, 1));
        assert_eq!(snake.next_head(Some(Direction::Down)), (4, 3));
        assert_eq!(snake.next_head(Some(Direction::Left)), (3, 2));
        assert_eq!(snake.next_head(Some(Direction::Right)), (5, 2));
        assert_eq!(snake.next_head(None), (5, 2));
    }

    #[test]
    fn test_snake_overlap_tail() {
        let mut snake = Snake::new(2, 2);
        snake.move_forward(Some(Direction::Right));
        snake.move_forward(Some(Direction::Right));
        snake.move_forward(Some(Direction::Up));
        snake.move_forward(Some(Direction::Left));
        assert_eq!(snake.overlap_tail(3, 2), false);
        assert_eq!(snake.overlap_tail(2, 2), false);
    }

    #[test]
    fn test_snake_restore_tail() {
        let mut snake = Snake::new(2, 2);
        snake.move_forward(Some(Direction::Right));
        snake.move_forward(Some(Direction::Right));
        assert_eq!(snake.body.len(), 3);
        snake.restore_tail();
        assert_eq!(snake.body.len(), 4);
    }
}
