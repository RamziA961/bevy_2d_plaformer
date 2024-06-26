use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource as KiraAudioSource;

use crate::{
    game_state::{GameState, GameStateTransitionState},
    ui,
};

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub struct MenuAssetCollection {
    #[asset(path = "sound-effects/ui/press.wav")]
    pub press_sound: Handle<KiraAudioSource>,

    #[asset(path = "sound-effects/ui/hover.wav")]
    pub hover_sound: Handle<KiraAudioSource>,
}

pub fn initialize_systems(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup_menu_scene)
        .add_systems(
            StateTransition,
            (ui::menu::render_menu_ui)
                .run_if(in_state(GameStateTransitionState::Next))
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (ui::menu::menu_button_system)
                .run_if(in_state(GameStateTransitionState::Done))
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), ui::menu::cleanup_menu_ui);
}

pub fn setup_menu_scene(mut commands: Commands) {
    info!("Configuring menu scene");
    // camera
    // translation is along the x-axis
    commands.spawn(Camera2dBundle::default());

    // lighting
    // translation is along the x-axis
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-12., 0., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 10_000_000.,
            range: 10_000.,
            ..default()
        },
        ..default()
    });
    info!("Main menu scene configuration successful");
}
