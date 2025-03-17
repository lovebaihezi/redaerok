use bevy::prelude::*;

#[derive(SubStates, Debug, Clone, Default, Hash, PartialEq, Eq)]
#[source(Page = Page::TxtViewer)]
pub enum TxtReaderState {
    #[default]
    Welcome,
    WaitForLoadingFile,
    PreDisplaying,
    Displaying,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Page {
    #[default]
    Welcome, // Root
    TxtViewer,
    AIChat,
}
