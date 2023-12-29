use std::time::Duration;

use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::{
    default_windows::add::{AddItem, AddWindow},
    editor::Editor,
    prelude::*,
};
use bevy_game::{blueprints::CharacterBundle, controls::Controller};

pub enum EditorOpenSetting {
    Windowed,
    FullScreen,
}

pub fn app_with_editor(editor_settings: EditorOpenSetting) -> App {
    let mut app = App::new();
    let editor_plugin = match editor_settings {
        EditorOpenSetting::FullScreen => EditorPlugin::default().on_second_monitor_fullscreen(),
        EditorOpenSetting::Windowed => EditorPlugin::default().in_new_window(Window::default()),
    };
    app.add_plugins((
        DefaultPlugins,
        editor_plugin,
        FrameTimeDiagnosticsPlugin::default(),
        EntityCountDiagnosticsPlugin::default(),
        bevy_game::GamePlugin,
    ))
    .add_systems(Update, (handle_pause, propagate_window_despawn));

    register_blueprints(&mut app.world);

    app
}

// Use this to register blueprints to the editor
fn register_blueprints(world: &mut World) {
    let mut editor = world
        .get_resource_mut::<Editor>()
        .expect("Editor should exist");
    let state = editor
        .window_state_mut::<AddWindow>()
        .expect("AddWindow should exist");
    state.add("Blueprints", AddItem::bundle::<CharacterBundle>());
    state.add("Blueprints", AddItem::component::<Controller>());
}

fn handle_pause(
    mut commands: Commands,
    inputs: Res<Input<KeyCode>>,
    mut fixed_time: ResMut<Time<Fixed>>,
    mut pause_ui_entity: Local<Option<(Duration, Entity)>>,
) {
    // TODO: only if selecting the correct window?
    if inputs.just_pressed(KeyCode::Escape) {
        if let Some((prev_duration, entity)) = pause_ui_entity.take() {
            commands.entity(entity).despawn_recursive();
            fixed_time.set_timestep(prev_duration);
        } else {
            *pause_ui_entity = Some((
                fixed_time.timestep(),
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            left: Val::Percent(35.),
                            right: Val::Percent(35.),
                            top: Val::Percent(10.),
                            bottom: Val::Percent(70.),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        border_color: Color::BLACK.into(),
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            "PAUSED",
                            TextStyle {
                                font_size: 64.,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        ));
                    })
                    .id(),
            ));
            fixed_time.set_timestep(Duration::MAX);
        }
    }
}

fn propagate_window_despawn(
    mut commands: Commands,
    removed_windows: RemovedComponents<Window>,
    window_query: Query<Entity, With<Window>>,
) {
    if !removed_windows.is_empty() {
        for entity in window_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
