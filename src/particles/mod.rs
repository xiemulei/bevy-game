pub mod components;
pub mod material;
pub mod systems;

use crate::state::GameState;
use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
pub use material::*;
pub use systems::*;

/// 粒子插件 - 处理粒子效果系统
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing ParticlesPlugin");
        // 注册粒子材质插件
        app.add_plugins(Material2dPlugin::<ParticleMaterial>::default())
            // 注册粒子系统：更新发射器、更新粒子、清理完成发射器
            .add_systems(
                Update,
                (update_emitters, update_particles, cleanup_finished_emitters)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
        info!("ParticlesPlugin Initialized");
    }
}
