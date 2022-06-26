// sprite.rs

use macroquad::prelude::*;

struct Sprite {
    x: f32,
    y: f32,

    texture: Texture2D,

    theta: f32,

    width: f32,
    height: f32,
}


