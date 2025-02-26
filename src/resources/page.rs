use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Component)]
pub enum PageState {
    WelcomePage, // Root
    TxtReadPage,
}

pub fn on_page_switch() {}
