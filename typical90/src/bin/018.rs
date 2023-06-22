// use nalgebra::Vector3;
use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q]
    }

    for s in &e {
        let (dy, dz) = calc_pos(s, l / 2.0, t);
        let dx = (x * x + (y - dy).powi(2)).sqrt();
        println!("{}", dz.atan2(dx) * 180.0 / PI);

        // let oa: Vector3<f64> = Vector3::from([x, y, 0.0]);
        // let ob: Vector3<f64> = calc_ob(s, l / 2.0, t);
        // let ba: Vector3<f64> = &oa - &ob;
        // let ac: Vector3<f64> = Vector3::from([0.0, y - ob.y, ob.z]);
        // let angle = ba.angle(&ac) * 180.0 / PI / 2.0;
        // println!("{}", angle);
    }
}

fn calc_pos(e: &f64, l: f64, t: f64) -> (f64, f64) {
    let rad = 2.0 * PI * e / t;
    let dy = -l * rad.sin();
    let dz = l * (1.0 - rad.cos());

    (dy, dz)
}

// fn calc_ob(e: &f64, l: f64, t: f64) -> Vector3<f64> {
//     let rad = 2.0 * PI * e / t;
//     let dy = -l * rad.sin();
//     let dz = l * (1.0 - rad.cos());

//     Vector3::from([0.0, dy, dz])
// }
