# Socket连接规则详解

## 什么是Socket？

Socket（插座/连接器）是程序化地形生成系统中的核心概念，用于定义相邻地块（tile）之间的连接兼容性。

```
Socket的作用：
┌─────────────┐     ┌─────────────┐
│    Tile A   │     │    Tile B   │
│  ┌─────────┐ │     │ ┌─────────┐  │
│  │ Socket1 │ ├─────┤┤ Socket2 │  │ ← 这两个socket必须兼容才能连接
│  └─────────┘ │     │ └─────────┘  │
└─────────────┘     └─────────────┘
```

## Socket类型定义

从 `socket.rs` 可以看到，系统定义了以下Socket类型：

```rust
// 土壤层Socket
pub struct DirtLayerSockets {
    pub layer_up: Socket,    // 向上连接到草地层
    pub material: Socket,    // 土壤材质
    pub layer_down: Socket,  // 向下连接
}

// 草地层Socket
pub struct GrassLayerSockets {
    pub layer_up: Socket,          // 向上连接
    pub material: Socket,          // 草地材质
    pub layer_down: Socket,        // 向下连接到土壤层
    pub void_and_grass: Socket,    // 虚空→草地（边缘）
    pub grass_and_void: Socket,    // 草地→虚空（边缘）
    pub grass_fill_up: Socket,     // 填充向上
}

// 地形Socket集合
pub struct TerrainSockets {
    pub dirt: DirtLayerSockets,
    pub void: Socket,              // 空白区域
    pub grass: GrassLayerSockets,
}
```

## 连接规则解析

```rust
socket_collection.add_connections(vec![
    (terrain_sockets.void, vec![terrain_sockets.void]),
    (
        terrain_sockets.grass.material,
        vec![terrain_sockets.grass.material],
    ),
    (
        terrain_sockets.grass.void_and_grass,
        vec![terrain_sockets.grass.grass_and_void],
    ),
]);
```

### 规则1：虚空自我连接

```rust
(terrain_sockets.void, vec![terrain_sockets.void])
```

**含义**：虚空区域只能与虚空区域连接。

**应用场景**：
```
    [Void] ←─── 连接 ───→ [Void]
     空白               空白
    
✅ 可以连接：Void ↔ Void
❌ 不能连接：Void ↔ Grass
```

**作用**：确保空白区域不会与草地或土壤连接，形成清晰的边界。

---

### 规则2：草地材质自我连接

```rust
(terrain_sockets.grass.material, vec![terrain_sockets.grass.material])
```

**含义**：草地材质只能与草地材质连接。

**应用场景**：
```
    [Grass] ←─── 连接 ───→ [Grass]
     草地                草地
    
✅ 可以连接：Grass.material ↔ Grass.material
❌ 不能连接：Grass.material ↔ Void
❌ 不能连接：Grass.material ↔ Dirt
```

**作用**：确保草地区域内部可以无缝连接，形成连续的草地面。

---

### 规则3：边缘Socket互补连接

```rust
(terrain_sockets.grass.void_and_grass, vec![terrain_sockets.grass.grass_and_void])
```

**含义**：草地边缘（虚空→草地）必须与边缘（草地→虚空）互补连接。

**这是最重要的规则！**

#### 为什么需要两个方向？

想象草地的边缘：

```
        Void (虚空)
            ↑
            │ void_and_grass (从虚空看向草地)
            │
    ┌───────┴───────┐
    │   Grass       │
    │   (草地)       │
    └───────┬───────┘
            │ grass_and_void (从草地看向虚空)
            ↓
        Void (虚空)
```

**两个方向的作用**：

- `void_and_grass`：从虚空一侧看向草地（"我旁边是草地"）
- `grass_and_void`：从草地一侧看向虚空（"我旁边是虚空"）

**连接规则**：
```
   [Void]                    [Grass]
      │                        │
      │ void_and_grass         │ grass_and_void
      ├────────────────────────┤
              可以连接！
      
   [Grass]                   [Void]
      │                        │
      │ grass_and_void         │ void_and_grass
      ├────────────────────────┤
              可以连接！
```

**应用场景**：
```
草地平台的边缘：
    Void    Void    Void    Void
    ┌───────┬───────┬───────┐
    │       │ Grass │       │  ← 外角
    │       │ █████ │       │
    └───────┴───────┴───────┘
    
边缘格子的socket配置：
- 面向虚空的一侧：void_and_grass
- 面向草地的一侧：grass_and_void
```

## 实际应用示例

### 场景1：创建一个3x3的草地平台

```
Step 1: 定义每个格子的socket
┌─────────────────────────────────┐
│  Void   │   Void   │   Void     │
│   [ ]   │   [ ]    │   [ ]      │
├─────────┼─────────┼─────────────┤
│  Void   │  Grass   │   Void     │
│   [ ]   │  [████]  │   [ ]      │
├─────────┼─────────┼─────────────┤
│  Void   │   Void   │   Void     │
│   [ ]   │   [ ]    │   [ ]      │
└─────────┴─────────┴─────────────┘

Step 2: 应用连接规则
中央草地格子：
  x_pos: grass.material
  x_neg: grass.material
  y_pos: grass.material
  y_neg: grass.material
  
边缘草地格子（如果有）：
  面向虚空的一侧：void_and_grass
  面向草地的一侧：grass_and_void
```

### 场景2：外角格子的socket配置

回顾之前的外角代码：

```rust
let green_grass_corner_out = SocketsCartesian3D::Simple {
    x_pos: terrain_sockets.grass.void_and_grass,  // 右侧：虚空→草地
    x_neg: terrain_sockets.void,                 // 左侧：虚空
    z_pos: terrain_sockets.grass.layer_up,        // 上层：草地
    z_neg: terrain_sockets.grass.layer_down,      // 下层：草地
    y_pos: terrain_sockets.void,                 // 前侧：虚空
    y_neg: terrain_sockets.grass.grass_and_void, // 后侧：草地→虚空
}
```

**可视化**：
```
        Z+ (上)
        ↑
        │ grass
        │
   ┌────┴────┐
   │         │
   │  草地   │ ← 外角伸出来
   │         │
   └────┬────┘
        │ grass
        ↓
        Z- (下)

侧面视图 (从X+方向看):
        Void
          ↑
          │ void_and_grass
    ┌─────┴─────┐
    │  Grass    │
    └─────┬─────┘
          │ grass_and_void
          ↓
        Void

这个外角可以放在草地的右下角，因为：
- 它的x_pos是void_and_grass，可以连接到右边的grass_and_void
- 它的y_neg是grass_and_void，可以连接到后面的void_and_grass
```

## 连接规则的图解总结

```
连接规则网络：

Void ─────────────────────────── Void
 │                                │
 │ void_and_grass               grass_and_void
 │                                │
 └────────────────────────────────┘
                ↓
             Grass.material
                ↓
             Grass.material
                ↓
           ┌────────┐
           │ Grass  │
           │ 内部   │ ← 只能与material连接
           └────────┘

兼容性矩阵：
              │ Void │ Grass │ void_and_grass │ grass_and_void
──────────────┼──────┼───────┼────────────────┼────────────────
Void          │  ✓   │   ✗   │       ✗        │       ✗
Grass         │  ✗   │   ✓   │       ✗        │       ✗
void_and_grass│  ✗   │   ✗   │       ✗        │       ✓
grass_and_void│  ✗   │   ✗   │       ✓        │       ✗

✓ = 可以连接
✗ = 不能连接
```

## 为什么这些规则重要？

1. **自动约束地形生成**：算法会自动选择符合连接规则的模型，避免产生不连贯的地形
2. **确保边界清晰**：Void和Grass通过特殊的边缘socket连接，产生自然的边界效果
3. **支持复杂地形**：通过组合不同的socket配置，可以创建角落、边缘、岛屿等各种地形特征
4. **代码复用**：相同的socket配置可以用于多个位置，只需要旋转即可

## 完整的草地层模型总结

基于这些连接规则，草地层包含以下模型：

1. **Main Grass**（主草地）- 四周都是grass.material
2. **Void**（虚空）- 四周都是void
3. **Outer Corner**（外角）- 三面虚空，一面草地的边缘
4. **Inner Corner**（内角）- 草地内部的凹陷角
5. **Side Edge**（侧边）- 草地与虚空的直线边界

所有这些模型通过socket连接规则协同工作，自动生成连贯的地形！
