use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod stack;
use board::grid_2_world;
use stack::{Piece, Stack};

mod drag_and_drop_event;
use drag_and_drop_event::{on_drag_end, stack_pieces};

mod utils;
use crate::utils::{image_path, stack_to_image_path};

mod board;
use crate::board::{generate_all_positions, BoardPosition, GridPosition};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(low_latency_window_plugin()),
        DefaultPickingPlugins,
    ))
    .add_systems(Startup, setup);
    app.run();
}

fn spawn_piece_stack(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_position: &GridPosition,
) {
    let color = if grid_position.x % 2 == 0 {
        Piece::Yellow
    } else {
        Piece::Red
    };
    let stack = Stack::new(vec![color]);
    let board_position = BoardPosition::from_grid_pos(grid_position);
    let world_pos = board_position.world_pos;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(stack_to_image_path(&stack)),
            transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
            ..default()
        },
        board_position,
        stack,
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::run(on_drag_end),
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x;
            transform.translation.y -= drag.delta.y;
        }),
        On::<Pointer<Drop>>::run(stack_pieces),
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(500.0, 200.0, 0.0),
            ..default()
        },
        ..default()
    });

    // possible to use commands.spawn_batch(vec![]);
    for grid_pos in generate_all_positions() {
        spawn_piece_stack(&mut commands, &asset_server, &grid_pos);
        let world_pos = grid_2_world(&grid_pos);
        commands.spawn(SpriteBundle {
            texture: asset_server.load(image_path("xxxxx")),
            transform: Transform::from_xyz(world_pos.x, world_pos.y, -1.0),
            ..default()
        });
    }
}
