use wasm_bindgen::prelude::*;
use rand_distr::{Distribution, Uniform};
use js_sys::Float64Array;

#[wasm_bindgen]
pub fn approximate_pi(arr: &Float64Array, lab: &Float64Array) -> f64 {
    let data = unsafe { arr.view() };
    let labels = unsafe { lab.view() };
    let mut rng = rand::rng();
    let x =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");
    let y =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");

    let mut count: i32 = 0;
    let mut points_in: i32 = 0;
    let mut points_out: i32 = 0;
    
    while count <= 10 {
        let x_coord = x.sample(&mut rng);
        let y_coord = y.sample(&mut rng);
        let squared: f64 = x_coord*x_coord + y_coord*y_coord;   
        let in_circle: bool = squared <= 1.0; 
        if in_circle {
            points_in += 1;
        } else {
            points_out +=1;
        };
        let pi_process = points_in as f64/(points_in as f64 + points_out as f64)*4.0;
        data[count] = pi_process;
        labels[count] = count+1;
        count += 1;
    }
    let pi_approx = points_in as f64/(points_in as f64 + points_out as f64)*4.0;
    return pi_approx;
}
