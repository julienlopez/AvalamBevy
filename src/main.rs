use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod stack;
use stack::{Piece, Stack};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(low_latency_window_plugin()),
        DefaultPickingPlugins,
    ))
    .add_systems(Startup, setup);
    app.run();
}

#[derive(Component)]
struct BoardPosition {
    pub pos: Vec2,
}

fn stack_to_image_path(stack: &Stack) -> String {
    format!("images/{}.png", stack.to_string()).to_string()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn pieces
    // possible to use commands.spawn_batch(vec![]);
    for x in -2..=2 {
        let x = x as f32 * 200.0;
        let stack = Stack::new(vec![Piece::Red]);
        println!("stack is {}", stack.to_string());
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
}

fn stack_pieces(
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
        original_dropped_position.pos.x,
        original_dropped_position.pos.y,
        0.0,
    );
}
