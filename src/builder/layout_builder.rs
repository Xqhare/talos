
use crate::layout::{Constraint, Direction, Layout};

pub struct LayoutBuilder {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: u16,
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {
            direction: Direction::Horizontal,
            constraints: Vec::new(),
            margin: 0,
        }
    }
}

impl LayoutBuilder {
    pub fn direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn margin(&mut self, margin: u16) -> &mut Self {
        self.margin = margin;
        self
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> &mut Self {
        self.constraints.push(constraint);
        self
    }

    pub fn build(&self) -> Layout {
        Layout::new(self.direction, self.constraints.clone(), self.margin)
    }
}
