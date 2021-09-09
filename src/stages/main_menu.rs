use bevy::prelude::*;
use bevy_spicy_aseprite::AsepriteImage;

use crate::{ui::create_nine_patch, utils::GetSubHandle, GameAssets};

use super::GameState;

pub struct MainMenuStagePlugin;

impl Plugin for MainMenuStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu));
        app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update_main_menu));
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(remove_main_menu));
    }
}

struct MainMenuScreenEntity(Entity);

fn create_main_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_assets: ResMut<Assets<Texture>>,
    asprite_assets: Res<Assets<AsepriteImage>>,
) {
    let ui_aseprite = if let Some(ui_aseprite) = asprite_assets.get(&game_assets.ui_sprites) {
        ui_aseprite
    } else {
        return;
    };

    let slices = ui_aseprite.aseprite().slices();

    let menu_nine_slice =
        if let Some(menu_nine_slice) = slices.get_by_name(&crate::ui_sprites::slices::Menu) {
            menu_nine_slice
        } else {
            error!("Could not find menu slice");
            return;
        };

    let mut loading_screen = commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent((100. - 65.) / 2.0),
                right: Val::Undefined,
                top: Val::Percent(50. - 10. / 2.),
                bottom: Val::Undefined,
            },
            size: Size {
                width: Val::Percent(65.),
                height: Val::Percent(50.),
            },
            ..Default::default()
        },
        // material: materials.add(ColorMaterial::texture(
        //     game_assets
        //         .ui_sprites
        //         .get_sub_handle("Slice/menu", &mut texture_assets),
        // )),
        ..Default::default()
    });
    let center_entity = create_nine_patch(
        &mut loading_screen,
        menu_nine_slice,
        game_assets.ui_sprites.clone(),
        &mut materials,
        &mut texture_assets,
    );
    let loading_screen = loading_screen.id();

    commands.insert_resource(MainMenuScreenEntity(loading_screen));
    debug!("Created main menu");
}

fn update_main_menu() {}

fn remove_main_menu(mut commands: Commands, ls_entity: Res<MainMenuScreenEntity>) {
    commands.entity(ls_entity.0).despawn_recursive();

    commands.remove_resource::<MainMenuScreenEntity>();
    debug!("Done with main menu!")
}
