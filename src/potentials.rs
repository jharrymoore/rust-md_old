
pub trait Potential {
	// compute energies and forces for a given set of atomic positions
	fn energy(&self, x: f64) -> f64;
	fn force(&self, x: f64) -> f64;
}




// define some basic forcefield potentials
pub struct LennardJones {
	pub sigma: f64,
	pub epsilon: f64,
}

impl Potential for LennardJones {
	fn energy(&self, r: f64) -> f64 {
		let s6 = f64::powi(self.sigma / r, 6);
		4. * self.epsilon * ( f64::powi(s6, 2) - s6)
	}
	fn force(&self, r: f64) -> f64 {
		// derivative of the potential
		let s6 = f64::powi(self.sigma / r, 6);
		24. * self.epsilon * (s6 - 2. * f64::powi(s6, 2)) / r
	}
}

// for a nn potential, the only constraint is that we just be able to evaluate the force/energies with an appropriate implementation