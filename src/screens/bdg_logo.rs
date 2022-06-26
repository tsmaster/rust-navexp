// screens/bdg_logo.rs

use std::any::Any;

use macroquad::prelude::*;
//use futures::executor::block_on;

use async_trait::async_trait;
use crate::screens::Screen;
use crate::texture_mgr::TextureMgr;

const LOGO_PATH: &str = "Textures/Screens/bdg_logo.png";

pub struct BdgLogoScreen
{
    time_elapsed: f32,
    is_complete_flag: bool,
    is_loaded_flag: bool,
}

impl BdgLogoScreen {
    pub fn new() -> BdgLogoScreen {
	BdgLogoScreen {
	    is_complete_flag: false,
	    time_elapsed: 0.0,
	    is_loaded_flag: false,
	}
    }
}

#[async_trait]
impl Screen for BdgLogoScreen {
    async fn load(&mut self, tex_mgr:&mut TextureMgr) {
	let texture_filenames = vec!(LOGO_PATH.to_string());
	    
	tex_mgr.load(texture_filenames).await;

	self.is_loaded_flag = true;
    }

    fn is_loaded(&self) -> bool {
	self.is_loaded_flag
    }
    
    fn tick(&mut self, dt: f32) {
	// TODO wait for 2 seconds, or get key input

	self.time_elapsed += dt;

	if self.time_elapsed >= 2.0 {
	    self.is_complete_flag = true;
	}
    }

    fn is_complete(&self) -> bool {
	self.is_complete_flag
    }

    fn render(&self, tex_mgr: &TextureMgr) {
	let bg_color = color_u8!(
	    136, 249, 163, 255);

	clear_background(bg_color);

	let logo_screen = tex_mgr.get_texture(&LOGO_PATH.to_string());
	
	draw_texture(
	    *logo_screen,
	    (screen_width() - logo_screen.width()) / 2.0,
	    (screen_height() - logo_screen.height()) / 2.0,
	    WHITE,
	);
    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}

