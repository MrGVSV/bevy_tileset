# bevy_tileset

[![crates.io](https://img.shields.io/crates/v/bevy_tileset?style=flat-square)](https://crates.io/crates/bevy_tileset)
[![docs.rs](https://img.shields.io/docsrs/bevy_tileset?style=flat-square)](https://docs.rs/bevy_tileset)

Simple, configurable tilesets in Bevy using RON.

<p align="center">
	<img alt="Smart tile placement" src="https://raw.githubusercontent.com/MrGVSV/bevy_tileset/b81d2d7483785e5aa58ef0b449482d9d57bca3be/screenshots/tile_placement_demo.gif" />
</p>

> All GIFs generated with the [`bevy_tileset_map`](https://github.com/MrGVSV/bevy_tileset_map) crate

## 📋 Features

* Define tilesets and tiles via [RON](https://github.com/ron-rs/ron) files
* Load a tileset directly as a Bevy asset
* Define Standard, Animated, Variant, and Auto tiles

## 📲 Installation

Add one of the following lines to your `Cargo.toml`.

```toml
[dependencies]
bevy_tileset_tiles = "0.7" # For the base tile definitions
bevy_tileset = "0.7" # For general tileset usage (includes above)
```

## ✨ Usage

Simply **define** your tiles and tilesets in config files:

```rust
// assets/tiles/my_tile.ron
(
name: "My Tile",
tile: Standard("textures/my_tile.png")
)
```

```rust
// assets/my_tileset.ron
(
name: Some("My Awesome Tileset"),
id: 0,
tiles: {
0: "../tiles/my_tile.ron",
// ...
}
)
```

And **load** it in via a system:

```rust
use bevy::prelude::*;
use bevy_tileset::prelude::*;

fn load_tiles(asset_server: Res<AssetServer>) {
    let handle: Handle<Tileset> = asset_server.load("my_tileset.ron");
    // Store handle...
}
```

Then **access** the generated tileset from anywhere:

```rust
fn my_system(tilesets: Tilesets, /* other system params */) {
    let tileset = tilesets.get_by_name("My Awesome Tileset").unwrap();
    let tile_index = tileset.get_tile_index("My Tile").unwrap();

    match tile_index {
        TileIndex::Standard(texture_index) => { /* Do something */ }
        TileIndex::Animated(start, end, speed) => { /* Do something */ }
    }
}
```

## Tile Types

Currently there are four main tile types:

### 🖼 Standard

Defines a basic tile.

```rust
// assets/tiles/my-tile.ron

(
name: "My Tile",
tile: Standard("textures/my_tile.png")
)
```

### 🎞️ Animated

Defines an animated tile that can be generated with the `GPUAnimated` component from `bevy_ecs_tilemap`.

```rust
// assets/tiles/my-animated-tile.ron

(
name: "My Animated Tile",
tile: Animated((
speed: 2.25,
frames: [
"textures/animated-001.png",
"textures/animated-002.png",
"textures/animated-003.png",
]
))
)
```

### 🎲 Variant

> With the `variants` feature enabled

Defines a tile that has a set of possible variants. A variant is chosen at random when placed. These variants can either
be Standard or Animated.

```rust
// assets/tiles/my-variant-tile.ron

(
name: "My Crazy Random Tile",
tile: Variant([
(
weight: 1.0,
tile: Standard("textures/variant-standard-001.png")
),
(
// Default weight: 1.0
tile: Standard("textures/variant-standard-002.png")
),
(
weight: 0.0001, // Wow that's rare!
tile: Animated((
// Default speed: 1.0
frames: [
"textures/variant-animated-001.png",
"textures/variant-animated-002.png",
"textures/variant-animated-003.png",
]
))
)
])
)
```

### 🧠 Auto

> With the `auto-tile` feature enabled

Defines a tile that automatically chooses its active tile based on its neighbors. This behavior can be controlled with
rules. These sub-tiles are themselves Variant tiles.

```rust
// assets/tiles/my-auto-tile.ron

#![enable(implicit_some)]

(
name: "My Auto Tile",
tile: Auto([
(
rule: (
north: true,
east: false,
west: true,
),
variants: [
(
tile: Standard("textures/n_w-e-001.png")
),
(
weight: 2.0,
tile: Standard("textures/n_w-e-002.png")
)
]
),
(
rule: (
// Also supports short notation
n: false,
s: false,
// And ordinal directions
south_west: true,
nw: false
),
variants: [
(
tile: Standard("textures/sw-n_s_nw.png")
)
]
),
])
)
```

<p align="center">
	<img alt="Auto tiling" src="https://github.com/MrGVSV/bevy_tileset/blob/b81d2d7483785e5aa58ef0b449482d9d57bca3be/screenshots/auto_tiling_demo.gif" />
</p>

## 🎓 Examples

* [tileset](examples/tileset.rs) - Simply load and display a tileset
* [dynamic](examples/dynamic.rs) - Dynamically create a tileset at runtime

Also, be sure to check out the [assets](/assets/) folder for how to define a tile or tileset.

## 🌱 Areas of Growth

There are some things this crate could do better in. Here's a list of potential areas to grow:

- [x] Tileset
    - [x] Config files ★
- [ ] Improved Auto Tiles
    - [ ] Mirror/Rotation (designate a rule to be mirrored or rotated)
- [x] Loading
    - [x] Load configs as assets

As well as just an overall improved and cleaner API.

## 🎵 Important Note

These tiles are defined with the [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap) crate in mind.
Therefore, it's meant to work with an index-based tile system (where a tile's texture is defined as an index into a
texture atlas). Other solutions may need to be adapted in order to work with this crate.

## 🕊 Bevy Compatibility

| bevy | bevy_tileset |
|------|--------------|
| 0.10 | 0.7          |
| 0.9  | 0.6          |
| 0.8  | 0.5          |
| 0.7  | 0.4          |
| 0.6  | 0.3          |
| 0.5  | 0.2          |

