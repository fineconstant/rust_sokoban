use std::fs;
use std::path::{Path, PathBuf};

use ggez::nalgebra::{Point2, Point3};
use log::*;
use specs::{Component, NullStorage, VecStorage, World, WorldExt};

use crate::error::*;
use crate::sokoban_game::SokobanGame;
use core::fmt;

#[derive(Clone, Hash, Eq, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point2<usize>,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {
            point: Point2::from([x, y]),
        }
    }

    pub fn move_north(&mut self) -> SokobanResult<Position> {
        if self.point.y == 0 {
            return Err(SokobanError::OutOfBounds(self.clone()));
        }

        self.point.y -= 1;
        Ok(self.clone())
    }

    pub fn move_south(&mut self) -> SokobanResult<Position> {
        self.point.y += 1;
        Ok(self.clone())
    }

    pub fn move_west(&mut self) -> SokobanResult<Position> {
        if self.point.x == 0 {
            return Err(SokobanError::OutOfBounds(self.clone()));
        }

        self.point.x -= 1;
        Ok(self.clone())
    }

    pub fn move_east(&mut self) -> SokobanResult<Position> {
        self.point.x += 1;
        Ok(self.clone())
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("x", &self.point.x)
            .field("y", &self.point.y)
            .finish()
    }
}
