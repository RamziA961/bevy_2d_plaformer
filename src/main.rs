use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use game_state::{GameState, GameStateTransitionState};

pub mod game_state;
pub mod menu;
pub mod player;
pub mod ui;

fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "2D Platformer".to_string(),
            ..default()
        }),
        ..default()
    });

    let mut app = App::new();
    app.add_plugins((default_plugins, AudioPlugin))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));

    // game and loading states
    app.init_state::<GameStateTransitionState>();
    app.init_state::<GameState>();

    // general asset loading
    ui::load_assets(&mut app);

    // configure initial app states
    app.add_systems(PreStartup, pre_startup_system);

    app.add_loading_state(
        // TODO: Investigate deferring asset loading
        LoadingState::new(GameStateTransitionState::AssetLoading)
            .load_collection::<menu::MenuAssetCollection>()
            .continue_to_state(GameStateTransitionState::Next),
    )
    .add_loading_state(
        LoadingState::new(GameStateTransitionState::Next)
            .continue_to_state(GameStateTransitionState::Done),
    );

    menu::initialize_systems(&mut app);
    player::initialize_systems(&mut app);
    ui::initialize_systems(&mut app);
    

    app.run()
}

fn pre_startup_system(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_game_transition_state: ResMut<NextState<GameStateTransitionState>>,
) {
    info!("Pre-startup started");
    next_game_transition_state.set(GameStateTransitionState::AssetLoading);
    next_game_state.set(GameState::MainMenu);
    info!("Pre-startup complete");
}
