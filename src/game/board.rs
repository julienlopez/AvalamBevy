use bevy::prelude::{Component, Vec2};

#[derive(Clone, Debug)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct BoardPosition {
    pub grid_pos: GridPosition,
    pub world_pos: Vec2,
}

impl BoardPosition {
    pub fn from_grid_pos(grid_pos: &GridPosition) -> Self {
        Self {
            grid_pos: grid_pos.clone(),
            world_pos: grid_2_world(grid_pos),
        }
    }
}

pub fn grid_2_world(grid_pos: &GridPosition) -> Vec2 {
    Vec2 {
        x: 50.0 * (grid_pos.x + grid_pos.y) as f32,
        y: 50.0 * ((4 + 2 * grid_pos.y) - grid_pos.x) as f32,
    }
}

pub fn generate_all_positions() -> Vec<GridPosition> {
    vec![
        GridPosition { x: 0, y: 1 },
        GridPosition { x: 0, y: 2 },
        GridPosition { x: 1, y: 1 },
        GridPosition { x: 1, y: 2 },
        GridPosition { x: 1, y: 3 },
        GridPosition { x: 2, y: 0 },
        GridPosition { x: 2, y: 1 },
        GridPosition { x: 2, y: 2 },
        GridPosition { x: 2, y: 3 },
        GridPosition { x: 3, y: 0 },
        GridPosition { x: 3, y: 1 },
        GridPosition { x: 3, y: 2 },
        GridPosition { x: 3, y: 3 },
        GridPosition { x: 3, y: 4 },
        GridPosition { x: 4, y: 1 },
        GridPosition { x: 4, y: 2 },
        GridPosition { x: 4, y: 3 },
        GridPosition { x: 4, y: 4 },
        GridPosition { x: 5, y: 2 },
        GridPosition { x: 5, y: 3 },
        GridPosition { x: 5, y: 4 },
        GridPosition { x: 5, y: 5 },
        GridPosition { x: 6, y: 2 },
        GridPosition { x: 6, y: 3 },
        GridPosition { x: 6, y: 5 },
        GridPosition { x: 6, y: 6 },
        GridPosition { x: 7, y: 3 },
        GridPosition { x: 7, y: 4 },
        GridPosition { x: 7, y: 5 },
        GridPosition { x: 7, y: 6 },
        GridPosition { x: 8, y: 4 },
        GridPosition { x: 8, y: 5 },
        GridPosition { x: 8, y: 6 },
        GridPosition { x: 8, y: 7 },
        GridPosition { x: 9, y: 4 },
        GridPosition { x: 9, y: 5 },
        GridPosition { x: 9, y: 6 },
        GridPosition { x: 9, y: 7 },
        GridPosition { x: 9, y: 8 },
        GridPosition { x: 10, y: 5 },
        GridPosition { x: 10, y: 6 },
        GridPosition { x: 10, y: 7 },
        GridPosition { x: 10, y: 8 },
        GridPosition { x: 11, y: 5 },
        GridPosition { x: 11, y: 6 },
        GridPosition { x: 11, y: 7 },
        GridPosition { x: 12, y: 6 },
        GridPosition { x: 12, y: 7 },
    ]
}

pub fn are_positions_are_next_to_each_other(from: &GridPosition, to: &GridPosition) -> bool {
    let dx = (from.x as i32 - to.x as i32).abs();
    let dy = (from.y as i32 - to.y as i32).abs();
    dx + dy == 1 || (dx == 1 && dy == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_positions_are_next_to_each_other_from_2_1() {
        let from = GridPosition { x: 2, y: 1 };
        assert!(are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 2, y: 2 }
        ));
        assert!(are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 2, y: 0 }
        ));
        assert!(are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 3, y: 1 }
        ));
        assert!(are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 1, y: 1 }
        ));
        assert!(are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 3, y: 2 }
        ));

        assert!(!are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 4, y: 1 }
        ));
        assert!(!are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 0, y: 1 }
        ));
    }
}
