# Bevy zhuose qi

Easy to use shaders for Bevy that works in native and on WebGL2.

## Top Down Fire

From https://github.com/wilk10/shader_practice/tree/main/src/shaders/fire

```rust
use bevy_zhuose_qi::topdownfire::*;

fn setup(
    mut commands: Commands,
    mut fire_textures: ResMut<Assets<FireTexture>>,
) {
    commands.spawn_bundle(FireBundle {
        fire_texture: fire_textures.add(my_texture_handle.into()),
        ..Default::default()
    });
}
```
![single fire](./examples/single_fire.gif)

See the [examples](./examples) for a working code example.