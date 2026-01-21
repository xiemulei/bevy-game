use bevy::prelude::*;

/// 角色朝向枚举
///
/// 定义角色在 2D 平面上的四个朝向
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Facing {
    /// 上
    Up,
    /// 左
    Left,
    /// 下
    #[default]
    Down,
    /// 右
    Right,
}

impl Facing {
    /// 根据移动方向向量确定角色朝向
    ///
    /// 比较水平和垂直分量的大小，选择较大的一侧作为朝向
    pub fn from_velocity(velocity: Vec2) -> Self {
        if velocity.x.abs() > velocity.y.abs() {
            if velocity.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            if velocity.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        }
    }

    /// 获取朝向对应的索引
    ///
    /// 上: 0, 左: 1, 下: 2, 右: 3
    pub(crate) fn direction_index(&self) -> usize {
        match self {
            Facing::Up => 0,
            Facing::Left => 1,
            Facing::Down => 2,
            Facing::Right => 3,
        }
    }
}
