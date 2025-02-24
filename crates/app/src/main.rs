use bevy::app::AppExit;

#[cfg(not(target_arch = "wasm32"))]
use mimalloc::MiMalloc;

use redaerok_app::app::{AppType, Game};

#[cfg(not(target_arch = "wasm32"))]
#[global_allocator]
static MIMALLOC_ALLOCATOR: MiMalloc = MiMalloc;

fn main() {
    let game = Game::init(AppType::Normal);
    let exit = game.run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
