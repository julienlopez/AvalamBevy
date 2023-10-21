use crate::game::stack::Stack;

pub fn image_path(filename_root: &str) -> String {
    format!("images/{}.png", filename_root).to_string()
}

pub fn stack_to_image_path(stack: &Stack) -> String {
    image_path(&stack.to_string())
}
