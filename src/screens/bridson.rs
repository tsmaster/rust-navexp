// screens/bridson.rs

use std::any::Any;

//use ::rand::prelude::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

use async_trait::async_trait;

use crate::screens::Screen;

use crate::texture_mgr::TextureMgr;

use crate::big_dice_games::math::vector::Vec2f;
use crate::big_dice_games::util::map;
use crate::big_dice_games::math::vector::Vector;

use voronoice::*;


#[derive(PartialEq)]
enum SubMode {
    AddPoints,
    Show,
}


pub struct BridsonScreen
{
    time_elapsed: f32,
    is_complete_flag: bool,
    points: Vec<Vec2f>,
    points_open: Vec<bool>,

    num_x: i32,
    num_y: i32,
    cell_count: i32,

    occupancy: Vec<i32>,

    radius: f32,
    cell_width: f32,

    sub_mode: SubMode,
    voronoi_data: Option<Voronoi>,
}

impl BridsonScreen {
    pub fn new() -> BridsonScreen {
	//let mut rng = thread_rng();

	let radius: f32 = 20.0;
	let cell_width: f32 = radius / 2.0_f32.sqrt();

	let num_x = (screen_width() / cell_width).ceil() as i32;
	let num_y = (screen_height() / cell_width).ceil() as i32;

	let cell_count = num_x * num_y;

	println!("num x: {}", num_x);
	println!("num y: {}", num_y);
	println!("num cells: {}", cell_count);

	let mut point_list:Vec<Vec2f> = Vec::new();
	let mut points_open_list:Vec<bool> = Vec::new();

	//for _i in 0..2000 {
	let v = Vec2f {
	    //x: rng.gen_range(0.0 .. screen_width()),
	    //y: rng.gen_range(0.0 .. screen_height()),

	    x: gen_range::<f32>(0.0, screen_width()),
	    y: gen_range::<f32>(0.0, screen_height()),
	};

	point_list.push(v);
	points_open_list.push(true);
	//}

	let occupancy:Vec<i32> = vec![-1; cell_count.try_into().unwrap()];

	BridsonScreen {
	    is_complete_flag: false,
	    time_elapsed: 0.0,
	    points: point_list,
	    points_open: points_open_list,
	    num_x: num_x,
	    num_y: num_y,
	    cell_count: cell_count,
	    occupancy: occupancy,
	    radius: radius,
	    cell_width: cell_width,
	    sub_mode: SubMode::AddPoints,
	    voronoi_data: Option::<Voronoi>::None,
	}
    }

    pub fn reset(&mut self) {
	self.time_elapsed = 0.0;
	self.points.clear();
	self.points_open.clear();
	self.occupancy = vec![-1; self.cell_count.try_into().unwrap()];
	self.is_complete_flag = false;
	self.sub_mode = SubMode::AddPoints;
	self.voronoi_data = Option::<Voronoi>::None;	
    }

    fn point_in_box(&self, p: &Vec2f) -> bool {
	p.x >= 0.0 &&
	    p.x < screen_width() &&
	    p.y >= 0.0 &&
	    p.y < screen_height()
    }

    fn make_voronoi(&mut self) {
	println!("making voronoi");

	let mut sites:Vec<Point> = Vec::new();
	for p in &self.points {
	    let site_point = Point {
		x: p.x as f64,
		y: p.y as f64,
	    };
	    
	    sites.push(site_point);
	}

	let scr_w = (screen_width() / 2.0) as f64;
	let scr_h = (screen_height() / 2.0) as f64;

	let bounding_box = BoundingBox::new(
	    Point{
		x: scr_w,
		y: scr_h,
	    },
	    scr_w * 2.0,
	    scr_h * 2.0,
	);
	let my_voronoi = VoronoiBuilder::default()
	    .set_sites(sites)
	    .set_bounding_box(bounding_box)
	    .set_lloyd_relaxation_iterations(0)
	    .build();
	self.voronoi_data = my_voronoi;
    }

    fn cell_index(&self, p: &Vec2f) -> i32 {
	let ix = (p.x / self.cell_width).floor() as i32;
	let iy = (p.y / self.cell_width).floor() as i32;
	iy * self.num_x + ix
    }

    fn find_open_index(&self) -> i32 {
	if self.points.len() == 0 {
	    return -1;
	}
	
	//let mut rng = thread_rng();
	//let offset = rng.gen_range(0 .. self.points.len());
	let offset = gen_range::<usize>(0, self.points.len());

	for i in 0 .. self.points.len() {
	    let mod_i = (i + offset) % self.points.len();
	    if self.points_open[mod_i] {
		return mod_i as i32;
	    }
	}
	-1
    }

    fn find_neighbor_points(&self, p: &Vec2f) -> Vec<Vec2f> {
	let mut out_vec:Vec<Vec2f> = Vec::new();

	let p_index = self.cell_index(p);
	
	for dx in -2 .. 3 {
	    for dy in -2 .. 3 {
		let n_index = p_index + dx + dy * self.num_x;
		if n_index < 0 || n_index >= self.occupancy.len() as i32 {
		    continue;
		}
		if self.occupancy[n_index as usize] != -1 {
		    
		    out_vec.push(self.points[self.occupancy[n_index as usize] as usize]);
		}
	    }
	}

	out_vec
    }

    fn insert_point(&mut self, p: &Vec2f) {
	if !self.point_in_box(p) {
	    return;
	}

	for neighbor_point in self.find_neighbor_points(p) {
	    let dist = (neighbor_point - *p).mag();
	    if dist < self.radius {
		return;
	    }
	}

	
	let ci = self.cell_index(p) as usize;
	if self.occupancy[ci] == -1 {
	    self.occupancy[ci] = self.points.len() as i32;
	    self.points.push(*p);
	    self.points_open.push(true);
	}
    }

    fn make_new_points_around(&mut self, i: usize) -> Vec<Vec2f> {
	let center_vec = &self.points[i];

	let num_steps = 30;

	let mut out_vec:Vec<Vec2f> = Vec::new();

	//let mut rng = thread_rng();
	
	//let theta_rot = rng.gen_range(0.0 .. 3.141592654 * 2.0);
	let theta_rot = gen_range::<f32>(0.0, 3.141592654 * 2.0);

	for step in 0 .. num_steps {
	    let theta = map(step as f32, 0.0, num_steps as f32,
			    0.0, 3.141592654 * 2.0) + theta_rot;

	    let offset_vec = Vec2f {
		x: self.radius * theta.cos(),
		y: self.radius * theta.sin()
	    };
	    out_vec.push(*center_vec + offset_vec);
	}
	
	self.points_open[i] = false;

	out_vec
    }
}

#[async_trait]
impl Screen for BridsonScreen {
    fn is_loaded(&self) -> bool {
	true
    }

    async fn load(&mut self, _tex_mgr: &mut TextureMgr) {
    }
    
    fn tick(&mut self, dt: f32) {
	// TODO wait for 2 seconds, or get key input

	self.time_elapsed += dt;

	if self.time_elapsed >= 25.0 {
	    self.is_complete_flag = true;
	}

	if is_key_down(KeyCode::Escape) {
	    self.is_complete_flag = true;
	    return;
	}	

	if self.sub_mode == SubMode::AddPoints {

 	    if self.points.len() == 0 {
 		let p = Vec2f {
		    x: screen_width() / 2.0,
		    y: screen_height() / 2.0
		};

		self.insert_point(&p);
	    } else {
		
		for _i in 0 .. 5 {
		    
		    let open_index = self.find_open_index();
		    if open_index < 0 {
			//self.is_complete_flag = true;
			self.sub_mode = SubMode::Show;
			self.make_voronoi();
			break;
		    }
		    else
		    {
			let new_points = self.make_new_points_around(
			    open_index.try_into().unwrap());
		
			for p in new_points.iter() {
			    self.insert_point(p);
			}
		    }
		}
	    }
	}
    }

    fn is_complete(&self) -> bool {
	self.is_complete_flag
    }

    fn render(&self, _tex_mgr: &TextureMgr) {
	clear_background(WHITE);

	let dot_size = 2.5;
	let half_dot_size = dot_size / 2.0;

	if self.points.len() > 0 {
	    for i in 0 .. self.points.len() {
		let p = &self.points[i];

		let c = match self.points_open[i] {
		    true => RED,
		    false => BLACK
		};

		/*
		draw_rectangle(p.x,
			       p.y,
			       1.0, 1.0,
			       c);		
		 */

		draw_rectangle(p.x - half_dot_size,
			       p.y - half_dot_size,
			       dot_size,
			       dot_size,
			       c);
		 
		
		//draw_circle(p.x, p.y, half_dot_size, c);
	    }
	}

	match &self.voronoi_data {
	    Option::<Voronoi>::None => {}
	    Option::<Voronoi>::Some(data) => {
		let verts = data.vertices();
		let cells = data.cells();

		for c in 0 .. cells.len() {
		    let nv = cells[c].len();
		    for vi in 0 .. nv {
			let vj = (vi + 1) % nv;
			draw_line(verts[cells[c][vi]].x as f32,
				  verts[cells[c][vi]].y as f32,
				  verts[cells[c][vj]].x as f32,
				  verts[cells[c][vj]].y as f32,
				  1.5,
				  BLUE);
		    }
		}
	    }
	}
    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}
