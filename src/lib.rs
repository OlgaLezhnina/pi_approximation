use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use rand_distr::{Distribution, Uniform};
use js_sys::{Float64Array, Array, ArrayTuple, Map};
use std::mem::forget;

#[wasm_bindgen]
pub fn approximate_pi() -> ArrayTuple<(Float64Array, Float64Array, js_sys::Array)> {
    let n = 10000;
    let mut data = Vec::with_capacity(n);
    let mut labels = Vec::with_capacity(n);
    let dots = Array::new();
    let mut rng = rand::rng();
    let x =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");
    let y =
        Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution: invalid range");

    let mut count: i32 = 1;
    let mut points_in: i32 = 0;
    while count <= n as i32 {
        let x_coord = x.sample(&mut rng);
        let y_coord = y.sample(&mut rng);
        let dot = Map::new();
        dot.set(&JsValue::from_str(&"x"), &JsValue::from_f64(x_coord));
        dot.set(&JsValue::from_str(&"y"), &JsValue::from_f64(y_coord));
        let squared: f64 = x_coord*x_coord + y_coord*y_coord;   
        let in_circle: bool = squared <= 1.0; 
        if in_circle {
            points_in += 1;
        };
        let pi_process = points_in as f64 / count as f64 *4.0;
        data.push(pi_process);
        labels.push(count as f64);
        dots.push(&dot);
        count += 1;
    }
    unsafe {
        let arr = Float64Array::view(&data);
        let lab = Float64Array::view(&labels);
        forget(data);
        forget(labels);
        let all: ArrayTuple<(Float64Array, Float64Array, js_sys::Array)> = ArrayTuple::new3(&arr,&lab, &dots);
        forget(arr);
        forget(lab);
        return all;
    }
}
