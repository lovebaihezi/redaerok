use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(SubStates, Debug, Clone, Default)]
#[source(PageState = PageState::TxtReadPage)]
pub enum TxtReaderState {
    #[default]
    Welcome,
    WaitForUserSelecting,
    // Custom PartialEq and Eq to make sure it the String won't messed up with the State System management
    WaitForLoadingFile(String),
    PreDisplaying,
    Displaying,
}

impl PartialEq for TxtReaderState {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Welcome, Self::Welcome)
                | (Self::WaitForUserSelecting, Self::WaitForUserSelecting)
                | (Self::WaitForLoadingFile(_), Self::WaitForLoadingFile(_))
                | (Self::PreDisplaying, Self::PreDisplaying)
                | (Self::Displaying, Self::Displaying)
        )
    }
}

impl Hash for TxtReaderState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Welcome => 0.hash(state),
            Self::WaitForUserSelecting => 1.hash(state),
            Self::WaitForLoadingFile(_) => 2.hash(state),
            Self::PreDisplaying => 3.hash(state),
            Self::Displaying => 4.hash(state),
        }
    }
}

impl Eq for TxtReaderState {}

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
