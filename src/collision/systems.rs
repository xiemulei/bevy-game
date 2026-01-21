use crate::collision::map::CollisionMap;
use crate::collision::tile_type::{TileMarker, TileType};
use crate::config::map::{GRID_X, GRID_Y, TILE_SIZE};
use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// 碰撞地图构建完成标志资源
///
/// 用于追踪碰撞地图是否已经构建完成，避免重复构建
#[derive(Resource, Default, Eq, PartialEq)]
pub struct CollisionMapBuilt(pub bool);

/// 构建碰撞地图系统
///
/// 从场景中的瓦片实体收集数据，构建用于碰撞检测的地图。
/// 系统会扫描所有带有 TileMarker 的实体，根据它们的位置和类型创建碰撞地图。
///
/// # 处理流程
/// 1. 遍历所有瓦片实体，收集位置和类型信息
/// 2. 对于同一网格位置的多个瓦片，只保留 Z 轴最高的（最上层）
/// 3. 计算实际地图边界
/// 4. 创建 CollisionMap 并填充瓦片类型
/// 5. 将水边缘转换为海岸类型
/// 6. 插入 CollisionMap 资源并标记构建完成
///
/// # 参数
/// - `commands`: Bevy 命令队列，用于插入资源
/// - `built`: 碰撞地图构建状态资源
/// - `tile_query`: 查询所有瓦片实体（TileMarker 和 Transform 组件）
pub fn build_collision_map(
    mut commands: Commands,
    mut built: ResMut<CollisionMapBuilt>,
    tile_query: Query<(&TileMarker, &Transform)>,
) {
    let mut tile_iter = tile_query.iter();
    // 如果场景中没有瓦片实体，直接返回
    let Some((first_marker, first_transform)) = tile_iter.next() else {
        return;
    };

    // 计算网格原点坐标（地图中心）
    let grid_origin_x = -TILE_SIZE * GRID_X as f32 / 2.0;
    let grid_origin_y = -TILE_SIZE * GRID_Y as f32 / 2.0;

    // 初始化地图边界变量
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    // 使用 HashMap 追踪每个网格位置的最高层瓦片
    // Key: (网格X, 网格Y), Value: (瓦片类型, Z轴高度)
    let mut layer_tracker: HashMap<(i32, i32), (TileType, f32)> = HashMap::new();
    let mut tile_count: usize = 0;

    // 定义处理单个瓦片的闭包函数
    let mut process_tile = |marker: &TileMarker, transform: &Transform| {
        // 统计瓦片数量
        tile_count += 1;

        // 获取瓦片的世界坐标
        let world_x = transform.translation.x;
        let world_y = transform.translation.y;
        let world_z = transform.translation.z;

        // 将世界坐标转换为网格坐标
        let grid_x = ((world_x - grid_origin_x) / TILE_SIZE).floor() as i32;
        let grid_y = ((world_y - grid_origin_y) / TILE_SIZE).floor() as i32;

        // 更新地图边界
        min_x = min_x.min(grid_x);
        max_x = max_x.max(grid_x);
        min_y = min_y.min(grid_y);
        max_y = max_y.max(grid_y);

        // 处理同一位置的多个瓦片（多层结构）
        // 只保留 Z 轴最高的瓦片（最上层）
        match layer_tracker.entry((grid_x, grid_y)) {
            Entry::Occupied(mut entry) => {
                // 如果当前瓦片比已存在的瓦片更高，替换它
                if world_z > entry.get().1 {
                    *entry.get_mut() = (marker.tile_type, world_z);
                }
            }
            Entry::Vacant(entry) => {
                // 该位置还没有瓦片，直接插入
                entry.insert((marker.tile_type, world_z));
            }
        }
    };

    // 处理第一个瓦片
    process_tile(first_marker, first_transform);
    // 处理剩余的所有瓦片
    for (marker, transform) in tile_iter {
        process_tile(marker, transform);
    }

    // 计算实际地图尺寸
    let actual_width = max_x - min_x + 1;
    let actual_height = max_y - min_y + 1;

    // 创建碰撞地图实例
    let mut map = CollisionMap::new(
        actual_width,
        actual_height,
        TILE_SIZE,
        grid_origin_x,
        grid_origin_y,
    );

    // 将收集到的瓦片数据填充到碰撞地图中
    // 注意：需要将绝对网格坐标转换为相对坐标（0-based）
    for ((grid_x, grid_y), (tile_type, _z)) in layer_tracker.iter() {
        let local_x = grid_x - min_x;
        let local_y = grid_y - min_y;
        map.set_tile(local_x, local_y, *tile_type);
    }

    // 将水的边缘瓦片转换为海岸类型
    convert_water_edges_to_shore(&mut map);

    // 将碰撞地图插入为资源
    commands.insert_resource(map);
    // 标记碰撞地图已构建完成
    built.0 = true;
}

/// 将水边缘转换为海岸类型
///
/// 遍历地图中的所有水域瓦片，检查其 8 个方向的邻居。
/// 如果水瓦片的任何邻居是可行走的（陆地），则将该水瓦片标记为海岸。
///
/// # 处理逻辑
/// 1. 遍历地图中每个位置
/// 2. 如果当前位置是水域，检查其 8 个方向的邻居
/// 3. 如果有任何邻居是可行走的（非水域），则该水瓦片是海岸
/// 4. 将所有海岸瓦片的类型从 Water 改为 Shore
///
/// # 作用
/// - 区分深水和浅水/海岸
/// - 角色可以在海岸上行走（如果 Shore 被标记为可行走）
/// - 提供更精确的碰撞检测
///
/// # 参数
/// - `map`: 碰撞地图的可变引用
fn convert_water_edges_to_shore(map: &mut CollisionMap) {
    let mut shores = Vec::new();

    // 遍历地图的每一行
    for y in 0..map.height() {
        // 遍历地图的每一列
        for x in 0..map.width() {
            // 跳过非水域瓦片
            if map.get_tile(x, y) != Some(TileType::Water) {
                continue;
            }

            // 检查 8 个方向的邻居（水平和垂直方向 + 对角线方向）
            let neighbors = [
                (x - 1, y),     // 左
                (x + 1, y),     // 右
                (x, y - 1),     // 下
                (x, y + 1),     // 上
                (x - 1, y - 1), // 左下
                (x + 1, y - 1), // 右下
                (x - 1, y + 1), // 左上
                (x + 1, y + 1), // 右上
            ];

            // 检查所有邻居
            for (nx, ny) in neighbors {
                // 如果邻居是可行走的（陆地），则当前水瓦片是海岸
                if map.is_walkable(nx, ny) {
                    shores.push((x, y));
                    break;
                }
            }
        }
    }

    // 将识别出的海岸瓦片类型设置为 Shore
    for (x, y) in shores {
        map.set_tile(x, y, TileType::Shore);
    }
}
