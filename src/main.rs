mod camera;
mod characters;
mod collision;
mod combat;
mod config;
mod inventory;
mod map;
mod particles;
mod state;

use crate::map::generate::setup_generator;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_procedural_tilemaps::prelude::{Cartesian3D, ProcGenSimplePlugin};

fn main() {
    App::new()
        // 设置背景颜色为白色
        .insert_resource(ClearColor(Color::BLACK))
        // 添加默认插件
        .add_plugins(
            DefaultPlugins
                // 设置资源文件路径
                .set(AssetPlugin {
                    file_path: "src/assets".to_string(),
                    ..default()
                })
                // 配置主窗口
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                // 设置图像采样模式为最近邻（保持像素风格）
                .set(ImagePlugin::default_nearest()),
        )
        // 添加过程化地图生成插件
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_plugins(state::StatePlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(collision::CollisionPlugin)
        // 添加角色插件
        .add_plugins(characters::CharactersPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(particles::ParticlesPlugin)
        // 在启动时设置相机和地图生成器
        .add_systems(Startup, setup_generator)
        .run();
}
