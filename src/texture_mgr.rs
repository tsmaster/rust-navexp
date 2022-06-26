use std::collections::HashMap;

use macroquad::prelude::*;

//use macroquad::prelude::state_machine::StateMachine::Ready;
//use macroquad::prelude::state_machine::StateMachine::Pending;

//use core::future::Future;

pub struct TextureMgr {
    texture_map: HashMap<String, Texture2D>,
    empty_texture: Texture2D,
    loaded_count: u32,
    target_count: u32,
    error_count: u32,
}

impl TextureMgr {
    pub fn new() -> TextureMgr {
	TextureMgr {
	    texture_map: HashMap::new(),
	    empty_texture: Texture2D::empty(),
	    loaded_count: 0,
	    target_count: 0,
	    error_count: 0,
	}
    }
    
    pub async fn load(&mut self, texture_names: Vec<String>) {
	self.loaded_count = 0;
	self.error_count = 0;
	self.target_count = texture_names.len() as u32;
	
	for n in texture_names {
	    let texture:Texture2D = load_texture(&n).await.unwrap();
	    self.texture_map.insert(n, texture);
	    self.loaded_count += 1;
	}
    }

    pub fn complete(&self) -> bool {
	self.loaded_count + self.error_count == self.target_count
    }

    pub fn clear(&mut self) {
	self.texture_map.clear();
	self.loaded_count = 0;
	self.error_count = 0;
	self.target_count = 0;
    }

    pub fn get_texture(&self, texture_name: &String) -> &Texture2D {
	if self.texture_map.contains_key(texture_name) {
	    return self.texture_map.get(texture_name).unwrap();
	}
	
	// return empty
	&self.empty_texture
    }
}
