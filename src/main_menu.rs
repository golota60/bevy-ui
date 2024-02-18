use bevy::prelude::*;

use crate::AppState;

const STATIC_BUTTON_INNER: Color = Color::rgb(0.5, 0.5, 0.5);
const STATIC_BUTTON_OUTER: Color = Color::rgb(0.2, 0.2, 0.2);

const HOVER_BUTTON_INNER: Color = Color::rgb(0.5, 0.5, 1.0);
const HOVER_BUTTON_OUTER: Color = Color::rgb(0.4, 0.4, 0.2);

const PRESSED_BUTTON_INNER: Color = Color::rgb(0.3, 0.3, 0.5);
const PRESSED_BUTTON_OUTER: Color = Color::rgb(0.6, 0.6, 0.2);

// Assumption: Each button is going to have only one section of text
// Texts are also treated as IDs
struct ButtonTexts;

impl ButtonTexts {
    pub const START: &'static str = "Start";
    pub const CONTINUE: &'static str = "Continue";
    pub const OPTIONS: &'static str = "Options";
    pub const QUIT: &'static str = "Quit";
}

fn create_ui_button(text: String) -> impl FnOnce(&mut ChildBuilder) {
    |parent| {
        // Spawn the actual UI inside the node
        parent
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect {
                        left: Val::Px(5.0),
                        right: Val::Px(5.0),
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    margin: UiRect {
                        left: Val::Px(5.0),
                        right: Val::Px(5.0),
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    ..default()
                },
                background_color: BackgroundColor(STATIC_BUTTON_INNER),
                border_color: BorderColor(STATIC_BUTTON_OUTER),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: text,
                            ..default()
                        }],
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

pub fn setup_menu(mut commands: Commands) {
    println!("Setting up main_menu");
    commands.spawn(Camera2dBundle::default());

    // Main UI Node to house all the UI elements
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.5, 0.2, 0.2)),
            ..default()
        })
        .with_children(create_ui_button(String::from(ButtonTexts::START)))
        .with_children(create_ui_button(String::from(ButtonTexts::CONTINUE)))
        .with_children(create_ui_button(String::from(ButtonTexts::OPTIONS)))
        .with_children(create_ui_button(String::from(ButtonTexts::QUIT)));
}

pub fn update_menu(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&mut Text>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for (inter, mut bg_color, mut border_color, children) in &mut query {
        let is_released = mouse.just_released(MouseButton::Left);
        let current_child = children.first().unwrap();
        let text = text_query.get(current_child.to_owned()).unwrap();
        let text = text.sections.first().unwrap();
        let text = text.value.as_str();

        println!(
            "mouse: {:?}, inter: {:?}, child: {:?}, text_q: {:?}, is_released: {:?}",
            is_released, inter, current_child, text, is_released
        );
        // Change UI
        match inter {
            Interaction::Hovered => {
                bg_color.0 = HOVER_BUTTON_INNER;
                border_color.0 = HOVER_BUTTON_OUTER;

                // Click handler lies in Hover, not press, as we want the action to be taken right after we release the mouse button
                if is_released {
                    match text {
                        ButtonTexts::START => next_app_state.set(AppState::InGame),
                        ButtonTexts::CONTINUE => {}
                        ButtonTexts::OPTIONS => {}
                        ButtonTexts::QUIT => {
                            app_exit_events.send(bevy::app::AppExit);
                        }
                        _ => {
                            panic!("Unreachable UI state.")
                        }
                    }
                }
            }
            Interaction::Pressed => {
                bg_color.0 = PRESSED_BUTTON_INNER;
                border_color.0 = PRESSED_BUTTON_OUTER;
            }
            Interaction::None => {
                bg_color.0 = STATIC_BUTTON_INNER;
                border_color.0 = STATIC_BUTTON_OUTER;
            }
        }
    }
}
