use bevy::prelude::*;

use crate::game_state::GameState;
use super::FontAssetCollection;

pub fn initialize_hud_systems(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), render_hud_ui);
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
            let child_contaienr = compose_header_vertical_container();
            header
                .spawn(child_contaienr.clone())
                .with_children(|child| {
                    let (title, timer_placeholder) = compose_header_timer(&fonts);
                    child.spawn(title);
                    child.spawn(timer_placeholder);
                });

            header.spawn(child_contaienr).with_children(|child| {
                let (title, score) = compose_header_score(&fonts);
                child.spawn(title);
                child.spawn(score);
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
        font: fonts.regular.clone(),
        font_size: 18.,
        color: Color::WHITE,
    };

    let title = TextBundle::from_section("TIME", text_style.clone());
    let time = TextBundle::from_section("00:00", text_style);
    (title, time)
}

fn compose_header_score(fonts: &Res<FontAssetCollection>) -> (TextBundle, TextBundle) {
    let text_style = TextStyle {
        font: fonts.regular.clone(),
        font_size: 18.,
        color: Color::WHITE,
    };

    let title = TextBundle::from_section("SCORE", text_style.clone());
    let score = TextBundle::from_section("------", text_style);
    (title, score)
}
