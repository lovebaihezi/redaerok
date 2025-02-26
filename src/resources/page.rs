use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Resource)]
pub enum PageState {
    WelcomePage, // Root
    TxtReadPage,
    AIChatPage,
}
