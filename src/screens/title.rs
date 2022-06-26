// screens/title.rs

use std::any::Any;

use macroquad::prelude::*;
//use futures::executor::block_on;

use async_trait::async_trait;

use crate::texture_mgr::TextureMgr;

use crate::screens::Screen;

const TITLE_PATH: &str = "Textures/Screens/title.png";

pub struct TitleScreen
{
    time_elapsed: f32,
    is_complete_flag: bool,
    is_loaded_flag: bool,
}

impl TitleScreen {
    pub fn new() -> TitleScreen {
	
	TitleScreen {
	    is_complete_flag: false,
	    time_elapsed: 0.0,
	    is_loaded_flag: false,
	}
    }
}

#[async_trait]
impl Screen for TitleScreen {
    fn is_loaded(&self) -> bool {
	self.is_loaded_flag
    }

    async fn load(&mut self, tex_mgr: &mut TextureMgr) {
	let texture_filenames = vec!(TITLE_PATH.to_string());

	tex_mgr.load(texture_filenames).await;
	
	self.is_loaded_flag = true;
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
	clear_background(WHITE);

	let title_tex = tex_mgr.get_texture(&TITLE_PATH.to_string());
	
	draw_texture(
	    *title_tex,
	    (screen_width() - title_tex.width()) / 2.0,
	    (screen_height() - title_tex.height()) / 2.0,
	    WHITE,
	);
    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}

