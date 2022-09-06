use atom::Atom;
use config::Config;
use rayon::prelude::*;

use crate::atom::Vec3;
pub mod atom;
pub mod config;
pub mod integrators;

pub fn init_system(config: Config) -> Vec<Atom> {
    // initialise the system, consume the config
    let mut atoms = vec![];
    for obj in config.objects {
        for x in obj.anchor[0]..obj.anchor[0] + obj.d_x {
            for y in obj.anchor[0]..obj.anchor[0] + obj.d_x {
                // compute position on the lattice
                let x_pos = x as f64 * 2.0_f64.powf(1. / 6.) * config.potential.sigma;
                let y_pos = y as f64 * 2.0_f64.powf(1. / 6.) * config.potential.sigma;
                atoms.push(Atom::new(obj.m, x_pos, y_pos, obj.v_x, obj.v_y))
            }
        }
    }
    atoms
}

pub fn compute_forces(particles: &mut Vec<Atom>, potential: &String) {
    // reset forces
    for p in particles.iter_mut() {
        p.reset_forces()
    }
    // compute forces using whatever potential specified
    // we loop over all particles, manipulate the objects directly in the outer loop, and take shared reference in the innter loop, they both have to be immutable
    //
    let len_particles = particles.len();

    let particles_ref = particles.clone();
    particles.into_par_iter().enumerate().for_each(|(i, a_i)| {
        for j in i + 1..len_particles {
            match potential.as_str() {
                "lj" => {
                    // dbg!(i, j);
                    a_i.compute_force_lj(particles_ref[j], 5.);
                }
                "gravitational" => {
                    a_i.compute_force_grav(particles_ref[j]);
                }
                _ => panic!("Force type {} not recognised", potential),
            }
        }
		// no forces are being returned as zero
		// assert!(a_i.get_forces() != Vec3::new( 0.0, 0.0))
    });
}

pub fn update_positions(particles: &mut Vec<Atom>, dt: &f64) {
    for p in particles.iter_mut() {
        p.update_pos(dt)
    }
}

pub fn compute_statistics(particles: &Vec<Atom>, t: usize) {
    let mut e: f64 = 0.;
    for p in particles {
        let v2: f64 = p.get_velocities().magnitude().powi(2);
        e += 0.5 * p.m as f64 * v2;
    }
    println!("Kinetic energy = {} at time {}", e, t);
}
