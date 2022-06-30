// screens/square_nav.rs

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

	// now compare based on remaining heuristic
	if self.heuristic_remaining > other.heuristic_remaining {
	    return Ordering::Less;
	} else if self.heuristic_remaining < other.heuristic_remaining {
	    return Ordering::Greater;
	}

	if self.distance_travelled > other.distance_travelled {
	    return Ordering::Greater;
	} else if self.distance_travelled < other.distance_travelled {
	    return Ordering::Less;
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

pub struct SquareNavScreen
{
    time_elapsed: f32,
    is_complete_flag: bool,
    points: Vec<Vec2f>,

    num_x: i32,
    num_y: i32,
    space_count: i32,

    space_width: f32,

    sub_mode: SubMode,

    start_index: i32,
    end_index: i32,

    a_star_nodes: BinaryHeap<AStarRecord>,

    found_distances: HashMap<i32, f32>,
    open_set: HashSet<i32>,

    prev_index: HashMap<i32, i32>,

    is_8_way: bool,
}

impl SquareNavScreen {
    pub fn new() -> SquareNavScreen {
	let space_width: f32 = 20.0;

	let num_x = (screen_width() / space_width).ceil() as i32;
	let num_y = (screen_height() / space_width).ceil() as i32;

	let space_count = num_x * num_y;

	println!("num x: {}", num_x);
	println!("num y: {}", num_y);
	println!("num spaces: {}", space_count);

	let mut point_list:Vec<Vec2f> = Vec::new();

	for y in 0 .. num_y {
	    for x in 0..num_x {
		point_list.push(Vec2f {
		    x: (x as f32 + 0.5) * space_width,
		    y: (y as f32 + 0.5) * space_width,
		});
	    }
	}

	let seconds_since_start = (macroquad::time::get_time() * 1234.5) as u64;
	let m_pos:Vec2 = mouse_position().into();

	macroquad::rand::srand(seconds_since_start +
			       m_pos.x as u64 * 1234 +
			       m_pos.y as u64 * 5678);


	SquareNavScreen {
	    is_complete_flag: false,
	    time_elapsed: 0.0,
	    points: point_list,
	    num_x: num_x,
	    num_y: num_y,
	    space_count: space_count,
	    space_width: space_width,
	    sub_mode: SubMode::AddPoints,
	    start_index: -1,
	    end_index: -1,
	    a_star_nodes: BinaryHeap::new(),
	    found_distances: HashMap::new(),
	    open_set: HashSet::new(),
	    prev_index: HashMap::new(),
	    is_8_way: true,
	}
    }

    pub fn reset(&mut self) {
	self.time_elapsed = 0.0;
	self.is_complete_flag = false;
	self.sub_mode = SubMode::AddPoints;
	self.start_index = -1;
	self.end_index = -1;
	self.prev_index.clear();
	self.open_set.clear();
	self.found_distances.clear();
	self.a_star_nodes.clear();
    }

    fn point_in_box(&self, p: &Vec2f) -> bool {
	p.x >= 0.0 &&
	    p.x < screen_width() &&
	    p.y >= 0.0 &&
	    p.y < screen_height()
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

    fn calc_heuristic_by_indices(&self, a:i32, b:i32, si:i32) -> f32 {
	//(self.points[a as usize] - self.points[b as usize]).mag()

	let ap = self.points[a as usize];
	let bp = self.points[b as usize];
	let start_point = self.points[si as usize];

	let ax = ap.x;
	let ay = ap.y;
	let bx = bp.x;
	let by = bp.y;
	let sx = start_point.x;
	let sy = start_point.y;

	// Amit wins again
	// http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#breaking-ties

	let dx1 = ax - bx;
	let dy1 = ay - by;
	let dx2 = sx - bx;
	let dy2 = sy - by;
	let cross = (dx1*dy2 - dx2*dy1).abs();

	let dx = (bx-ax).abs();
	let dy = (by-ay).abs();

	if (!self.is_8_way) {
	    return dx + dy + cross * 0.01;
	}

	let mut max_dim:f32 = 0.0;
	let mut min_dim:f32 = 0.0;

	if dx < dy {
	    max_dim = dy;
	    min_dim = dx;
	} else {
	    max_dim = dx;
	    min_dim = dy;
	}

	if (min_dim < 1.0) {
	    return max_dim;
	}

	let straight_leg = max_dim - min_dim;
	let diag_leg = min_dim * 2.0_f32.sqrt();

	straight_leg + diag_leg + cross*0.01
    }

    fn get_neighbor_space_indices(&self, i:i32) -> Vec<i32> {
	let mut out_vec = Vec::<i32>::new();

	let x = i % self.num_x;
	let y = (i - x) / self.num_x;

	//println!("from {} x: {} y: {}", i, x, y);

	if (x > 0) {
	    out_vec.push(self.space_coord_to_index(x-1, y));
	}
	if (x < self.num_x - 1) {
	    out_vec.push(self.space_coord_to_index(x+1, y));
	}

	if (y > 0) {
	    out_vec.push(self.space_coord_to_index(x, y-1));
	}
	if (y < self.num_y - 1) {
	    out_vec.push(self.space_coord_to_index(x, y+1));
	}

	if (self.is_8_way) {
	    if (x > 0 && y > 0) {
		out_vec.push(self.space_coord_to_index(x-1, y-1));
	    }

	    if (x < self.num_x - 1 && y > 0)  {
		out_vec.push(self.space_coord_to_index(x+1, y-1));
	    }

	    if (x > 0 && y < self.num_y - 1) {
		out_vec.push(self.space_coord_to_index(x-1, y+1));
	    }

	    if (x < self.num_x - 1 && y < self.num_y - 1) {
		out_vec.push(self.space_coord_to_index(x+1, y+1));
	    }
	}

	//println!("neighbors {:?}", out_vec);

	out_vec
    }

    fn space_coord_to_index(&self, x: i32, y: i32) -> i32 {
	x + y * self.num_x
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
			self.start_index, self.end_index, self.start_index);

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
		let neighbor_space_list = self.get_neighbor_space_indices(n.index);

		for neighbor_index in neighbor_space_list {
		    let neighbor_point = self.points[neighbor_index as usize];
		    let this_step_dist = (neighbor_point - n.point).mag();
		    let new_elapsed_dist = n.distance_travelled + this_step_dist;

		    let mut insert_node = true;

		    let neighbor_index_i32 = neighbor_index as i32;
		    if self.found_distances.contains_key(&neighbor_index_i32) {
			insert_node = self.found_distances[&neighbor_index_i32] > new_elapsed_dist;
		    }

		    if insert_node {
			let new_h = self.calc_heuristic_by_indices(
			    neighbor_index_i32, self.end_index, self.start_index);

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
	    }
	}
    }
}

#[async_trait]
impl Screen for SquareNavScreen {
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

	if self.sub_mode == SubMode::FindPath {
	    self.advance_a_star();
	}

	if self.sub_mode == SubMode::AddPoints {
	    self.start_index = gen_range::<i32>(0, self.points.len() as i32);
	    self.end_index = gen_range::<i32>(0, self.points.len() as i32);
	    println!("start {} end {}", self.start_index, self.end_index);

	    self.sub_mode = SubMode::Show;
	}
    }

    fn is_complete(&self) -> bool {
	self.is_complete_flag
    }

    fn render(&self, _tex_mgr: &TextureMgr) {
	clear_background(GRAY);

	let mut dot_size = 2.5;

	if self.points.len() > 0 {
	    for i in 0 .. self.points.len() {
		let p = &self.points[i];

		let mut c = BLUE;

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

		let i_i32 = i as i32;

		if self.open_set.contains(&i_i32) {
		    dot_size = 10.0;
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

		/*
		let label = format!("{}", i);
		draw_text(&label, p.x - 12.0, p.y - 12.0,
			  12.0, BLACK);*/

	    }
	}

	for x in 0 .. self.num_x {
	    draw_line(x as f32 * self.space_width,
		      0.0,
		      x as f32 * self.space_width,
		      screen_height() as f32,
		      1.5,
		      BLACK);
	}

	for y in 0 .. self.num_y {
	    draw_line(0.0,
		      y as f32 * self.space_width,
		      screen_width() as f32,
		      y as f32 * self.space_width,
		      1.5,
		      BLACK);
	}

	for (node_index, prev_index) in &self.prev_index {
	    if (*prev_index == -1) {
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
