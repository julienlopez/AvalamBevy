use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::board::BoardPosition;
use crate::stack::{are_not_stackable, Stack};
use crate::utils::stack_to_image_path;

pub fn stack_pieces(
    event: Listener<Pointer<Drop>>,
    asset_server: Res<AssetServer>,
    query_positions: Query<&BoardPosition>,
    mut query_sprites: Query<&mut Handle<Image>>,
    mut query_transforms: Query<&mut Transform>,
    mut query_stacks: Query<&mut Stack>,
) {
    let dropped_stack: Stack = (*(&query_stacks.get(event.dropped).unwrap())).clone();
    let target_stack: Stack = (*(&query_stacks.get(event.target).unwrap())).clone();
    if are_not_stackable(&dropped_stack, query_stacks.get(event.target).unwrap())
        || target_stack.get_pieces().len() == 0
    {
        return;
    }

    // merge the stacks
    query_stacks
        .get_mut(event.target)
        .unwrap()
        .push_stack(dropped_stack);
    *query_stacks.get_mut(event.dropped).unwrap() = Stack::default();

    // update the sprites
    *query_sprites.get_mut(event.dropped).unwrap() = asset_server.load(stack_to_image_path(
        &(&query_stacks).get(event.dropped).unwrap(),
    ));
    *query_sprites.get_mut(event.target).unwrap() = asset_server.load(stack_to_image_path(
        &(&query_stacks).get(event.target).unwrap(),
    ));

    // replace dropped to proper position
    let original_dropped_position = query_positions.get(event.dropped).unwrap();
    *query_transforms.get_mut(event.dropped).unwrap() = Transform::from_xyz(
        original_dropped_position.world_pos.x,
        original_dropped_position.world_pos.y,
        0.0,
    );

    // commands.entity(event.dropped).despawn();
}

pub fn on_drag_end(
    event: Listener<Pointer<DragEnd>>,
    mut commands: Commands,
    query_positions: Query<&BoardPosition>,
    mut query_transforms: Query<&mut Transform>,
) {
    commands.entity(event.target).insert(Pickable::default());

    // replace dropped to proper position
    let original_dropped_position = query_positions.get(event.target).unwrap();
    *query_transforms.get_mut(event.target).unwrap() = Transform::from_xyz(
        original_dropped_position.world_pos.x,
        original_dropped_position.world_pos.y,
        0.0,
    );
}
