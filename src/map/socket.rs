use bevy_procedural_tilemaps::prelude::{Socket, SocketCollection};

/// 泥土层连接器
///
/// 定义泥土层的连接规则
pub struct DirtLayerSockets {
    /// 向上连接的连接器
    pub layer_up: Socket,
    /// 向下连接的连接器
    pub layer_down: Socket,
    /// 材质连接器
    pub material: Socket,
}

/// 草地层连接器
///
/// 定义绿色草地层的连接规则
pub struct GrassLayerSockets {
    /// 向上连接的连接器
    pub layer_up: Socket,
    /// 向下连接的连接器
    pub layer_down: Socket,
    /// 材质连接器
    pub material: Socket,
    /// 空地和草地的混合连接器
    pub void_and_grass: Socket,
    /// 草地和空地的混合连接器
    pub grass_and_void: Socket,
    /// 向上填充的连接器
    pub grass_fill_up: Socket,
}

/// 黄色草地层连接器
///
/// 定义黄色草地层的连接规则
pub struct YellowGrassLayerSockets {
    /// 向上连接的连接器
    pub layer_up: Socket,
    /// 向下连接的连接器
    pub layer_down: Socket,
    /// 向下填充的连接器
    pub yellow_grass_fill_down: Socket,
}

/// 水层连接器
///
/// 定义水层的连接规则
pub struct WaterLayerSockets {
    /// 向上连接的连接器
    pub layer_up: Socket,
    /// 向下连接的连接器
    pub layer_down: Socket,
    /// 材质连接器
    pub material: Socket,
    /// 空地和水的混合连接器
    pub void_and_water: Socket,
    /// 水和空地的混合连接器
    pub water_and_void: Socket,
    /// 向上地面连接器
    pub ground_up: Socket,
}

/// 道具层连接器
///
/// 定义树木、岩石等装饰物的连接规则
pub struct PropsLayerSockets {
    /// 向上连接的连接器
    pub layer_up: Socket,
    /// 向下连接的连接器
    pub layer_down: Socket,
    /// 向下道具连接器
    pub props_down: Socket,
    /// 大树-1 基座连接器
    pub big_tree_1_base: Socket,
    /// 大树-2 基座连接器
    pub big_tree_2_base: Socket,
}

/// 地形连接器集合
///
/// 包含所有地形层的连接器定义
pub struct TerrainSockets {
    /// 泥土层
    pub dirt: DirtLayerSockets,
    /// 空地连接器
    pub void: Socket,
    /// 草地层
    pub grass: GrassLayerSockets,
    /// 黄色草地层
    pub yellow_grass: YellowGrassLayerSockets,
    /// 水层
    pub water: WaterLayerSockets,
    /// 道具层
    pub props: PropsLayerSockets,
}

/// 创建地形连接器集合
///
/// 初始化所有地形层的连接器
///
/// # 参数
/// - `socket_collection`: 连接器集合
///
/// # 返回
/// 完整的地形连接器集合
pub fn create_sockets(socket_collection: &mut SocketCollection) -> TerrainSockets {
    // 创建新连接器的辅助函数
    let mut new_socket = || -> Socket { socket_collection.create() };

    let sockets = TerrainSockets {
        // 初始化泥土层连接器
        dirt: DirtLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
        },
        // 初始化空地连接器
        void: new_socket(),
        // 初始化草地层连接器
        grass: GrassLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
            void_and_grass: new_socket(),
            grass_and_void: new_socket(),
            grass_fill_up: new_socket(),
        },
        // 初始化黄色草地层连接器
        yellow_grass: YellowGrassLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            yellow_grass_fill_down: new_socket(),
        },
        // 初始化水层连接器
        water: WaterLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            material: new_socket(),
            void_and_water: new_socket(),
            water_and_void: new_socket(),
            ground_up: new_socket(),
        },
        // 初始化道具层连接器
        props: PropsLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            props_down: new_socket(),
            big_tree_1_base: new_socket(),
            big_tree_2_base: new_socket(),
        },
    };

    sockets
}
