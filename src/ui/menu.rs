use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::FontAssetCollection;
use crate::{
    game_state::{GameState, GameStateTransitionState},
    menu::MenuAssetCollection,
};

const WIDTH: f32 = 35.;
const HEIGHT: f32 = 30.;

#[derive(Component)]
pub struct MainMenuUiContainer;

#[derive(Component)]
pub enum MainMenuUiButton {
    Play,
    Options,
    Quit,
}

pub fn menu_button_system(
    assets: Res<MenuAssetCollection>,
    audio: Res<Audio>,
    mut next_game_transition_state: ResMut<NextState<GameStateTransitionState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MainMenuUiButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    info_once!("Main menu button system active");
    for (interaction, button_type, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::DARK_GRAY;
                audio.play(assets.press_sound.clone());

                match button_type {
                    MainMenuUiButton::Play => {
                        next_game_transition_state.set(GameStateTransitionState::AssetLoading);
                        next_game_state.set(GameState::InGame);
                    }
                    MainMenuUiButton::Options => todo!(),
                    MainMenuUiButton::Quit => todo!(),
                }
            }

            Interaction::Hovered => {
                background_color.0 = Color::GRAY;
                audio.play(assets.hover_sound.clone());
            }
            Interaction::None => {
                background_color.0 = Color::GRAY;
            }
        }
    }
}

pub fn render_menu_ui(mut commands: Commands, assets: Res<FontAssetCollection>) {
    info_once!("Rendering main menu");
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        background_color: BackgroundColor(Color::ALICE_BLUE),
        ..default()
    };

    let button_menu_container = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(3.),
            ..default()
        },
        ..default()
    };

    let title_container = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Percent(1.),
            ..default()
        },
        ..default()
    };

    let title_style = TextStyle {
        font: assets.mono.clone(),
        font_size: 64.,
        color: Color::GRAY,
    };

    commands
        .spawn(container)
        .insert(MainMenuUiContainer)
        .with_children(|parent| {
            parent
                .spawn(title_container)
                .with_children(|title_container| {
                    title_container.spawn(
                        TextBundle::from_section("Another", title_style.clone()).with_style(
                            Style {
                                margin: UiRect::right(Val::Px(75.)),
                                ..default()
                            },
                        ),
                    );
                    title_container.spawn(
                        TextBundle::from_section("2D Platformer", title_style).with_style(Style {
                            margin: UiRect::left(Val::Px(75.)),
                            ..default()
                        }),
                    );
                });

            parent
                .spawn(button_menu_container)
                .with_children(|btn_menu_container| {
                    let btn_container = compose_button_container();

                    btn_menu_container
                        .spawn(btn_container.clone())
                        .insert(MainMenuUiButton::Play)
                        .with_children(|bc| {
                            bc.spawn(compose_button_text("PLAY", &assets.regular));
                        });

                    btn_menu_container
                        .spawn(btn_container.clone())
                        .insert(MainMenuUiButton::Options)
                        .with_children(|bc| {
                            bc.spawn(compose_button_text("OPTIONS", &assets.regular));
                        });

                    btn_menu_container
                        .spawn(btn_container.clone())
                        .insert(MainMenuUiButton::Quit)
                        .with_children(|bc| {
                            bc.spawn(compose_button_text("QUIT", &assets.regular));
                        });
                });
        });
    info_once!("Rendering main menu successful");
}

pub fn cleanup_menu_ui(mut commands: Commands, ui_query: Query<Entity, With<MainMenuUiContainer>>) {
    info!("Cleaning up Menu UI");
    // should be only top level container
    for element_entity in ui_query.iter() {
        commands.entity(element_entity).despawn_recursive();
    }
    info!("Cleaning up Menu UI successful");
}

fn compose_button_container() -> ButtonBundle {
    let button_style = Style {
        width: Val::Percent(WIDTH),
        height: Val::Percent(HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(3.)),
        ..default()
    };

    let button_color = BackgroundColor(Color::GRAY);

    ButtonBundle {
        style: button_style,
        background_color: button_color,
        border_color: BorderColor(Color::DARK_GRAY),
        ..default()
    }
}

fn compose_button_text(content: &str, font: &Handle<Font>) -> TextBundle {
    let text_style = TextStyle {
        font: font.clone(),
        color: Color::WHITE,
        font_size: 32.,
    };
    TextBundle::from_section(content, text_style).with_text_justify(JustifyText::Center)
}
