use crate::collision::tile_type::TileType;
use bevy::prelude::*;

/// 碰撞地图资源
///
/// 用于管理游戏中的瓦片碰撞检测，存储每个瓦片的碰撞类型和地图信息
#[derive(Resource)]
pub struct CollisionMap {
    /// 瓦片类型数组，地图瓦片展开成一维数组
    tiles: Vec<TileType>,
    /// 网格宽度（瓦片数量）
    width: i32,
    /// 网格高度（瓦片数量）
    height: i32,
    /// 每个瓦片在世界单位中的大小
    tile_size: f32,
    /// 网格原点的世界 X 坐标（左下角）
    origin_x: f32,
    /// 网格原点的世界 Y 坐标（左下角）
    origin_y: f32,
}

impl CollisionMap {
    /// 创建新的碰撞地图
    ///
    /// # 参数
    /// - `width`: 网格宽度（瓦片数量）
    /// - `height`: 网格高度（瓦片数量）
    /// - `tile_size`: 单个瓦片的世界单位大小
    /// - `origin_x`: 网格原点的 X 坐标
    /// - `origin_y`: 网格原点的 Y 坐标
    ///
    /// # 返回
    /// 新的碰撞地图实例
    pub fn new(width: i32, height: i32, tile_size: f32, origin_x: f32, origin_y: f32) -> Self {
        // 计算瓦片总数
        let size = (width * height) as usize;

        Self {
            // 初始化所有瓦片为空类型
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
            origin_x,
            origin_y,
        }
    }

    /// 将网格坐标转换为数组索引
    ///
    /// # 参数
    /// - `x`: 网格 X 坐标
    /// - `y`: 网格 Y 坐标
    ///
    /// # 返回
    /// 瓦片数组的索引
    #[inline]
    fn xy_to_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// 检查坐标是否在网格范围内
    ///
    /// # 参数
    /// - `x`: 网格 X 坐标
    /// - `y`: 网格 Y 坐标
    ///
    /// # 返回
    /// 如果坐标在范围内返回 true，否则返回 false
    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// 将世界坐标转换为网格坐标
    ///
    /// # 参数
    /// - `world_pos`: 世界坐标位置
    ///
    /// # 返回
    /// 对应的网格坐标
    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        let grid_x = ((world_pos.x - self.origin_x) / self.tile_size).floor() as i32;
        let grid_y = ((world_pos.y - self.origin_y) / self.tile_size).floor() as i32;
        IVec2::new(grid_x, grid_y)
    }

    /// 将网格坐标转换为世界坐标
    ///
    /// # 参数
    /// - `grid_x`: 网格 X 坐标
    /// - `grid_y`: 网格 Y 坐标
    ///
    /// # 返回
    /// 对应的世界坐标（瓦片中心点）
    #[allow(unused)]
    pub fn grid_to_world(&self, grid_x: i32, grid_y: i32) -> Vec2 {
        Vec2::new(
            // 计算瓦片中心点的 X 坐标
            self.origin_x + (grid_x as f32 + 0.5) * self.tile_size,
            // 计算瓦片中心点的 Y 坐标
            self.origin_y + (grid_y as f32 + 0.5) * self.tile_size,
        )
    }

    /// 获取指定位置的瓦片类型
    ///
    /// # 参数
    /// - `x`: 网格 X 坐标
    /// - `y`: 网格 Y 坐标
    ///
    /// # 返回
    /// 瓦片类型，如果坐标超出范围则返回 None
    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileType> {
        if self.in_bounds(x, y) {
            Some(self.tiles[self.xy_to_idx(x, y)])
        } else {
            None
        }
    }

    /// 设置指定位置的瓦片类型
    ///
    /// # 参数
    /// - `x`: 网格 X 坐标
    /// - `y`: 网格 Y 坐标
    /// - `tile`: 要设置的瓦片类型
    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        if self.in_bounds(x, y) {
            let idx = self.xy_to_idx(x, y);
            self.tiles[idx] = tile;
        }
    }

    /// 检查指定位置是否可行走
    ///
    /// # 参数
    /// - `x`: 网格 X 坐标
    /// - `y`: 网格 Y 坐标
    ///
    /// # 返回
    /// 如果可行走返回 true，否则返回 false
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).map_or(false, |tile| tile.is_walkable())
    }

    /// 检查世界坐标位置是否可行走
    ///
    /// # 参数
    /// - `world_pos`: 世界坐标位置
    ///
    /// # 返回
    /// 如果可行走返回 true，否则返回 false
    pub fn is_world_pos_walkable(&self, world_pos: Vec2) -> bool {
        let grid_pos = self.world_to_grid(world_pos);
        self.is_walkable(grid_pos.x, grid_pos.y)
    }

    /// 检查圆形是否与指定瓦片相交
    ///
    /// # 参数
    /// * `center` - 圆形中心点坐标
    /// * `radius` - 圆形半径
    /// * `gx` - 瓦片网格x坐标
    /// * `gy` - 瓦片网格y坐标
    ///
    /// # 返回值
    /// * `bool` - 如果圆形与瓦片相交则返回true，否则返回false
    fn circle_intersects_tile(&self, center: Vec2, radius: f32, gx: i32, gy: i32) -> bool {
        // 计算瓦片边界框的最小坐标点
        let tile_min = Vec2::new(
            self.origin_x + gx as f32 * self.tile_size,
            self.origin_y + gy as f32 * self.tile_size,
        );
        // 计算瓦片边界框的最大坐标点
        let tile_max = tile_min + Vec2::splat(self.tile_size);

        // 找到圆心在瓦片边界框内的最近点
        let closest = Vec2::new(
            center.x.clamp(tile_min.x, tile_max.x),
            center.y.clamp(tile_min.y, tile_max.y),
        );

        // 判断圆心到最近点的距离平方是否小于等于半径平方
        center.distance_squared(closest) <= radius * radius
    }

    /// 检查给定的圆形区域是否完全在当前对象的边界范围内
    ///
    /// # 参数
    /// * `center` - 圆形区域的中心点坐标
    /// * `radius` - 圆形区域的半径
    ///
    /// # 返回值
    /// 如果圆形区域完全在边界内则返回 true，否则返回 false
    fn is_within_bounds(&self, center: Vec2, radius: f32) -> bool {
        // 计算边界的左、右、下、上四个边界值
        let left = self.origin_x;
        let right = self.origin_x + self.width as f32 * self.tile_size;
        let bottom = self.origin_y;
        let top = self.origin_y + self.height as f32 * self.tile_size;

        // 检查圆形是否完全在边界内
        center.x - radius >= left
            && center.x + radius <= right
            && center.y - radius >= bottom
            && center.y + radius <= top
    }

    /// 检查圆形区域（玩家角色）是否清晰（不与任何不可行走的瓦片碰撞）
    ///
    /// # 参数
    /// - `center`: 圆形中心的世界坐标
    /// - `radius`: 圆形半径
    ///
    /// # 返回
    /// 如果圆形区域不与任何障碍物碰撞返回 true，否则返回 false
    pub fn is_circle_clear(&self, center: Vec2, radius: f32) -> bool {
        // 首先检查圆形是否在边界内
        if !self.is_within_bounds(center, radius) {
            return false;
        }

        // 如果半径为 0 或负数，只检查中心点
        if radius <= 0.0 {
            return self.is_world_pos_walkable(center);
        }

        // 找到可能与圆相交的网格单元
        let min_gx = ((center.x - radius - self.origin_x) / self.tile_size).floor() as i32;
        let max_gx = ((center.x + radius - self.origin_x) / self.tile_size).floor() as i32;
        let min_gy = ((center.y - radius - self.origin_y) / self.tile_size).floor() as i32;
        let max_gy = ((center.y + radius - self.origin_y) / self.tile_size).floor() as i32;

        // 遍历所有可能相交的瓦片
        for gy in min_gy..=max_gy {
            for gx in min_gx..=max_gx {
                // 检查瓦片是否在边界内
                if !self.in_bounds(gx, gy) {
                    return false;
                }

                // 获取瓦片类型
                if let Some(tile) = self.get_tile(gx, gy) {
                    // 如果瓦片不可行走，检查碰撞
                    if !tile.is_walkable() {
                        // 计算有效半径（考虑瓦片的碰撞调整值）
                        let effective_radius =
                            radius + tile.collision_adjustment() * self.tile_size;
                        // 检查圆形是否与瓦片相交
                        if self.circle_intersects_tile(center, effective_radius, gx, gy) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// 扫描圆形移动路径，检测并处理碰撞
    ///
    /// 此函数模拟角色从起点移动到终点的过程，使用"扫掠检测"（Sweep Test）方法。
    /// 当遇到障碍物时，角色会尝试沿着墙壁"滑动"，类似于玩家可以贴墙移动的效果。
    ///
    /// # 算法原理
    /// 1. 将移动路径分解为多个小步长，逐步检测碰撞
    /// 2. 如果直接移动会碰撞，尝试只沿 X 轴或 Y 轴移动（贴墙滑动）
    /// 3. 使用细分步长（瓦片大小的 1/4）确保碰撞检测的精确性
    ///
    /// # 参数
    /// - `start`: 起点坐标（圆形中心）
    /// - `end`: 目标终点坐标
    /// - `radius`: 圆形半径（角色碰撞体积）
    ///
    /// # 返回
    /// 实际可达的位置（可能在障碍物前停下，或沿着障碍物滑动后的位置）
    ///
    /// # 示例场景
    /// - 角色直线移动到目标：正常移动
    /// - 角色移动路径上有墙壁：会在墙前停下或贴墙滑动
    /// - 角色沿墙移动：可以沿着墙壁的方向继续前进
    pub fn sweep_circle(&self, start: Vec2, end: Vec2, radius: f32) -> Vec2 {
        // 计算从起点到终点的位移向量
        let delta = end - start;

        // 如果位移非常小（小于 0.001），直接返回起点，避免不必要的计算
        if delta.length() < 0.001 {
            return start;
        }

        // 设置最大步长为瓦片大小的 1/4
        // 这样可以确保碰撞检测足够精确，不会"穿透"薄墙
        let max_step = self.tile_size * 0.25;

        // 计算需要多少步才能从起点走到终点
        // 向上取整确保至少走 1 步，不会除以零
        let steps = (delta.length() / max_step).ceil().max(1.0) as i32;

        // 计算每一步的向量（将总位移平均分配到每一步）
        let step_vec = delta / steps as f32;

        // 初始化当前位置为起点
        let mut pos = start;

        // 逐步向前移动，每一步都进行碰撞检测
        for _ in 0..steps {
            // 计算候选位置（当前位置 + 步长向量）
            let candidate = pos + step_vec;

            // 检查候选位置是否没有碰撞
            if self.is_circle_clear(candidate, radius) {
                // 如果清晰，直接移动到候选位置
                pos = candidate;
            } else {
                // 如果直接移动会碰撞，尝试"贴墙滑动"策略
                // 首先尝试只沿 X 轴移动（保持 Y 坐标不变）
                let try_x = Vec2::new(candidate.x, pos.y);
                if self.is_circle_clear(try_x, radius) {
                    // 如果 X 轴方向可以移动，更新位置并继续下一步
                    pos = try_x;
                    continue;
                }

                // 如果 X 轴方向也不能移动，尝试只沿 Y 轴移动（保持 X 坐标不变）
                let try_y = Vec2::new(pos.x, candidate.y);
                if self.is_circle_clear(try_y, radius) {
                    // 如果 Y 轴方向可以移动，更新位置并继续下一步
                    pos = try_y;
                    continue;
                }

                // 如果 X 和 Y 轴方向都不能移动，说明碰到了墙角或障碍物
                // 此时保持当前位置不变（停在障碍物前）
                break;
            }
        }

        // 返回最终可达的位置
        pos
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    #[cfg(debug_assertions)]
    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }

    #[cfg(debug_assertions)]
    pub fn origin(&self) -> Vec2 {
        Vec2::new(self.origin_x, self.origin_y)
    }
}
