# bevy_tileset_map

An implementation of  [`bevy_tileset`](https://github.com/MrGVSV/bevy_tileset) for
the [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap) crate.

<p align="center">
	<img alt="Smart tile placement" src="https://github.com/MrGVSV/bevy_tileset/blob/770b45653fc8272921c1401d73f048406f3e2618/screenshots/tile_placement_demo.gif" />
</p>

## 📋 Features

All features from `bevy_tileset`, including:

- Define tilesets and tiles via [RON](https://github.com/ron-rs/ron) files
- Load a tileset directly as a Bevy asset
- Define Standard, Animated, Variant, and Auto tiles

As well as features specific to this crate:

* Super easy serialization and deserialization
* Auto tiling support
* Compact tile placement/removal

## 📲 Installation

This crate is still not publicly released yet as I might want to make a PR to try and integrate it directly
with `bevy_ecs_tilemap`, but for now you can use it with git:

```toml
[dependencies]
bevy_tileset_map = { git = "https://github.com/MrGVSV/bevy_tileset", version = "0.3" }
```

## ✨ Usage

### 🧩 Tilesets

For info on how to define and use tilesets, check out the [README](https://github.com/MrGVSV/bevy_tileset#-usage)
for `bevy_tileset`. This crate re-exports the entire crate under the `tileset` submodule.

To use it, make sure you add the following to your app:

```rust
fn main() {
    App::new()
        // ...
        .add_plugin(TilesetPlugin)
        // ...
        .run();
}
```

> **Note:** `TilesetPlugin` is an override of the one exported from `bevy_tileset`. Be sure to use the one from this crate!

### 💾 Serialization/Deserialization

> With the `serialization` feature enabled

With this crate, serialization is very simple (as long as your tiles are generated using tilesets).

Simply add the `TilemapSerializer` to your system:

```rust
/// Assumes bevy_ecs_tilemap has already been properly setup to have tiles read from it
fn save_maps(serializer: TilemapSerializer) {
    // This saves all currently generated maps
    let maps = serializer.save_maps();

    // Write to disk using something like serde_json...
}
```

And deserializing is just as simple:

```rust
/// Assumes bevy_ecs_tilemap has already been properly setup to have tiles placed into it
fn load_maps(mut serializer: TilemapSerializer) {
    let path = FileAssetIo::get_root_path().join("assets/map.json");
    let data = std::fs::read_to_string(path).unwrap();
    let maps = serde_json::from_str::<SerializableTilemap>(&data).unwrap();

    serializer.load_maps(&maps);
}
```

Check out
the [serialization](https://github.com/MrGVSV/bevy_tileset/blob/main/bevy_tileset_map/examples/serialization.rs)
example to see how we turn
some [JSON](https://github.com/MrGVSV/bevy_tileset/tree/main/bevy_tileset_map/assets/map.json) into a full
tilemap. Again, as long as you set everything up using tilesets, it should work pretty much as expected.

### 🏗 Placement/Removal

<p align="center">
	<img alt="Tile placement modes" src="https://github.com/MrGVSV/bevy_tileset/main/screenshots/tile_placement_modes.gif" />
</p>

One of the nice features about this crate is that it provides some built-in tile placement/removal logic so you don't have to! This can easily be accessed using the `TilePlacer` system param.

```rust
fn place_tile(mut placer: Tileplacer, /* ... */) {
  // ...
  placer.place(
    tile_id,
    tile_pos,
    map_id,
    layer_id
  );
}
```

Easy!

Plus it comes with other variants the `place` method (with all the same properties). So give them a try!

### 🧠 Auto Tiling

<p align="center">
	<img alt="Auto tiling" src="https://github.com/MrGVSV/bevy_tileset/blob/b81d2d7483785e5aa58ef0b449482d9d57bca3be/screenshots/auto_tiling_demo.gif" />
</p>

While `bevy_tileset` adds the ability to define Auto Tiles, this crate actually puts it to use.

Using the aforementioned `TilePlacer` system parameter makes handling auto tiles a breeze! Besides simply handling the placement code, internally this handles multiple things related to auto tiles:

1. When placing a tile, it automatically inserts the required `AutoTile` component
2. If a tile needs to be removed and it's an auto tile, the removal process is also automatically taken care of

If you decide you want to do this manually, make sure you properly handle the placement/removal process. When placing you *must* add the `AutoTile` component (so the `AutoTiler` knows it exists). And when you remove an auto tile, make sure you send a `RemoveAutoTileEvent` event (otherwise surrounding auto tiles won't know to update).

Just remember that auto tiles can be _slow_, so thousands of them may result in lag when first placed (this can be mitigated by avoiding very large batch placements). However, once placed, they don't need to be updated anymore, so it shouldn't affect performance after that.

## 🎓 Examples

Check out the [examples](https://github.com/MrGVSV/bevy_tileset#-examples) for `bevy_tileset` for tileset-specific
examples.

* [clickable](bevy_tileset_map/examples/clickable.rs) - Add and remove tiles using `bevy_ecs_tilemap`
  and `bevy_tileset_map`
* [serialization](https://github.com/MrGVSV/bevy_tileset/blob/main/bevy_tileset_map/examples/serialization.rs) -
  Load a tilemap from JSON
* [drag](https://github.com/MrGVSV/bevy_tileset/blob/main/bevy_tileset_map/examples/drag.rs) -
  Click and drag to add or remove tiles

## 🕊 Bevy Compatibility

| bevy | bevy_tileset_map |
| ---- | ------------------------ |
| 0.6  | 0.3                      |
| 0.5  | 0.2                      |
