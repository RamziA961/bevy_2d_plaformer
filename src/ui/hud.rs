use bevy::prelude::*;

use super::{FontAssetCollection, RefreshableUIElement};
use crate::{
    game_state::{GameState, GameStateTransitionState},
    player::LevelTimer,
};

#[derive(Component)]
struct HudUIElement;

#[derive(Component)]
struct TimerUIElement;

pub fn intialize_hud_systems(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_hud_timer)
            .run_if(in_state(GameState::InGame))
            .run_if(in_state(GameStateTransitionState::Done)),
    );
}

fn update_hud_timer(
    mut timer_ui_query: Query<
        &mut Text,
        (
            With<RefreshableUIElement>,
            With<HudUIElement>,
            With<TimerUIElement>,
        ),
    >,
    timer_query: Query<&LevelTimer>,
) {
    let ui_text = timer_ui_query.get_single_mut();
    let timer = timer_query.get_single();

    match (ui_text, timer) {
        (Ok(mut text), Ok(timer)) => {
            info_once!("Found timer and timer UI element");
            let elapsed = timer.get_time().as_secs();
            let m = elapsed / 60;
            let s = elapsed % (m * 60);
            text.sections[0].value = format!("{:0>2}:{:0>2}", m, s);
        }
        _ => {
            info_once!("Could not find timer and/or timer UI element");
        }
    }
}

pub fn render_hud_ui(mut commands: Commands, fonts: Res<FontAssetCollection>) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    commands.spawn(container).with_children(|parent| {
        parent.spawn(compose_header()).with_children(|header| {
            let mut child_container = compose_header_vertical_container();
            header
                .spawn(child_container.clone())
                .with_children(|child| {
                    let (title, timer_placeholder) = compose_header_timer(&fonts);
                    child.spawn(title);
                    child
                        .spawn(timer_placeholder)
                        .insert(RefreshableUIElement)
                        .insert(HudUIElement)
                        .insert(TimerUIElement);
                });

            child_container.style.align_items = AlignItems::End;
            header.spawn(child_container).with_children(|child| {
                let (title, score) = compose_header_score(&fonts);
                child.spawn(title);
                child.spawn(score).insert(HudUIElement);
            });
        });
    });
}

fn compose_header() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    }
}

fn compose_header_vertical_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

fn compose_header_timer(fonts: &Res<FontAssetCollection>) -> (TextBundle, TextBundle) {
    let text_style = TextStyle {
        font: fonts.bold.clone(),
        font_size: 18.,
        color: Color::WHITE,
    };

    let title = TextBundle::from_section("TIME", text_style.clone());
    let time = TextBundle::from_section("00:00", text_style);
    (title, time)
}

fn compose_header_score(fonts: &Res<FontAssetCollection>) -> (TextBundle, TextBundle) {
    let text_style = TextStyle {
        font: fonts.bold.clone(),
        font_size: 18.,
        color: Color::WHITE,
    };

    let title = TextBundle::from_section("SCORE", text_style.clone());
    let score = TextBundle::from_section("------", text_style);
    (title, score)
}
