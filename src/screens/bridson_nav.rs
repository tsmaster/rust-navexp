// screens/bridson_nav.rs

use std::any::Any;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use async_trait::async_trait;

use crate::screens::Screen;

use crate::texture_mgr::TextureMgr;

use crate::big_dice_games::math::vector::Vec2f;
use crate::big_dice_games::util::map;
use crate::big_dice_games::math::vector::Vector;

//use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use voronoice::*;


#[derive(PartialEq)]
enum SubMode {
    AddPoints,
    Show,
    FindPath,
}

#[derive(Debug)]
struct AStarRecord
{
    combined_distances: f32,
    distance_travelled: f32,
    heuristic_remaining: f32,
    point: Vec2f,
    index: i32,
}

impl Ord for AStarRecord
{
    fn cmp(&self, other: &Self) -> Ordering {
	// Note, reverse order so we have a min-heap
	if self.combined_distances > other.combined_distances {
	    return Ordering::Less;
	} else if self.combined_distances < other.combined_distances {
	    return Ordering::Greater;
	}
	self.index.cmp(&other.index)
    }
}

impl PartialOrd for AStarRecord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }
}

impl Eq for AStarRecord
{

}

impl PartialEq for AStarRecord
{
    fn eq(&self, other: &Self) -> bool {
	self.index == other.index
    }
}

pub struct BridsonNavScreen
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

    start_index: i32,
    end_index: i32,

    a_star_nodes: BinaryHeap<AStarRecord>,

    found_distances: HashMap<i32, f32>,
    open_set: HashSet<i32>,

    prev_index: HashMap<i32, i32>,

    wall_nodes: HashSet<i32>,
}

impl BridsonNavScreen {
    pub fn new() -> BridsonNavScreen {
	let radius: f32 = 24.0;
	let cell_width: f32 = radius / 2.0_f32.sqrt();

	let num_x = (screen_width() / cell_width).ceil() as i32;
	let num_y = (screen_height() / cell_width).ceil() as i32;

	let cell_count = num_x * num_y;

	println!("num x: {}", num_x);
	println!("num y: {}", num_y);
	println!("num cells: {}", cell_count);

	let mut point_list:Vec<Vec2f> = Vec::new();
	let mut points_open_list:Vec<bool> = Vec::new();


	let seconds_since_start = (macroquad::time::get_time() * 1234.5) as u64;
	let m_pos:Vec2 = mouse_position().into();

	macroquad::rand::srand(seconds_since_start +
			       m_pos.x as u64 * 1234 +
			       m_pos.y as u64 * 5678);



	let v = Vec2f {
	    x: gen_range::<f32>(0.0, screen_width()),
	    y: gen_range::<f32>(0.0, screen_height()),
	};

	point_list.push(v);
	points_open_list.push(true);
	//}

	let occupancy:Vec<i32> = vec![-1; cell_count.try_into().unwrap()];

	BridsonNavScreen {
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
	    start_index: -1,
	    end_index: -1,
	    a_star_nodes: BinaryHeap::new(),
	    found_distances: HashMap::new(),
	    open_set: HashSet::new(),
	    prev_index: HashMap::new(),
	    wall_nodes: HashSet::new(),
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
	self.start_index = -1;
	self.end_index = -1;
	self.prev_index.clear();
	self.open_set.clear();
	self.found_distances.clear();
	self.a_star_nodes.clear();
	self.wall_nodes.clear();
    }

    fn point_in_box(&self, p: &Vec2f) -> bool {
	p.x >= 0.0 &&
	    p.x < screen_width() &&
	    p.y >= 0.0 &&
	    p.y < screen_height()
    }

    fn make_voronoi(&mut self) {
	println!("making voronoi for nav");

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

    fn find_index(&self, target: &Vec2f) -> i32 {
	let mut out_index = -1;
	let mut best_dist = 0.0;

	for i in 0 .. self.points.len() {
	    let p = & self.points[i];

	    let dist = (*target - *p).mag();

	    if (out_index < 0) || (dist < best_dist) {
		out_index = i as i32;
		best_dist = dist;
	    }
	}

	out_index
    }

    fn calc_heuristic_by_indices(&self, a:i32, b:i32) -> f32 {
	(self.points[a as usize] - self.points[b as usize]).mag()
    }

    fn advance_a_star(&mut self) {
	if self.found_distances.contains_key(&self.end_index) {
	    self.sub_mode = SubMode::Show;
	    return;
	}

	match self.a_star_nodes.pop() {
	    None => {
		// first step; push start on
		println!("starting search");

		let h = self.calc_heuristic_by_indices(
			self.start_index, self.end_index);

		self.a_star_nodes.push(AStarRecord {
		    combined_distances: h,
		    distance_travelled: 0.0,
		    heuristic_remaining: h,
		    point: self.points[self.start_index as usize],
		    index: self.start_index,
		});
		self.found_distances.insert(self.start_index, 0.0);
		self.open_set.insert(self.start_index);
		self.prev_index.insert(self.start_index, -1);
	    }
	    Some(n) => {
		//println!("continuing search");
		println!("processing {:?}", n);

		match &self.voronoi_data {
		    None => {
			self.sub_mode = SubMode::Show;
		    }
		    Some(vd) => {
			let cell = &vd.cell(n.index as usize);

			for neighbor_index in cell.iter_neighbors() {
			    if self.wall_nodes.contains(&(neighbor_index as i32)) {
				continue;
			    }
			    //println!("considering neighbor index {}", neighbor_index);
			    let neighbor_point = self.points[neighbor_index as usize];
			    let this_step_dist = (neighbor_point - n.point).mag();
			    let new_elapsed_dist = n.distance_travelled + this_step_dist;

			    let mut insert_node = true;

			    let neighbor_index_i32 = neighbor_index as i32;
			    if self.found_distances.contains_key(&neighbor_index_i32) {
				insert_node = self.found_distances[&neighbor_index_i32] > new_elapsed_dist;
			    }

			    if insert_node {
				/*
				println!("inserting {} with elapsed dist {}",
					 neighbor_index,
					 new_elapsed_dist);*/

				let new_h = self.calc_heuristic_by_indices(
				    neighbor_index_i32, self.end_index);

				self.a_star_nodes.push(AStarRecord {
				    combined_distances: new_h + new_elapsed_dist,
				    distance_travelled: new_elapsed_dist,
				    heuristic_remaining: new_h,
				    point: neighbor_point,
				    index: neighbor_index_i32,
				});
				self.found_distances.insert(neighbor_index_i32,
							    new_elapsed_dist);
				self.open_set.insert(neighbor_index_i32);
				self.prev_index.insert(neighbor_index_i32, n.index);
			    }
			}

			//self.open_set.remove(n.index);
		    }
		}
	    }
	}
    }
}

#[async_trait]
impl Screen for BridsonNavScreen {
    fn is_loaded(&self) -> bool {
	true
    }

    async fn load(&mut self, _tex_mgr: &mut TextureMgr) {
    }

    fn tick(&mut self, dt: f32) {
	self.time_elapsed += dt;

	if is_key_down(KeyCode::Escape) {
	    self.is_complete_flag = true;
	    return;
	}

	if is_key_down(KeyCode::S) {
	    let m_pos:Vec2 = mouse_position().into();
	    let mouse_pos_vec = Vec2f::new(m_pos.x, m_pos.y);
	    let idx = self.find_index(&mouse_pos_vec);
	    self.start_index = idx;
	}

	if is_key_down(KeyCode::E) {
	    let m_pos:Vec2 = mouse_position().into();
	    let mouse_pos_vec = Vec2f::new(m_pos.x, m_pos.y);
	    let idx = self.find_index(&mouse_pos_vec);
	    self.end_index = idx;
	}

	if is_key_down(KeyCode::R) {
	    self.reset();
	}

	if is_key_down(KeyCode::F) {
	    self.sub_mode = SubMode::FindPath;
	}

	if is_key_pressed(KeyCode::W) {
	    println!("W");
	    let m_pos:Vec2 = mouse_position().into();
	    let mouse_pos_vec = Vec2f::new(m_pos.x, m_pos.y);
	    let idx = self.find_index(&mouse_pos_vec);

	    if self.wall_nodes.contains(&idx) {
		self.wall_nodes.remove(&idx);
	    } else {
		self.wall_nodes.insert(idx);
	    }
	}


	if is_mouse_button_down(MouseButton::Left) {

	    let paint_radius = self.cell_width * 1.5;

	    let m_pos:Vec2 = mouse_position().into();
	    let mouse_pos_vec = Vec2f::new(m_pos.x, m_pos.y);
	    for idx in 0 .. self.points.len() {
		let p = &self.points[idx];

		let dist = (mouse_pos_vec - *p).mag();
		if dist < paint_radius {
		    self.wall_nodes.insert(idx as i32);
		}
	    }
	}

	if is_mouse_button_down(MouseButton::Right) {
	    let m_pos:Vec2 = mouse_position().into();
	    let mouse_pos_vec = Vec2f::new(m_pos.x, m_pos.y);
	    let idx = self.find_index(&mouse_pos_vec);

	    self.wall_nodes.remove(&idx);
	}

	if self.sub_mode == SubMode::FindPath {
	    self.advance_a_star();
	}

	if self.sub_mode == SubMode::AddPoints {

 	    if self.points.len() == 0 {
		let p = Vec2f {
		    x: gen_range::<f32>(0.0, screen_width()),
		    y: gen_range::<f32>(0.0, screen_height()),
		};

		self.insert_point(&p);
	    } else {
		loop {
		    let open_index = self.find_open_index();
		    if open_index < 0 {
			self.sub_mode = SubMode::Show;
			self.make_voronoi();
			self.start_index = gen_range::<i32>(0, self.points.len() as i32);
			self.end_index = gen_range::<i32>(0, self.points.len() as i32);
			println!("start {} end {}", self.start_index, self.end_index);
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

	let mut dot_size = 2.5;

	if self.points.len() > 0 {
	    for i in 0 .. self.points.len() {
		let p = &self.points[i];

		let mut c = BLUE;

		if !self.points_open[i] {
		    if i as i32 == self.start_index {
			dot_size = 7.5;
			c = GREEN;
		    } else if i as i32 == self.end_index {
			dot_size = 7.5;
			c = RED;
		    } else {
			c = BLACK;
			dot_size = 2.5;
		    }
		};

		let i_i32 = i as i32;

		if self.open_set.contains(&i_i32) {
		    dot_size = 10.0;
		}

		if self.wall_nodes.contains(&i_i32) {
		    dot_size = 15.0;
		    c = PURPLE;
		}


		/*
		draw_rectangle(p.x,
		p.y,
		1.0, 1.0,
		c);
		 */

		/*
		draw_rectangle(p.x - dot_size * 0.5,
		p.y - dot_size * 0.5,
		dot_size,
		dot_size,
		c);
		 */


		draw_circle(p.x, p.y, dot_size * 0.5, c);
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

	for (node_index, prev_index) in &self.prev_index {
	    if *prev_index == -1 {
		continue;
	    }

	    let n = self.points[(*node_index) as usize];
	    let p = self.points[(*prev_index) as usize];

	    draw_line(n.x, n.y,
		      p.x, p.y,
		      1.5,
		      BLUE);
	}
    }

    fn as_any(&self) -> &dyn Any {
	self
    }
}
