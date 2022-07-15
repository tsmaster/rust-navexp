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
    BridsonNav,
    SquareNav,
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
    let mut bridson_nav_screen = screens::bridson_nav::BridsonNavScreen::new();
    let mut square_nav_screen = screens::square_nav::SquareNavScreen::new();
    let mut house_nav_demo = screens::house_nav_demo::HouseNavDemo::new();

    let mut game_mode = GameScreen::BdgLogo;

    loop {
	//println!("FPS: {}", get_fps());

	if is_key_pressed(KeyCode::Space) {
	    println!("SPACE");
	}

	let scr:&mut dyn Screen = match game_mode {
	    GameScreen::BdgLogo => &mut bdg_logo_screen,
	    GameScreen::Title => &mut title_screen,
	    GameScreen::Menu => &mut menu_screen,
	    GameScreen::Bridson => &mut bridson_screen,
	    GameScreen::BridsonNav => &mut bridson_nav_screen,
	    GameScreen::SquareNav => &mut square_nav_screen,
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
		}
		GameScreen::BridsonNav => {
		    bridson_nav_screen.reset();
		    game_mode = GameScreen::Menu;
		}
		GameScreen::SquareNav => {
		    square_nav_screen.reset();
		    game_mode = GameScreen::Menu;
		}				
		GameScreen::HouseNav => {
		    game_mode = GameScreen::Menu;
		}
	    }
	} else {
	    scr.render(&texture_mgr);
	}
        next_frame().await
    }
}
