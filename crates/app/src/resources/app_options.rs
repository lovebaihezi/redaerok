use bevy::prelude::Resource;
use clap::Parser;

#[derive(Debug, Clone, Parser, Resource)]
#[command(version, about, long_about = None)]
pub struct AppOptions {
    txt_location: Option<String>,
}

impl AppOptions {
    pub fn txt_location(&self) -> Option<&str> {
        self.txt_location.as_deref()
    }
}
