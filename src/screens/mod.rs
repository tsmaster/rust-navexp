// screens/mod.rs

use std::any::Any;

pub mod bdg_logo;
pub mod title;
pub mod menu;

pub mod bridson;
pub mod bridson_nav;
pub mod square_nav;

pub mod house_nav_demo;

use crate::texture_mgr::TextureMgr;

use async_trait::async_trait;

#[async_trait]
pub trait Screen {
    fn tick(&mut self, dt: f32);
    fn is_loaded(&self) -> bool;
    async fn load(&mut self, tex_mgr: &mut TextureMgr);
    fn is_complete(&self) -> bool;
    fn render(&self, tex_mgr: &TextureMgr); // maybe take a render context?

    fn as_any(&self) -> &dyn Any;
}
