mod game;
mod main_menu;
mod utils;

use bevy::{prelude::*, winit::WinitSettings};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Built-in logs. Comment out for additional data
        // .add_plugins((
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin::default(),
        // ))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AppState>()
        // Main menu systems
        .add_systems(
            Startup,
            main_menu::setup_menu.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            main_menu::update_menu.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), utils::cleanup_system::<Node>)
        // Game system
        .add_systems(
            OnEnter(AppState::InGame),
            game::setup.run_if(in_state(AppState::InGame)),
        )
        .run();
}
