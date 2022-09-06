use std::ops::{Add, AddAssign, Div, Mul, Sub};

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
}
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64) -> Vec3 {
        Vec3 { x, y }
    }
    pub fn distance_to(&self, other: Vec3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn initialize_random(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen();
        self.y = rng.gen();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Atom {
    x: Vec3,
    v: Vec3,
    f: Vec3,
    f_old: Vec3,
    pub m: i32,
}

impl Atom {
    pub fn new(m: i32, x: f64, y: f64, v_x: f64, v_y: f64) -> Atom {
        let a = Atom {
            x: Vec3 { x, y },
            v: Vec3 {
                x: v_x,
                y: v_y,
            },
            f: Vec3 { x: 0., y: 0. },
            f_old: Vec3 { x: 0., y: 0. },
            m: m,
        };
        a
    }
    pub fn initialize_random_vel(&mut self) {
        self.v.initialize_random();
    }

    pub fn update_pos(&mut self, dt: &f64) {
        //  update atom position according to velocity verlet, central difference approximation
		let shift = self.v * *dt + (self.f * dt.powi(2)) / (2. * self.m as f64);
        self.x += shift;
    }
    pub fn update_vel(&mut self, dt: &f64) {
        let a = 0.5 * dt / self.m as f64;
        self.v += (self.f + self.f_old) * a;
    }
    pub fn compute_force_lj(&mut self, a_j: Atom, r_cut: f64) {
        // compute the LJ force between self and atom J, add to force vector
        let r = self.x.distance_to(a_j.x);
        match r < r_cut && r > 0. {
            true => {
                let r2i = 1. / r;
                let r6i = r2i.powi(3);
                let f = 48. * r2i * r6i * (r6i - 0.5);
                self.f = self.f + (self.x - a_j.x) * f;
            }
            false => {
                let f = 0.;
                self.f = self.f + (self.x - a_j.x) * f;
            }
        };
    }
    pub fn compute_force_grav(&mut self, a_j: Atom) {
        let r = self.x.distance_to(a_j.x).sqrt();
        let f = self.m as f64 * a_j.m as f64 / (r.sqrt() * r);
        self.f = self.f + (self.x - a_j.x) * f;
    }
    pub fn reset_forces(&mut self) {
		self.f_old = self.f;
        self.f = Vec3 { x: 0., y: 0. };
    }
    pub fn set_force(&mut self, f: Vec3) {
        self.f = f
    }
    pub fn get_velocities(&self) -> Vec3 {
        self.v.clone()
    }
	pub fn get_forces(&self) -> Vec3 {
		self.f.clone()
	}
}
