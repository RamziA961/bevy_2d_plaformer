use bevy::{prelude::*, time::Stopwatch};
use std::time::Duration;

use crate::{game_state::GameState, ui::hud};

#[derive(Component)]
pub struct LevelTimer(Stopwatch);

impl LevelTimer {
    pub fn get_time(&self) -> Duration {
        self.0.elapsed()
    }
}

pub fn initialize_systems(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), hud::render_hud_ui);
    app.add_systems(FixedUpdate, tick_level_timer);
}

#[allow(unused_must_use)]
pub fn tick_level_timer(time: Res<Time>, mut timer_query: Query<&mut LevelTimer>) {
    // there should only be one level timer
    timer_query
        .get_single_mut()
        .map(|mut timer| {
            timer.0.tick(time.delta());
        })
        .map_err(|e| {
            error_once!(e=%e, "Could not find level timer");
        });
}
