mod characters;
mod map;

use crate::map::generate::{map_pixel_dimensions, setup_generator};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_procedural_tilemaps::prelude::{Cartesian3D, ProcGenSimplePlugin};
fn main() {
    // 获取地图的像素尺寸
    let map_size = map_pixel_dimensions();

    // 创建 Bevy 应用
    App::new()
        // 设置背景颜色为白色
        .insert_resource(ClearColor(Color::WHITE))
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
                        // 根据地图尺寸设置窗口大小
                        resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32),
                        // 禁止调整窗口大小
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                // 设置图像采样模式为最近邻（保持像素风格）
                .set(ImagePlugin::default_nearest()),
        )
        // 添加过程化地图生成插件
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        // 添加角色插件
        .add_plugins(characters::CharactersPlugin)
        // 在启动时设置相机和地图生成器
        .add_systems(Startup, (setup_camera, setup_generator))
        .run();
}

/// 设置游戏相机
fn setup_camera(mut commands: Commands) {
    // 生成 2D 相机
    commands.spawn(Camera2d);
}
