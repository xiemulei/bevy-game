use crate::characters::animation::{
    AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME,
};
use crate::characters::collider::Collider;
use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::characters::physics::Velocity;
use crate::characters::state::CharacterState;
use crate::collision::CollisionMap;
use crate::combat::PlayerCombat;
use crate::config::player::{PLAYER_SCALE, PLAYER_Z_POSITION};
use bevy::prelude::*;

/// 当前角色索引资源
///
/// 追踪当前选择的角色索引，用于切换角色
#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    /// 当前角色在列表中的索引
    pub index: usize,
}

/// 角色列表资源
///
/// 存储角色列表资源的句柄
#[derive(Resource)]
pub struct CharactersListResource {
    /// 角色列表资源的句柄
    pub handle: Handle<CharactersList>,
}

#[derive(Resource, Default, PartialEq, Eq)]
pub struct PlayerSpawned(pub bool);

/// 创建角色精灵图集布局
///
/// 根据角色配置创建纹理图集布局
///
/// # 参数
/// - `atlas_layouts`: 图集布局资源集合
/// - `character_entry`: 角色配置条目
///
/// # 返回
/// 图集布局的句柄
pub fn create_character_atlas_layout(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    character_entry: &CharacterEntry,
) -> Handle<TextureAtlasLayout> {
    // 计算动画所需的最大行号
    let max_row = character_entry.calculate_max_animation_row();

    // 创建网格布局
    atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ))
}

fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    let player_radius = 12.0;

    if collision_map.is_circle_clear(desired_pos, player_radius) {
        return desired_pos;
    }

    let grid_pos = collision_map.world_to_grid(desired_pos);
    if let Some(walkable) = collision_map.find_nearest_walkable(grid_pos) {
        let world_pos = collision_map.grid_to_world(walkable.x, walkable.y);
        info!(
            "Adjusted player spawn from {:?} to {:?} (was on obstacle)",
            desired_pos, world_pos
        );
        return world_pos;
    }

    warn!(
        "Could not find walkable spawn position near {:?}",
        desired_pos
    );
    desired_pos
}

pub fn load_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut character_index: ResMut<CurrentCharacterIndex>,
) {
    let characters_list_handle: Handle<CharactersList> =
        asset_server.load("characters/characters.ron");

    commands.insert_resource(CharactersListResource {
        handle: characters_list_handle,
    });
    character_index.index = 0;

    info!("Character assets loading stared");
}

pub fn spawn_player_at_valid_position(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_list: Res<Assets<CharactersList>>,
    character_index: Res<CurrentCharacterIndex>,
    characters_list_resource: Option<Res<CharactersListResource>>,
    collision_map: Option<Res<CollisionMap>>,
    mut player_spawned: ResMut<PlayerSpawned>,
) {
    let Some(collision_map) = collision_map else {
        return;
    };

    let Some(characters_list_resource) = characters_list_resource else {
        return;
    };

    let Some(characters_list) = characters_list.get(&characters_list_resource.handle) else {
        return;
    };

    if character_index.index >= characters_list.characters.len() {
        warn!("Invalid character index: {}", character_index.index);
        return;
    }

    let character_entry = &characters_list.characters[character_index.index];

    let desired_pos = Vec2::new(0.0, 0.0);
    let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);

    let texture = asset_server.load(&character_entry.texture_path);
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(valid_pos.x, valid_pos.y, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        sprite,
        AnimationController::default(),
        CharacterState::default(),
        Velocity::default(),
        Facing::default(),
        Collider::default(),
        PlayerCombat::default(),
        AnimationTimer(Timer::from_seconds(
            DEFAULT_ANIMATION_FRAME_TIME,
            TimerMode::Repeating,
        )),
        character_entry.clone(),
    ));

    player_spawned.0 = true;
    info!("Player spawned at validated position {:?}", valid_pos);
}

/// 切换角色
///
/// 按下数字键 1-9 切换到对应的角色
pub fn switch_character(
    input: Res<ButtonInput<KeyCode>>,
    mut character_index: ResMut<CurrentCharacterIndex>,
    characters_list: Res<Assets<CharactersList>>,
    characters_list_resource: Option<Res<CharactersListResource>>,
    mut query: Query<(&mut CharacterEntry, &mut Sprite), With<Player>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // 数字键到索引的映射
    const DIGIT_KEYS: [KeyCode; 9] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    // 检测按下的数字键
    let new_index = DIGIT_KEYS.iter().position(|&key| input.just_pressed(key));
    let Some(new_index) = new_index else {
        return;
    };
    // 等待角色列表资源可用
    let Some(characters_list_resource) = characters_list_resource else {
        return;
    };
    // 等待角色配置加载完成
    let Some(characters_list) = characters_list.get(&characters_list_resource.handle) else {
        return;
    };
    // 检查索引是否有效
    if new_index >= characters_list.characters.len() {
        return;
    }
    // 更新角色索引
    character_index.index = new_index;

    // 获取玩家实体的组件
    let Ok((mut current_entry, mut sprite)) = query.single_mut() else {
        return;
    };

    // 获取新角色的配置
    let character_entry = &characters_list.characters[new_index];
    // 更新角色配置
    *current_entry = character_entry.clone();

    // 加载新角色的纹理
    let texture = asset_server.load(&character_entry.texture_path);
    // 创建新的精灵图集布局
    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

    // 更新精灵
    *sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });
}
