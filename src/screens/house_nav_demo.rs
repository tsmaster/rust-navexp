// screens/house_nav_demo.rs

use std::any::Any;

use macroquad::prelude::*;
//use futures::executor::block_on;

use async_trait::async_trait;

use crate::texture_mgr::TextureMgr;

use crate::screens::Screen;

use crate::GameScreen;

const BRICK_PATH: &str = "Textures/brick.png";
const CAR_PATH: &str = "Textures/car.png";
const HOUSE_PATH: &str = "Textures/house.png";
const PARK_PATH: &str = "Textures/parking_space.png";
const ROBOT_PATH: &str = "Textures/robot.png";

pub struct HouseNavDemo
{
    is_loaded_flag: bool,
    is_complete_flag: bool,
}

impl HouseNavDemo {
    pub fn new() -> HouseNavDemo {
	HouseNavDemo {
	    is_loaded_flag: false,
	    is_complete_flag: false,
	}
    }
}

#[async_trait]
impl Screen for HouseNavDemo {
    fn tick(&mut self, dt: f32) {
	// todo move things around

	if is_key_down(KeyCode::Escape) {
	    self.is_complete_flag = true;
	}
    }

    fn is_complete(&self) -> bool {
	self.is_complete_flag
    }

    fn is_loaded(&self) -> bool {
	self.is_loaded_flag
    }

    async fn load(&mut self, tex_mgr: &mut TextureMgr) {
	let texture_filenames = vec!(
	    BRICK_PATH.to_string(),
	    CAR_PATH.to_string(),
	    HOUSE_PATH.to_string(),
	    PARK_PATH.to_string(),
	    ROBOT_PATH.to_string(),
	);

	tex_mgr.load(texture_filenames).await;

	self.is_loaded_flag = true;
    }

    fn render(&self, tex_mgr: &TextureMgr) {
        clear_background(LIGHTGRAY);

	let park_texture = tex_mgr.get_texture(&PARK_PATH.to_string());

        draw_texture(
            *park_texture,
	    675.0,
	    175.0,
            WHITE,
        );

	let house_texture = tex_mgr.get_texture(&HOUSE_PATH.to_string());

        draw_texture(
            *house_texture,
	    500.0,
	    50.0,
            WHITE,
        );

	let robot_texture = tex_mgr.get_texture(&ROBOT_PATH.to_string());

        draw_texture(
            *robot_texture,
            300.0,
	    150.0,
            WHITE,
        );

	let brick_texture = tex_mgr.get_texture(&BRICK_PATH.to_string());

	for x in (0..800).step_by(50) {
            draw_texture(
		*brick_texture,
		x as f32,
		0.0,
		WHITE,
            );
            draw_texture(
		*brick_texture,
		x as f32,
		550.0,
		WHITE,
            );
	}

	for y in (50..550).step_by(50) {
            draw_texture(
		*brick_texture,
		0.0,
		y as f32,
		WHITE,
            );
            draw_texture(
		*brick_texture,
		750.0,
		y as f32,
		WHITE,
            );
	}

	let car_params = DrawTextureParams {
	    dest_size: None,
	    source: None,
	    rotation: std::f32::consts::PI / 2.0,
	    flip_x: false,
	    flip_y: false,
	    pivot: None
	};
	/*
	    dest_size : Some(vec2(25.0, 50.0)),
	    source : Some(Rect {
		x:0.0,
		y:0.0,
		w: 50.0,
		h: 100.0}),
	    rotation: 1.5,
	    flip_x: false,
	    flip_y: false,
	    pivot: Some(vec2(25., 50.)),
    };*/

	let car_texture = tex_mgr.get_texture(&CAR_PATH.to_string());	

        draw_texture_ex(
            *car_texture,
            100.0,
	    400.0,
            WHITE,
	    car_params,
        );

    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}
