use bevy::{color::palettes::basic::*, prelude::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub trait NormalButton
where
    Self: Component + Sized,
{
    fn outer_node(
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
                width: Val::Px(128.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        )
    }

    fn on_press(&mut self) {}

    fn on_hover(&mut self) {}

    fn none(&mut self) {}

    #[allow(clippy::type_complexity)]
    fn fixed_update(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &mut Self,
            ),
            (Changed<Interaction>, With<Button>, With<Self>),
        >,
    ) {
        for (interaction, mut color, mut border_color, mut component) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    component.on_press();
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = RED.into();
                }
                Interaction::Hovered => {
                    component.on_hover();
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    component.none();
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}
