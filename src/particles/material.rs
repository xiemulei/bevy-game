use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, ColorWrites,
    RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dKey};

/// 粒子材质 - 使用自定义着色器实现发光效果
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ParticleMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl ParticleMaterial {
    /// 创建新的粒子材质
    pub fn new(color: Color) -> Self {
        Self {
            color: color.to_linear(),
        }
    }
}

impl Material2d for ParticleMaterial {
    /// 使用自定义粒子着色器
    fn fragment_shader() -> ShaderRef {
        "shaders/particle_glow.wgsl".into()
    }

    /// 使用混合模式实现透明效果
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    /// 自定义渲染管线配置 - 设置加法混合实现发光效果
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target) = fragment.targets.first_mut() {
                if let Some(target_state) = target.as_mut() {
                    // 设置加法混合模式实现发光效果
                    target_state.blend = Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    });
                    target_state.write_mask = ColorWrites::ALL;
                }
            }
        }
        Ok(())
    }
}
