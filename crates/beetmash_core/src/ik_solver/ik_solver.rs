use anyhow::Result;
use bevy::prelude::*;
use forky::prelude::*;
use std::f32::consts::PI;

const DEFAULT_EXTENT_MIN: f32 = 0.05;
const DEFAULT_EXTENT_MAX: f32 = 0.98;
const DEFAULT_LEN_A: f32 = 0.3;
const DEFAULT_LEN_B: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct IkOptions2d {
	pub target: Vec2,
	pub point_a: Vec2,
	// point_b: Vec2, // unused
	// point_c: Vec2,
	pub len_a: f32,
	pub len_b: f32,
	pub extent_min: f32,
	pub extent_max: f32,
}
impl Default for IkOptions2d {
	fn default() -> Self {
		Self {
			target: Vec2::new(1., 0.),
			point_a: Vec2::ZERO,
			// point_b: Vec2::new(DEFAULT_LEN_A, 0.0),
			// point_c: Vec2::new(DEFAULT_LEN_A, DEFAULT_LEN_B),
			len_a: DEFAULT_LEN_A,
			len_b: DEFAULT_LEN_B,
			extent_min: DEFAULT_EXTENT_MIN,
			extent_max: DEFAULT_EXTENT_MAX,
		}
	}
}

#[derive(Debug, Clone)]
pub struct IkOptions3d {
	pub origin: Vec3,
	pub target: Vec3,
	pub up: Vec3,
	pub len_a: f32,
	pub len_b: f32,
	// minimum angle, scalar from 0 to 1
	pub extent_min: f32,
	// maximum angle, scalar from 0 to 1
	pub extent_max: f32,
}


impl Default for IkOptions3d {
	fn default() -> Self {
		Self {
			origin: Vec3::ZERO,
			target: Vec3::new(1.0, 0.0, 0.0),
			up: Vec3::UP,
			len_a: DEFAULT_LEN_A,
			len_b: DEFAULT_LEN_B,
			extent_min: DEFAULT_EXTENT_MIN,
			extent_max: DEFAULT_EXTENT_MAX,
		}
	}
}

pub fn solve_ik_3d(
	IkOptions3d {
		origin,
		target,
		up,
		len_a,
		len_b,
		extent_min,
		extent_max,
	}: IkOptions3d,
) -> Result<(Mat4, Mat4, Mat4)> {
	let right = (target - origin).normalize();
	let forward = Vec3::cross(right, up).normalize();
	if forward.is_nan() {
		anyhow::bail!("forward is nan");
	}
	let local_to_world = Mat4::look_to_rh(origin, forward, up);
	let world_to_local = local_to_world.inverse();
	// println!("point_a: {:?}, point_b: {:?}", forward, right);

	let point_a = Mat4::transform_point(&world_to_local, origin);
	// let point_b = Vec3::ZERO;
	// let point_c = Vec3::ZERO;
	let local_target = Mat4::transform_point(&world_to_local, target);

	let (point_b, point_c) = solve_ik_2d(&IkOptions2d {
		target: Vec2::new(local_target.x, local_target.y),
		point_a: Vec2::new(point_a.x, point_a.y),
		// point_b: Vec2::new(point_b.x, point_b.y),
		// point_c: Vec2::new(point_c.x, point_c.y),
		len_a,
		len_b,
		extent_min,
		extent_max,
	});


	let world_a = Mat4::transform_point(&local_to_world, point_a);
	let world_b = Mat4::transform_point(&local_to_world, to_vec_3(point_b));
	let world_c = Mat4::transform_point(&local_to_world, to_vec_3(point_c));

	let fwd_a = world_b - world_a;
	let fwd_b = world_c - world_b;

	let mat_a = Mat4::look_to_rh(world_a, fwd_a, up);
	let mat_b = Mat4::look_to_rh(world_b, fwd_b, up);
	let mat_c = Mat4::look_to_rh(world_c, fwd_b, up);
	Ok((mat_a, mat_b, mat_c))
}


fn to_vec_3(vec: Vec2) -> Vec3 { Vec3::new(vec.x, vec.y, 0.0) }

pub fn solve_ik_2d(
	IkOptions2d {
		target,
		point_a,
		len_a,
		len_b,
		extent_min,
		extent_max,
		..
	}: &IkOptions2d,
) -> (Vec2, Vec2) {
	let x = target.x;
	let y = target.y;
	let c2 = *len_a;
	let c3 = *len_b;
	let max_length = c2 + c3;

	let c1_raw = pythagoras_c(x, y);
	let c1_max = c1_raw.min(c2 + c3);
	let c1 = c1_max.clamp(max_length * *extent_min, max_length * *extent_max);

	let b1 = trig_toa_theta(x, y);
	let b4 = law_of_cosines_abc(c2, c3, c1);

	let a4 = law_of_sines_a(c2, c3, b4);
	let c4 = PI - a4 - b4;
	let b2 = b1 - b4;
	let b3 = PI - c4;

	let point_b = set_next_point_from_angle(&point_a, b2, *len_a);
	let point_c = set_next_point_from_angle(&point_b, b2 + b3, *len_b);
	(point_b, point_c)
}

fn set_next_point_from_angle(
	current_point: &Vec2,
	theta: f32,
	len: f32,
) -> Vec2 {
	let polar = polar_to_cartesian(theta, len);
	Vec2 {
		x: current_point.x + polar.x,
		y: current_point.y + polar.y,
	}
}

fn pythagoras_c(a: f32, b: f32) -> f32 { (a * a + b * b).sqrt() }

fn trig_toa_theta(x: f32, y: f32) -> f32 { y.atan2(x) }

fn law_of_cosines_abc(len_a: f32, len_b: f32, len_c: f32) -> f32 {
	((len_b * len_b - (len_a * len_a + len_c * len_c)) / (-2.0 * len_a * len_c))
		.acos()
}

fn law_of_sines_a(len_a: f32, len_b: f32, theta_b: f32) -> f32 {
	((theta_b.sin() / len_b) * len_a).asin()
}

fn polar_to_cartesian(theta: f32, len: f32) -> Vec2 {
	Vec2::new(theta.cos() * len, theta.sin() * len)
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use bevy::prelude::*;
	use std::f32::consts::TAU;

	#[test]
	fn works() -> Result<()> {
		for i in 0..8 {
			let theta = (i as f32 / 8.) * TAU;
			let x = theta.cos();
			let y = theta.sin();

			let (_mat1, _mat2, mat3) = solve_ik_3d(IkOptions3d {
				target: Vec3::new(x, 0., y),
				// target: Vec3::new(0., 100., 0.1),
				len_a: 10.,
				len_b: 10.,
				extent_min: 0.,
				extent_max: 1.,
				..default()
			})?;

			// let (_, _, pos1) = mat1.to_scale_rotation_translation();
			// let (_, _, pos2) = mat2.to_scale_rotation_translation();
			let (_, _, pos3) = mat3.to_scale_rotation_translation();

			// println!(
			// 	"IK solved to positions: {:?}, {:?}, {:?}",
			// 	pos1, pos2, pos3
			// );
			println!("IK solved to positions: {:?}", pos3);
		}
		// const

		// const origin = Vec3.scaleSet(Vec3.One(), -1)
		// const target = Vec3.One()
		// const up = Vec3.Up()
		// const m1 = Matrix()
		// const m2 = Matrix()
		// const m3 = Matrix()
		// const solver = IKSolver({ origin, target, up })
		// solver.update(m1, m2, m3)

		// expect(Matrix.position(m1)).toBeVec3(origin)

		Ok(())
	}
}
