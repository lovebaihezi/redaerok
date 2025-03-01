use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TxtReaderState {
    None,
    WaitForRFD,
    WaitForUserSelecting,
    WaitForLoadingFile(String),
    WaitForParagraphParsing,
    PreDisplaying,
    Displaying,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource)]
pub enum PageState {
    WelcomePage, // Root
    TxtReadPage(TxtReaderState),
    AIChatPage,
}

impl PageState {
    pub fn welcome_page() -> Self {
        Self::WelcomePage
    }
    pub fn txt_read_page() -> Self {
        Self::TxtReadPage(TxtReaderState::None)
    }
    pub fn ai_chat_page() -> Self {
        Self::AIChatPage
    }
}
