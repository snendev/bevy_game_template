use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};

use crate::GameState;

#[derive(AssetCollection, Resource)]
pub struct LogoAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

// This layer isn't particularly necessary, but may help manage groups of assets (for levels/etc)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(States)]
pub enum MenuLoadState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Active,
}

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuLoadState>()
            .add_loading_state(
                LoadingState::new(MenuLoadState::Loading)
                    .continue_to_state(MenuLoadState::Active)
                    .load_collection::<LogoAssets>(),
            )
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (
                    handle_play_button_click,
                    handle_logo_button_click,
                    render_button_interactions,
                )
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
    active: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            active: Color::rgb(0.30, 0.30, 0.30),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, textures: Res<LogoAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            Name::new("Menu"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    commands
        .spawn((
            Name::new("Social Links"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("Bevy Button"),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    Name::new("Github Button"),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                        active: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/snendev/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Source code",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn handle_play_button_click(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &ChangeState), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, change_state) in interaction_query.iter() {
        if let Interaction::Pressed = *interaction {
            next_state.set(change_state.0);
        }
    }
}

fn handle_logo_button_click(
    interaction_query: Query<(&Interaction, &OpenLink), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, open_link) in interaction_query.iter() {
        if let Interaction::Pressed = *interaction {
            if let Err(error) = webbrowser::open(open_link.0) {
                warn!("Failed to open link {error:?}");
            }
        }
    }
}

fn render_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = button_colors.active.into();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
