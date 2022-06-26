// Bridson's algorithm for blue noise
// https://www.jasondavies.com/poisson-disc/
// with the speedup
// http://extremelearning.com.au/an-improved-version-of-bridsons-algorithm-n-for-poisson-disc-sampling/

use bigdicegames.math.geom;

f64 epsilon = 0.0000001;

fn sampleHighDensity(Vec2f v, i32 k) {
    f32 seed = Math.random0to1();
    
    for (j = 0; j < k; ++j) {
	f64 theta = 2 * Math.PI * (seed + 1.0 * j / k);
	r = rInner + epsilon;
	x = v.x + r * Math.cos(theta);
	y = v.y + r * Math.sin(theta);

	Vec2f outVec = Vec2f.new(x,y);
	yield outVec;
    }
}

fn generatePointFromAnnulus(Vec2f center, f32 innerRadius) {
    f32 outerRadius = innerRadius * 2.0;

    // find point in normalized annulus
    while (true) {
	f32 x = Math.random0to1() * 2.0 - 1.0;
	f32 y = Math.random0to1() * 2.0 - 1.0;

	f32 r = Math.sqrt(x*x + y*y);
	if ((r <= 1.0) && (r >= 0.5)) {
	    // scale and offset normalized to desired annulus
	    f32 sx = x * outerRadius;
	    f32 sy = y * outerRadius;

	    f32 ox = center.x + sx;
	    f32 oy = center.y + sy;

	    return Vec2f.new(ox, oy);
	}
    }
}


// original Bridson approach
fn sampleOriginal(Vec2f v, i32 k, f32 r) {

    for (i32 j = 0; j < k; ++j) {
	Vec2f sample = generatePointFromAnnulus(v, r);
	
    }
}


fn isPointTooClose(Vec2f v) {
    f32 ox = v.x - bbox.left;
    f32 oy = v.y - bbox.top;

    i32 gx = floor(ox / grid_size);
    i32 gy = floor(oy / grid_size);

    for (i32 si = -neighbor_steps; si <= neighbor_steps; ++si) {
	i32 dx = gx + si;

	if ((dx < 0) ||
	    (dx >= grid_x)) {
	    continue;
	}
	
	for (i32 sj = -neighbor_steps; sj <= neighbor_steps; ++sj) {
	    i32 dy = gy + sj;

	    if ((dy < 0) ||
		(dy >= grid_y)) {
		continue;
	    }

	    if (grid[dy * grid_x + dx] != -1)
	    {
		return true;
	    }
	}
    }
    return false;
}
