use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct System {
    pub l1: i32,
    pub l2: i32,
    pub temp: f64,
    pub steps: i64,
    pub dt: f64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Potential {
    pub name: String,
    pub eps: f64,
    pub sigma: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Object {
    pub m: i32,
    pub n: i32,
    // specify top left corner for initial position computation
    pub anchor: [i32; 2],
    // object dims on the lattice
    pub d_x: i32,
    pub d_y: i32,
    pub v_x: f64,
    pub v_y: f64,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub system: System,
    pub potential: Potential,
    pub objects: Vec<Object>,
}
