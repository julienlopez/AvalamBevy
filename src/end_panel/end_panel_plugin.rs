use bevy::prelude::*;

use crate::gamestate::{FinalScore, GameState};
use crate::ui_utils::{button_system, despawn_screen, NORMAL_BUTTON, TEXT_COLOR};

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

fn screen_setup(mut commands: Commands, score: Res<FinalScore>) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

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
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Yellow won!",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                })
                .with_children(|parent| {
                    parent
                        .spawn((ButtonBundle {
                            style: button_style,
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Ok", button_text_style));
                        });
                });
        });
}

fn screen_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction) in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
        }
    }
}
