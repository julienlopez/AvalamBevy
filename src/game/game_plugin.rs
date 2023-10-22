use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{
    board::grid_2_world,
    board::{generate_all_positions, BoardPosition, GridPosition},
    drag_and_drop_event::{on_drag_end, stack_pieces},
    stack::{Piece, Stack},
    utils::{image_path, stack_to_image_path},
};
use crate::gamestate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins,
        )).add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, check_for_end_of_game.run_if(in_state(GameState::Game)))
            // .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            ;
    }
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform.translation.z = 1.0;
        }),
        On::<Pointer<Drop>>::run(stack_pieces),
    ));
}

fn check_for_end_of_game(query: Query<(&BoardPosition, &Stack)>) {
    let stacks: Vec<(&BoardPosition, &Stack)> = query
        .iter()
        .filter(|(_, stack)| stack.get_pieces().len() > 0 && stack.get_pieces().len() < 5)
        .collect();
    println!("check_for_end_of_game() : {}", stacks.len());
    for (board_pos, stack) in stacks {
        println!(
            "({}, {}) => {}",
            board_pos.grid_pos.x,
            board_pos.grid_pos.y,
            stack.get_pieces().len()
        );
    }
    // for p in stacks.iter().combinations_with_replacement(2) {
    //     println!(
    //         "({}, {}) : {} | {}, {}) : {}",
    //         p[0].board_pos.grid_pos.x,
    //         p[0].board_pos.grid_pos.y,
    //         p[0].stack.get_pieces().len(),
    //         p[1].board_pos.grid_pos.x,
    //         p[1].board_pos.grid_pos.y,
    //         p[1].stack.get_pieces().len()
    //     );
    // }
}
