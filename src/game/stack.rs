use bevy::ecs::component::Component;

#[derive(Debug, PartialEq, Clone)]
pub enum Piece {
    Red,
    Yellow,
}

#[derive(Component, Clone)]
pub struct Stack {
    pieces: Vec<Piece>,
}

impl Default for Stack {
    fn default() -> Self {
        Stack { pieces: vec![] }
    }
}

fn piece_to_char(piece: &Piece) -> char {
    match piece {
        Piece::Red => 'r',
        Piece::Yellow => 'y',
    }
}

impl Stack {
    pub fn new(pieces: Vec<Piece>) -> Stack {
        Stack { pieces }
    }

    pub fn push(&mut self, new_piece: Piece) {
        self.pieces.push(new_piece);
    }

    pub fn push_stack(&mut self, mut stack: Stack) {
        self.pieces.append(&mut stack.pieces);
    }

    pub fn pop(&mut self) {
        self.pieces.pop();
    }

    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn to_string(&self) -> String {
        let base: String = self.pieces.iter().map(piece_to_char).collect();
        format!("{:x<5}", base).to_string()
    }
}

pub fn are_not_stackable(s1: &Stack, s2: &Stack) -> bool {
    s1.pieces.len() + s2.pieces.len() > 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_stack_manipulation() {
        let mut stack = Stack::default();
        assert!(stack.get_pieces().is_empty());
        stack.push(Piece::Yellow);
        assert_eq!(stack.get_pieces(), &vec![Piece::Yellow]);
        stack.push(Piece::Red);
        assert_eq!(stack.get_pieces(), &vec![Piece::Yellow, Piece::Red]);
    }

    #[test]
    fn to_string() {
        assert_eq!(Stack::default().to_string(), "xxxxx");
        assert_eq!(Stack::new(vec![Piece::Red]).to_string(), "rxxxx");
        assert_eq!(Stack::new(vec![Piece::Yellow]).to_string(), "yxxxx");
        assert_eq!(
            Stack::new(vec![Piece::Red, Piece::Red]).to_string(),
            "rrxxx"
        );
        assert_eq!(
            Stack::new(vec![Piece::Yellow, Piece::Yellow]).to_string(),
            "yyxxx"
        );
        assert_eq!(
            Stack::new(vec![Piece::Red, Piece::Yellow]).to_string(),
            "ryxxx"
        );
    }

    #[test]
    fn full_test() {
        let mut stack = Stack::default();
        assert_eq!(stack.to_string(), "xxxxx");
        stack.push(Piece::Red);
        assert_eq!(stack.to_string(), "rxxxx");
        stack.push(Piece::Yellow);
        assert_eq!(stack.to_string(), "ryxxx");
        stack.push(Piece::Red);
        assert_eq!(stack.to_string(), "ryrxx");
        stack.push(Piece::Red);
        assert_eq!(stack.to_string(), "ryrrx");
        stack.pop();
        assert_eq!(stack.to_string(), "ryrxx");
        stack.push_stack(Stack::new(vec![Piece::Yellow, Piece::Red]));
        assert_eq!(stack.to_string(), "ryryr");
    }
}
