use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::board::BoardPosition;
use crate::stack::{are_not_stackable, Stack};
use crate::utils::stack_to_image_path;

fn move_is_invalid(
    dropped: Entity,
    target: Entity,
    query_positions: &Query<&BoardPosition>,
    query_stacks: &Query<&mut Stack>,
) -> bool {
    let dropped_stack: Stack = (*(&query_stacks.get(dropped).unwrap())).clone();
    let target_stack: &Stack = *(&query_stacks.get(target).unwrap());
    are_not_stackable(&dropped_stack, query_stacks.get(target).unwrap())
        || target_stack.get_pieces().len() == 0
}

fn merge_stacks(dropped: Entity, target: Entity, query_stacks: &mut Query<&mut Stack>) {
    let dropped_stack: Stack = (*(&query_stacks.get(dropped).unwrap())).clone();
    query_stacks
        .get_mut(target)
        .unwrap()
        .push_stack(dropped_stack);
    *query_stacks.get_mut(dropped).unwrap() = Stack::default();
}

fn update_sprites(
    dropped: Entity,
    target: Entity,
    asset_server: Res<AssetServer>,
    query_sprites: &mut Query<&mut Handle<Image>>,
    query_stacks: &Query<&mut Stack>,
) {
    *query_sprites.get_mut(dropped).unwrap() =
        asset_server.load(stack_to_image_path(&(&query_stacks).get(dropped).unwrap()));
    *query_sprites.get_mut(target).unwrap() =
        asset_server.load(stack_to_image_path(&(&query_stacks).get(target).unwrap()));
}

fn reposition_dropped_sprite(
    dropped: Entity,
    query_positions: &Query<&BoardPosition>,
    query_transforms: &mut Query<&mut Transform>,
) {
    let original_dropped_position = query_positions.get(dropped).unwrap();
    *query_transforms.get_mut(dropped).unwrap() = Transform::from_xyz(
        original_dropped_position.world_pos.x,
        original_dropped_position.world_pos.y,
        0.0,
    );
}

pub fn stack_pieces(
    event: Listener<Pointer<Drop>>,
    asset_server: Res<AssetServer>,
    query_positions: Query<&BoardPosition>,
    mut query_sprites: Query<&mut Handle<Image>>,
    mut query_transforms: Query<&mut Transform>,
    mut query_stacks: Query<&mut Stack>,
) {
    if move_is_invalid(event.dropped, event.target, &query_positions, &query_stacks) {
        return;
    }

    merge_stacks(event.dropped, event.target, &mut query_stacks);

    update_sprites(
        event.dropped,
        event.target,
        asset_server,
        &mut query_sprites,
        &query_stacks,
    );

    reposition_dropped_sprite(event.dropped, &query_positions, &mut query_transforms);

    // commands.entity(event.dropped).despawn();
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
