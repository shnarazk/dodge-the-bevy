# Dodge the bevy, a simple game in Rust with Bevy-0.6

This is a modified and extended game of [Dodge the Creeps](https://github.com/godotengine/godot-demo-projects/tree/master/2d/dodge_the_creeps) for [Godot game engine](https://godotengine.org).

![](https://user-images.githubusercontent.com/997855/148645225-a2403eee-200b-44c8-9787-8b4588131a98.gif)

### Run

```
cargo run --release
```

### Play

Dodge them just by moving pointer. It follows. No extra action by clicking, dragging nor pressing.

Note: the creeps accelerate in non-linearly.

### Technical note

The background is a [examples/2d/mesh2d.rs](https://github.com/bevyengine/bevy/blob/v0.6.0/examples/2d/mesh2d.rs).
Its animation is based on [examples/shader/animated_shader.rs](https://github.com/bevyengine/bevy/blob/v0.6.0/examples/shader/animate_shader.rs). The shader is copied from [Rhythm game in Rust using Bevy](https://caballerocoll.com/blog/bevy-rhythm-game/).

## Copying

`assets/sounds/House In a Forest Loop.ogg` Copyright &copy; 2012 [HorrorPen](https://opengameart.org/users/horrorpen), [CC-BY 3.0: Attribution](http://creativecommons.org/licenses/by/3.0/). Source: https://opengameart.org/content/loop-house-in-a-forest

`assets/sounds/laserpew.ogg` https://opengameart.org/content/pew-laser-fire-sound, [CC0](http://creativecommons.org/publicdomain/zero/1.0/)

`assets/sprites/bevy_logo_dark_*.png` are from "Abstract Platformer". Created in 2016 by kenney.nl, [CC0 1.0 Universal](http://creativecommons.org/publicdomain/zero/1.0/). Source: https://www.kenney.nl/assets/abstract-platformer

`assets/fonts/Xolonium-Regular.ttf` is "Xolonium". Copyright &copy; 2011-2016 Severin Meyer <sev.ch@web.de>, with Reserved Font Name Xolonium, SIL open font license version 1.1. Details are in `fonts/LICENSE.txt`.

## References

- [Bevy](https://bevyengine.org/)
- [Bevy repository](https://github.com/bevyengine/bevy)
- [Dodge the Creeps](https://github.com/godotengine/godot-demo-projects/tree/master/2d/dodge_the_creeps)
- [Godot game engine](https://godotengine.org/)
- [Rhythm game in Rust using Bevy](https://caballerocoll.com/blog/bevy-rhythm-game/)
- [Rust](https://www.rust-lang.org/)
