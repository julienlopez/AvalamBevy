use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::board::{BoardPosition, GridPosition};
use crate::stack::{are_not_stackable, Stack};
use crate::utils::stack_to_image_path;

fn are_positions_are_next_to_each_other(from: &GridPosition, to: &GridPosition) -> bool {
    let dx = (from.x as i32 - to.x as i32).abs();
    let dy = (from.y as i32 - to.y as i32).abs();
    dx + dy == 1
}

fn positions_are_next_to_each_other(
    dropped: Entity,
    target: Entity,
    query_positions: &Query<&BoardPosition>,
) -> Result<bool, QueryEntityError> {
    let dropped_pos = &query_positions.get(dropped)?.grid_pos;
    let target_pos = &query_positions.get(target)?.grid_pos;
    Ok(are_positions_are_next_to_each_other(
        dropped_pos,
        target_pos,
    ))
}

fn move_is_invalid(
    dropped: Entity,
    target: Entity,
    query_positions: &Query<&BoardPosition>,
    query_stacks: &Query<&mut Stack>,
) -> Result<bool, QueryEntityError> {
    let dropped_stack: Stack = (*(&query_stacks.get(dropped)?)).clone();
    let target_stack: &Stack = *(&query_stacks.get(target)?);

    Ok(are_not_stackable(&dropped_stack, query_stacks.get(target)?)
        || target_stack.get_pieces().len() == 0
        || !positions_are_next_to_each_other(dropped, target, &query_positions)?)
}

fn merge_stacks(
    dropped: Entity,
    target: Entity,
    query_stacks: &mut Query<&mut Stack>,
) -> Result<(), QueryEntityError> {
    let dropped_stack: Stack = (*(&query_stacks.get(dropped)?)).clone();
    query_stacks.get_mut(target)?.push_stack(dropped_stack);
    *query_stacks.get_mut(dropped)? = Stack::default();
    Ok(())
}

fn update_sprites(
    dropped: Entity,
    target: Entity,
    asset_server: Res<AssetServer>,
    query_sprites: &mut Query<&mut Handle<Image>>,
    query_stacks: &Query<&mut Stack>,
) -> Result<(), QueryEntityError> {
    *query_sprites.get_mut(dropped)? =
        asset_server.load(stack_to_image_path((&query_stacks).get(dropped)?));
    *query_sprites.get_mut(target)? =
        asset_server.load(stack_to_image_path((&query_stacks).get(target)?));
    Ok(())
}

fn reposition_dropped_sprite(
    dropped: Entity,
    query_positions: &Query<&BoardPosition>,
    query_transforms: &mut Query<&mut Transform>,
) -> Result<(), QueryEntityError> {
    let original_dropped_position = query_positions.get(dropped)?;
    *query_transforms.get_mut(dropped)? = Transform::from_xyz(
        original_dropped_position.world_pos.x,
        original_dropped_position.world_pos.y,
        0.0,
    );
    Ok(())
}

fn do_stack_pieces(
    event: Listener<Pointer<Drop>>,
    asset_server: Res<AssetServer>,
    query_positions: Query<&BoardPosition>,
    mut query_sprites: Query<&mut Handle<Image>>,
    mut query_transforms: Query<&mut Transform>,
    mut query_stacks: Query<&mut Stack>,
) -> Result<(), QueryEntityError> {
    if move_is_invalid(event.dropped, event.target, &query_positions, &query_stacks)? {
        return Ok(());
    }

    merge_stacks(event.dropped, event.target, &mut query_stacks)?;

    update_sprites(
        event.dropped,
        event.target,
        asset_server,
        &mut query_sprites,
        &query_stacks,
    )?;

    reposition_dropped_sprite(event.dropped, &query_positions, &mut query_transforms)?;

    // commands.entity(event.dropped).despawn();
    Ok(())
}

pub fn stack_pieces(
    event: Listener<Pointer<Drop>>,
    asset_server: Res<AssetServer>,
    query_positions: Query<&BoardPosition>,
    query_sprites: Query<&mut Handle<Image>>,
    query_transforms: Query<&mut Transform>,
    query_stacks: Query<&mut Stack>,
) {
    do_stack_pieces(
        event,
        asset_server,
        query_positions,
        query_sprites,
        query_transforms,
        query_stacks,
    )
    .expect("Unable to find entity");
}

pub fn on_drag_end(
    event: Listener<Pointer<DragEnd>>,
    mut commands: Commands,
    query_positions: Query<&BoardPosition>,
    mut query_transforms: Query<&mut Transform>,
) {
    commands.entity(event.target).insert(Pickable::default()); // makes the entity pickable again

    // replace dropped to proper position
    let original_dropped_position = query_positions.get(event.target).unwrap();
    *query_transforms.get_mut(event.target).unwrap() = Transform::from_xyz(
        original_dropped_position.world_pos.x,
        original_dropped_position.world_pos.y,
        0.0,
    );
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

        assert!(!are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 4, y: 1 }
        ));
        assert!(!are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 0, y: 1 }
        ));
        assert!(!are_positions_are_next_to_each_other(
            &from,
            &GridPosition { x: 3, y: 2 }
        ));
    }
}
