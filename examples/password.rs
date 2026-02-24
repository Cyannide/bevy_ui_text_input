//! minimal password input example
use bevy::{color::palettes::css::NAVY, prelude::*};
use bevy_ui_text_input::{
    SubmitText, TextInputMode, TextInputNode, TextInputPlugin, TextInputPrompt,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            column_gap: Val::Px(20.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    TextInputNode {
                        mode: TextInputMode::SingleLine,
                        max_chars: Some(20),
                        clear_on_submit: true,
                        ..Default::default()
                    },
                    TextFont {
                        font: assets.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.,
                        ..Default::default()
                    },
                    TextInputPrompt {
                        text: "Username".to_string(),
                        color: Some(Color::srgb(0.3, 0.3, 0.3)),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    BackgroundColor(NAVY.into()),
                ))
                .observe(|event: On<SubmitText>, mut query: Query<&mut Text>| {
                    for mut text in query.iter_mut() {
                        text.0 = event.text.clone();
                    }
                });
            parent
                .spawn((
                    TextInputNode {
                        mode: TextInputMode::SingleLine,
                        max_chars: Some(20),
                        clear_on_submit: true,
                        mask_character: Some('*'),
                        ..Default::default()
                    },
                    TextFont {
                        font: assets.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.,
                        ..Default::default()
                    },
                    TextInputPrompt {
                        text: "Password".to_string(),
                        color: Some(Color::srgb(0.3, 0.3, 0.3)),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    BackgroundColor(NAVY.into()),
                ))
                .observe(|event: On<SubmitText>, mut query: Query<&mut Text>| {
                    for mut text in query.iter_mut() {
                        text.0 = event.text.clone();
                    }
                });
            parent.spawn(Text::new("Login..."));
        });
}
