use bevy::{color::palettes::basic::*, prelude::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub trait NormalButton: Component + Sized {
    fn spawn_btn(
        self,
    ) -> (
        Button,
        Self,
        Node,
        BorderColor,
        BorderRadius,
        BackgroundColor,
    ) {
        (
            Button,
            self,
            Node {
                display: Display::Flex,
                width: Val::Auto,
                height: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        )
    }

    #[allow(clippy::type_complexity)]
    fn normal_button_update(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &mut BorderColor),
            (Changed<Interaction>, With<Button>, With<Self>),
        >,
    ) {
        interaction_query
            .iter_mut()
            .for_each(
                |(interaction, mut color, mut border_color)| match *interaction {
                    Interaction::Pressed => {
                        *color = PRESSED_BUTTON.into();
                        border_color.0 = RED.into();
                    }
                    Interaction::Hovered => {
                        *color = HOVERED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                    }
                },
            )
    }
}
