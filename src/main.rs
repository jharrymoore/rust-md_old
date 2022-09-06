use std::{fs::File, io::BufReader};

use clap::Parser;

use rust_md::{compute_forces, compute_statistics, config::Config, init_system, update_positions};
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    config: String,
}

fn main() {
    // Initialise simulation: parse topology, assign + scale velocities
    let args = Args::parse();
    // main MD loop
    let file = File::open(args.config).expect("path to config file does not exist");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();

    //  compute ensemble properties
    let mut particles = init_system(config.clone());
    println!("Initialzed system of {} particles", particles.len());
	dbg!(particles[0]);

    for t in 0..config.system.steps {
        // particles.iter().for_each(|p| p.update_pos(args.dt));
        // this is all done by mutable reference
        update_positions(&mut particles, &config.system.dt);

        compute_forces(&mut particles, &config.potential.name);
        for p in particles.iter_mut() {
            p.update_vel(&config.system.dt)
        };
        compute_statistics(&particles, t as usize);
		dbg!(particles[0]);
    }
}
