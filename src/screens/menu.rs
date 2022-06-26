// screens/menu.rs

use std::any::Any;

use macroquad::prelude::*;

use async_trait::async_trait;

use crate::screens::Screen;

use crate::GameScreen;
use crate::texture_mgr::TextureMgr;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};

const MENU_PATH: &str = "Textures/Screens/menu.png";

pub struct MenuScreen
{
    time_elapsed: f32,
    is_complete_flag: bool,
    is_loaded_flag: bool,
    pub next_screen: Option::<GameScreen>
}

impl MenuScreen {
    pub fn new() -> MenuScreen {
	MenuScreen {
	    is_complete_flag: false,
	    time_elapsed: 0.0,
	    is_loaded_flag: false,
	    next_screen: Option::<GameScreen>::None,
	}
    }

    pub fn reset(&mut self) {
	self.next_screen = Option::<GameScreen>::None;
	self.is_complete_flag = false;
	self.time_elapsed = 0.0;
    }
}

#[async_trait]
impl Screen for MenuScreen {
    fn is_loaded(&self) -> bool {
	self.is_loaded_flag
    }

    async fn load(&mut self, tex_mgr: &mut TextureMgr) {
	let texture_filenames = vec!(MENU_PATH.to_string());
	    
	tex_mgr.load(texture_filenames).await;

	self.is_loaded_flag = true;	
    }
    
    fn tick(&mut self, dt: f32) {
	// TODO wait for 2 seconds, or get key input

	self.time_elapsed += dt;

	if self.time_elapsed >= 2.0 {
	    //self.is_complete_flag = true;
	}

	widgets::Window::new(hash!(), vec2(400., 200.),
			     vec2(320., 400.))
	    .label("Menu")
	    .titlebar(true)
	    .ui(&mut *root_ui(), |ui| {
		ui.button(Vec2::new(30., 0.),
			  "discrete");
		
		if ui.button(Vec2::new(40., 20.),
			     "draw bridson") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::Bridson);
		}

		if ui.button(Vec2::new(40., 40.),
			     "bridson nav") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::Bridson);
		}

		if ui.button(Vec2::new(40., 60.),
			     "square grid nav") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::Bridson);
		}

		if ui.button(Vec2::new(40., 80.),
			     "square grid D*Lite exploration") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::Bridson);
		}

		ui.button(Vec2::new(30., 100.),
			  "continuous");
		
		if ui.button(Vec2::new(40., 120.),
			     "RRT house") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::HouseNav);
		}

		if ui.button(Vec2::new(40., 140.),
			     "RRT* house") {
		    self.is_complete_flag = true;
		    self.next_screen = Option::<GameScreen>::Some(GameScreen::HouseNav);
		}
	    });
	
    }

    fn is_complete(&self) -> bool {
	self.is_complete_flag
    }

    fn render(&self, tex_mgr: &TextureMgr) {
	// TODO draw menu
	let bg_color = color_u8!(
	    45,
	    55,
	    229,
	    255);
	
	clear_background(bg_color);

	let screen_texture = tex_mgr.get_texture(&MENU_PATH.to_string());
	
	draw_texture(
	    *screen_texture,
	    0.0,
	    0.0,
	    WHITE,
	);
    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}

