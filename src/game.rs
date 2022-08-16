use bevy::prelude::*;

use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
) {
    let font = asset_server.load("fonts\\fira-sans.bold.ttf");

    commands
        // First create a `NodeBundle` for centering what we want to display
        .spawn_bundle(NodeBundle {
            style: Style {
                // This will center the current node
                margin: UiRect::all(Val::Auto),
                // This will display its children in a column, from top to bottom. Unlike
                // in Flexbox, Bevy origin is on bottom left, so the vertical axis is reversed
                flex_direction: FlexDirection::ColumnReverse,
                // `align_items` will align children on the cross axis. Here the main axis is
                // vertical (column), so the cross axis is horizontal. This will center the
                // children
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(OnGameScreen)
        .with_children(|parent| {
            // Display two lines of text, the second one with the current settings
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Will be back to the menu shortly...",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        format!("quality: {:?}", *display_quality),
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::BLUE,
                        },
                    ),
                    TextSection::new(
                        " - ",
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: TEXT_COLOR,
                        },
                    ),
                    TextSection::new(
                        format!("volume: {:?}", *volume),
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::GREEN,
                        },
                    ),
                ])
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, false)));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}