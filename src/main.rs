use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod stack;
use stack::{Piece, Stack};

mod stack_pieces;
use stack_pieces::stack_pieces;

mod utils;
use crate::utils::{stack_to_image_path, BoardPosition};

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
    color: Piece,
    x: f32,
) {
    let stack = Stack::new(vec![color]);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(stack_to_image_path(&stack)),
            transform: Transform::from_xyz(x, 0.0, 0.0),
            ..default()
        },
        stack,
        BoardPosition {
            pos: Vec2 { x: x, y: 0.0 },
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x; // Make the square follow the mouse
            transform.translation.y -= drag.delta.y;
        }),
        On::<Pointer<Drop>>::run(stack_pieces),
    ));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // possible to use commands.spawn_batch(vec![]);
    for x in -2..=2 {
        let x = x as f32 * 200.0;
        spawn_piece_stack(&mut commands, &asset_server, Piece::Red, x);
    }
}
