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

impl PageState {
    pub fn welcome_page() -> Self {
        Self::WelcomePage
    }
    pub fn txt_read_page() -> Self {
        Self::TxtReadPage
    }
    pub fn ai_chat_page() -> Self {
        Self::AIChatPage
    }
}
