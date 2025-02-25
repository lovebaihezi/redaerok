use bevy::prelude::*;

use super::normal_button::NormalButton;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ImportButtonStatus {
    #[default]
    None,
    Hover,
    Importing,
    ImportSuccess,
    ImportFailed,
}

impl std::fmt::Display for ImportButtonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Hover => "Hover",
                Self::Importing => "Importing",
                Self::ImportSuccess => "ImportSuccess",
                Self::ImportFailed => "ImportFailed",
            }
        )
    }
}

#[derive(Component, Default)]
pub struct ImportButton;

impl NormalButton for ImportButton {}
