use wasm_bindgen::prelude::*;
use rand_distr::{Distribution, Uniform};
use js_sys::{Float64Array, ArrayTuple};

#[wasm_bindgen]
pub fn approximate_pi() -> ArrayTuple<(Float64Array, Float64Array)> {
    let N = 100;
    let mut data = Vec::with_capacity(N);
    let mut labels = Vec::with_capacity(N);
    let mut rng = rand::rng();
    let x =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");
    let y =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");

    let mut count: i32 = 0;
    let mut points_in: i32 = 0;
    let mut points_out: i32 = 0;
    
    while count < N as i32 {
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
        data.push(pi_process);
        labels.push((count+1) as f64);
        count += 1;
    }
    // let pi_approx = points_in as f64/(points_in as f64 + points_out as f64)*4.0;
    unsafe {
        let arr = Float64Array::view(&data);
        let lab = Float64Array::view(&labels);
        let all = ArrayTuple::new2(&arr,&lab);
        return all;
    }
}
