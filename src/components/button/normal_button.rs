use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

pub trait NormalButton: Component + Sized {
    fn spawn_btn(self) -> impl Bundle {
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
            BackgroundColor(Color::WHITE),
        )
    }

    #[allow(clippy::type_complexity)]
    fn normal_button_update(
        mut command: Commands,
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
            ),
            (Changed<Interaction>, With<Button>, With<Self>),
        >,
        window: Single<Entity, With<Window>>,
        mut text_color_query: Query<&mut TextColor>,
    ) {
        interaction_query.iter_mut().for_each(
            |(interaction, mut bg_color, mut border_color, children)| {
                let pointer: CursorIcon = SystemCursorIcon::Pointer.into();
                let normal: CursorIcon = SystemCursorIcon::Default.into();
                match *interaction {
                    Interaction::Pressed => {
                        command
                            .entity(*window)
                            .remove::<CursorIcon>()
                            .insert(normal);
                        *bg_color = Color::BLACK.into();
                        border_color.0 = Color::WHITE;
                        if let Ok(mut text_color) = text_color_query.get_mut(children[0]) {
                            **text_color = Color::WHITE;
                        }
                    }
                    Interaction::Hovered => {
                        command
                            .entity(*window)
                            .remove::<CursorIcon>()
                            .insert(pointer);
                        *bg_color = Color::WHITE.into();
                        border_color.0 = Color::BLACK;
                        if let Ok(mut text_color) = text_color_query.get_mut(children[0]) {
                            **text_color = Color::BLACK;
                        }
                    }
                    Interaction::None => {
                        command
                            .entity(*window)
                            .remove::<CursorIcon>()
                            .insert(normal);
                        *bg_color = Color::BLACK.into();
                        border_color.0 = Color::WHITE;
                        if let Ok(mut text_color) = text_color_query.get_mut(children[0]) {
                            **text_color = Color::WHITE;
                        }
                    }
                }
            },
        )
    }
}
