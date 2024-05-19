use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::menu::MenuAssetCollection;

const WIDTH: f32 = 35.;
const HEIGHT: f32 = 30.;


pub fn menu_button_system(
    assets: Res<MenuAssetCollection>,
    audio: Res<Audio>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for(interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => { 
                background_color.0 = Color::DARK_GRAY;
                audio.play(assets.press_sound.clone());
            },
            Interaction::Hovered => {
                background_color.0 = Color::GRAY; 
                audio.play(assets.hover_sound.clone());
            }
            Interaction::None => { 
                background_color.0 = Color::GRAY; 
            },
        }
    }
}


pub fn render_menu(mut commands: Commands, assets: Res<MenuAssetCollection>) {
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

    let button_container = NodeBundle {
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
        color: Color::GRAY
    };

    commands.spawn(container).with_children(|parent| {
        parent.spawn(title_container)
            .with_children(|title_container| {
                title_container.spawn(
                    TextBundle::from_section("Another", title_style.clone())
                        .with_style(Style {
                            margin: UiRect::right(Val::Px(75.)),
                            ..default()
                        })
                );
                title_container.spawn(
                    TextBundle::from_section("2D Platformer", title_style)
                        .with_style(Style {
                            margin: UiRect::left(Val::Px(75.)),
                            ..default()
                        })
                );
            });

        parent.spawn(button_container)
            .with_children(|button_container| {
                generate_menu_button(&assets, button_container, "PLAY");
                generate_menu_button(&assets, button_container, "OPTIONS");
                generate_menu_button(&assets, button_container, "QUIT");
            });
    });

    info_once!("Rendering main menu successful");
}

pub fn generate_menu_button(
    assets: &Res<MenuAssetCollection>,
    parent: &mut ChildBuilder,
    content: &str,
) {
    parent
        .spawn(generate_menu_button_container())
        .with_children(|btn_container| {
            btn_container.spawn(generate_menu_button_text(content, assets.regular.clone()));
        });
}

pub fn generate_menu_button_container() -> ButtonBundle {
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

pub fn generate_menu_button_text(content: &str, font: Handle<Font>) -> TextBundle {
    let text_style = TextStyle {
        font,
        color: Color::WHITE,
        font_size: 32.,
    };
    
    TextBundle::from_section(content, text_style)
        .with_text_justify(JustifyText::Center)
}

