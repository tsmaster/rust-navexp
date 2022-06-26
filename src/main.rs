use macroquad::prelude::*;

mod screens;
mod demos;
mod big_dice_games;
mod sprite;

mod texture_mgr;

//use big_dice_games::util::map;
use screens::Screen;

#[derive(Copy, Clone)]
pub enum GameScreen {
    BdgLogo,
    Title,
    Menu,
    Bridson,
    HouseNav,
}

fn window_conf() -> Conf {
    Conf {
	window_title: "Nav Experiments".to_owned(),
	fullscreen: false,
	high_dpi: false,

	window_width: 1100,
	window_height: 850,

	//window_width: 800,
	//window_height: 400,

	window_resizable: false,

	..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut texture_mgr = texture_mgr::TextureMgr::new();
    
    let mut bdg_logo_screen = screens::bdg_logo::BdgLogoScreen::new();
    let mut title_screen = screens::title::TitleScreen::new();
    let mut menu_screen = screens::menu::MenuScreen::new();
    let mut bridson_screen = screens::bridson::BridsonScreen::new();
    let mut house_nav_demo = screens::house_nav_demo::HouseNavDemo::new();

    let mut game_mode = GameScreen::BdgLogo;

    loop {
	//println!("FPS: {}", get_fps());

	let mut scr:&mut dyn Screen = match game_mode {
	    GameScreen::BdgLogo => &mut bdg_logo_screen,
	    GameScreen::Title => &mut title_screen,
	    GameScreen::Menu => &mut menu_screen,
	    GameScreen::Bridson => &mut bridson_screen,
	    GameScreen::HouseNav => &mut house_nav_demo,
	};

	if !(scr.is_loaded()) {
	    scr.load(&mut texture_mgr).await;
	    continue;
	}	

	scr.tick(get_frame_time());
	if scr.is_complete() {
	    match game_mode {
		GameScreen::BdgLogo => {
		    game_mode = GameScreen::Title;
		}
		GameScreen::Title => {
		    game_mode = GameScreen::Menu;
		}
		GameScreen::Menu => {
		    scr = &mut title_screen;
		    
		    match menu_screen.next_screen {
			Some(m) => {
			    game_mode = m;
			}
			None => {
			    println!("unknown menu successor");
			}
		    }
		    menu_screen.reset();
		}
		GameScreen::Bridson => {
		    bridson_screen.reset();
		    game_mode = GameScreen::Menu;
		    scr = &mut menu_screen;
		}
		GameScreen::HouseNav => {
		    game_mode = GameScreen::Menu;
		    scr = &mut menu_screen;
		}
	    }
	}
	scr.render(&texture_mgr);
        next_frame().await
    }
}
