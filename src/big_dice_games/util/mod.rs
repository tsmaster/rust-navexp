// big_dice_games/util/mod.rs

use std::ops::Mul;

pub trait ScaleByFloat {
    fn scale(&self, s: f32) -> Self;
}

impl ScaleByFloat for f32 {
    fn scale(&self, s: f32) -> Self {
	*self * s
    }
}


pub fn map <T: ScaleByFloat + Copy + std::ops::Sub<Output = T>>(
    val: f32,
    in_min: f32, in_max: f32,
    out_min: T, out_max: T)
    -> T

where f32: Mul<T>, T: std::ops::Add<Output = T> +
      std::ops::Mul<Output = T> +
    std::ops::Sub<Output = T>
    
{
    let frac = (val - in_min) / (in_max - in_min);

    let out_diff : T = out_max - out_min;
    let scaled_out_diff: T = out_diff.scale(frac);
    
    scaled_out_diff + out_min
}


