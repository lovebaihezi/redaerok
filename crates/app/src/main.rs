use bevy::app::AppExit;
use mimalloc::MiMalloc;
use redaerok_app::app::{AppType, Game};

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
