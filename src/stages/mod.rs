use bevy::prelude::*;
use bevy_loading::LoadingPlugin;

use self::{loading::LoadingStagePlugin, main_menu::MainMenuStagePlugin};

mod loading;
mod main_menu;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    Running
}

#[derive(Debug, Default)]
pub struct StagesPlugin;

impl Plugin for StagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading);

        app.add_plugin(LoadingPlugin {
            loading_state: GameState::Loading,
            next_state: GameState::MainMenu,
        });

        app.add_plugin(LoadingStagePlugin);

        app.add_plugin(MainMenuStagePlugin);
    }
}
