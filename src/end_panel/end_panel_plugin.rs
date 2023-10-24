use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::ui_utils::despawn_screen;

pub struct EndPanelPlugin;

impl Plugin for EndPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EndPanel), screen_setup)
            .add_systems(
                Update,
                (screen_action, button_system).run_if(in_state(GameState::EndPanel)),
            )
            .add_systems(
                OnExit(GameState::EndPanel),
                despawn_screen::<EndPanelScreen>,
            );
    }
}

#[derive(Component)]
struct EndPanelScreen;

fn screen_setup(mut commands: Commands) {
    println!("plop");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            EndPanelScreen,
        ))
        .with_children(|parent| {});
}
fn screen_action() {}
fn button_system() {}
