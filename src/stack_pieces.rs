use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::board::BoardPosition;
use crate::stack::Stack;
use crate::utils::stack_to_image_path;

pub fn stack_pieces(
    // The event data accessible by the callback system
    event: Listener<Pointer<Drop>>,
    asset_server: Res<AssetServer>,
    // mut commands: Commands,
    query_positions: Query<&BoardPosition>,
    mut query_sprites: Query<&mut Handle<Image>>,
    mut query_transforms: Query<&mut Transform>,
    mut query_stacks: Query<&mut Stack>,
) {
    // merge the stacks
    let dropped_stack: Stack = (*(&query_stacks.get(event.dropped).unwrap())).clone();
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
}
