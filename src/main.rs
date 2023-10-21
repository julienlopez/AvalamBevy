use bevy::prelude::*;

mod game;
mod gamestate;
mod menu;

use crate::gamestate::GameState;
use crate::{game::game_plugin::GamePlugin, menu::menu_plugin::MenuPlugin};

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>()
        .add_plugins((MenuPlugin, GamePlugin))
        .add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(500.0, 200.0, 0.0),
            ..default()
        },
        ..default()
    });
}
