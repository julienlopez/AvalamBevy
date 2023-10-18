use bevy::prelude::*;

use crate::stack::Stack;

#[derive(Component)]
pub struct BoardPosition {
    pub pos: Vec2,
}

pub fn stack_to_image_path(stack: &Stack) -> String {
    format!("images/{}.png", stack.to_string()).to_string()
}
