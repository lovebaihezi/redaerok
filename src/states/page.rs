use bevy::prelude::*;

#[derive(SubStates, Debug, Clone, Default, Hash, PartialEq, Eq)]
#[source(PageState = PageState::TxtReadPage)]
pub enum TxtReaderState {
    #[default]
    Welcome,
    WaitForLoadingFile,
    PreDisplaying,
    Displaying,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PageState {
    #[default]
    WelcomePage, // Root
    TxtReadPage,
    AIChatPage,
}
