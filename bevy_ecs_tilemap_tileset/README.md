# bevy_ecs_tilemap_tileset

An implementation of  [`bevy_tileset`](https://github.com/MrGVSV/bevy_tileset) for the [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap) crate.

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

## 📲 Installation

This crate is still not publicly released yet as I might want to make a PR to try and integrate it directly with `bevy_ecs_tilemap`, but for now you can use it with git:

```toml
[dependencies]
bevy_ecs_tilemap_tileset = { git = "https://github.com/MrGVSV/bevy_tileset", version = "0.2" }
```

## ✨ Usage

### 🧩 Tilesets

For info on how to define and use tilesets, check out the [README](https://github.com/MrGVSV/bevy_tileset#-usage) for `bevy_tileset`. This crate re-exports the entire crate under the `tileset` submodule.

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

Check out the [serialization](https://github.com/MrGVSV/bevy_tileset/blob/main/bevy_ecs_tilemap_tileset/examples/serialization.rs) example to see how we turn some [JSON](https://github.com/MrGVSV/bevy_tileset/tree/main/bevy_ecs_tilemap_tileset/assets/map.json) into a full tilemap. Again, as long as you set everything up using tilesets, it should work pretty much as expected.

### 🧠 Auto Tiling

<p align="center">
	<img alt="Auto tiling" src="https://github.com/MrGVSV/bevy_tileset/blob/b81d2d7483785e5aa58ef0b449482d9d57bca3be/screenshots/auto_tiling_demo.gif" />
</p>

While `bevy_tileset` adds the ability to define Auto Tiles, this crate actually puts it to use.

If you use the `place_tile` function to place tiles from your tileset, then this should work automatically (mostly). If not, just be sure to add this when you spawn your tile:

```rust
let tile_id = ...
commands.entity(tile_entity).insert(AutoTile::new(id.group_id, tile_id.tileset_id));
```

The `AutoTile` component acts as a marker, telling the auto tiling system that our tile is indeed an Auto Tile. And removing that component, removes it from the auto tiling system.

> Remember that adding/removing the component is automatically handled when you place the tile down using `place_tile`.

There are a few *annoying* things you'll have to be aware of, though— both of which relate to removing Auto Tiles.

1. You need to ensure that any code that might remove an Auto Tile is put in the correct Stage and runs before `TilesetLabel::RemoveAutoTiles`:

   ```rust
   .add_system_to_stage(
     TilesetStage,
     my_tile_removal_system.before(TilesetLabel::RemoveAutoTiles),
   )
   ```

   Failure to do this won't affect any other tile operation (including adding an Auto Tile). The only thing it affects is how the neighboring tiles are updated after removal.

2. If you remove an Auto Tile, you also need to send a `RemoveAutoTileEvent` event:

   ```rust
   fn my_tile_removal_system(mut event_writer: EventWriter<RemoveAutoTileEvent>, /* ... */) {
     // ...
     if is_auto_tile {
       event_writer.send(RemoveAutoTileEvent(tile_entity));
     }
   }
   ```

Other than that, though, it should be good to go! Just be careful with it. Auto tiles are slow, so thousands of them may result in lag when first placed (this can be mitigated by avoiding very large batch placements).

## 🎓 Examples

Check out the [examples](https://github.com/MrGVSV/bevy_tileset#-examples) for `bevy_tileset` for tileset-specific examples.

* [clickable](bevy_ecs_tilemap_tileset/examples/clickable.rs) - Add and remove tiles using `bevy_ecs_tilemap` and `bevy_ecs_tilemap_tileset`
*  [serialization](https://github.com/MrGVSV/bevy_tileset/blob/main/bevy_ecs_tilemap_tileset/examples/serialization.rs) - Load a tilemap from JSON

## 🕊 Bevy Compatibility

| bevy | bevy_tileset |
| ---- | ------------ |
| 0.5  | 0.2          |
