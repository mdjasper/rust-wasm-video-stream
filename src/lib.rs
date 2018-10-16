#![feature(const_vec_new)]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn main(rgba: &[i32]) -> Vec<i32>{
//   let len: i32 = rgba.len() as i32;
//   let mut rgba_v = Vec::new();
//   for x in (1..len).step_by(4) {
//     let r = rgba[x as usize];
//     let g = rgba[(x+1) as usize];
//     let b = rgba[(x+2) as usize];
//     let average = (r+b+g)/3;
//     rgba_v.push(average);
//     rgba_v.push(average);
//     rgba_v.push(average);
//     rgba_v.push(255);
//   }
//   rgba_v
// }

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_i32(a: i32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// static mut rgba_v: Vec<i32> = vec![255; 640*480*4];
static mut rgba_v: Vec<i32> = Vec::new();

#[wasm_bindgen]
pub extern fn calculate(rgba: &[i32]){
  unsafe{
    console_error_panic_hook::set_once();
    let len: i32 = rgba.len() as i32;
    if rgba_v.len() == 0 {
      for x in 0..len {
        rgba_v.push(255)
      }
    }

    // let mut rgba_v = Vec::new();
    // let mut rgba_v: Vec<i32> = vec![255; 640*480*4];
    // log_i32(len);
    // log_i32(rgba_v.len() as i32);
    // log_u32((640*480*4)+1);
    for x in (0..len).step_by(4) {
      let r = rgba[x as usize];
      let g = rgba[(x+1) as usize];
      let b = rgba[(x+2) as usize];

      if b > g && b > r{
          rgba_v[(x) as usize] = r;
          rgba_v[(x+1) as usize] = g;
          rgba_v[(x+2) as usize] = b;
      } else {
          let average = (r+b+g)/3;
          rgba_v[(x) as usize] = average;
          rgba_v[(x+1) as usize] = average;
          rgba_v[(x+2) as usize] = average;
      }
    }
  }
}

#[wasm_bindgen]
pub extern fn get() -> &[i32] {
  unsafe { &rgba_v[..] }
}


// #[wasm_bindgen]
// pub fn main(rgba: &mut [i32]){
//   let len: i32 = rgba.len() as i32;
//   for x in (0..len).step_by(4) {
//     let r = rgba[x as usize];
//     let g = rgba[(x+1) as usize];
//     let b = rgba[(x+2) as usize];
//     let average = (r+b+g)/3;
//     rgba[x as usize] = average;
//     rgba[(x+1) as usize] = average;
//     rgba[(x+2) as usize] = average;
//   }
// }

// #[wasm_bindgen]
// pub fn redgreen(rgba: &mut [i32]){
//   let len: i32 = rgba.len() as i32;
//   for x in (0..len).step_by(4) {
//     let r = rgba[x as usize];
//     let g = rgba[(x+1) as usize];
//     let b = rgba[(x+2) as usize];
//     if b > r && b > g  {
//         rgba[x as usize] = b;
//         rgba[(x+1) as usize] = g;
//         rgba[(x+2) as usize] = r;
//     }
//     // else {
//     //     rgba[x as usize] = r;
//     //     rgba[(x+1) as usize] = g;
//     //     rgba[(x+2) as usize] = b;
//     // }

//   }
// }