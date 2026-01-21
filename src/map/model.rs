use crate::map::assets::SpawnableAsset;
use bevy_procedural_tilemaps::prelude::{Cartesian3D, Model, ModelCollection, ModelTemplate};

/// 地形模型构建器
///
/// 用于构建和管理地形模型及其对应的资源
pub struct TerrainModelBuilder {
    /// 模型集合
    pub models: ModelCollection<Cartesian3D>,
    /// 每个模型对应的可生成资源列表
    pub assets: Vec<Vec<SpawnableAsset>>,
}

impl TerrainModelBuilder {
    /// 创建新的地形模型构建器
    pub fn new() -> Self {
        Self {
            models: ModelCollection::new(),
            assets: Vec::new(),
        }
    }

    /// 创建新模型
    ///
    /// # 参数
    /// - `template`: 模型模板
    /// - `assets`: 该模型对应的可生成资源列表
    ///
    /// # 返回
    /// 新创建模型的引用
    pub fn create_model<T>(
        &mut self,
        template: T,
        assets: Vec<SpawnableAsset>,
    ) -> &mut Model<Cartesian3D>
    where
        T: Into<ModelTemplate<Cartesian3D>>,
    {
        // 在模型集合中创建新模型
        let model_ref = self.models.create(template);
        // 保存对应的资源
        self.assets.push(assets);
        model_ref
    }

    /// 将构建器拆分为组件
    ///
    /// # 返回
    /// 包含资源列表和模型集合的元组
    pub fn into_parts(self) -> (Vec<Vec<SpawnableAsset>>, ModelCollection<Cartesian3D>) {
        (self.assets, self.models)
    }
}
