#![feature(const_vec_new)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(rgba: &[i32]) -> Vec<i32> {
  let len: i32 = rgba.len() as i32;

  // let mut rgba_v = Vec::new();
  let mut rgba_v: Vec<i32> = vec![255; 640*480*4];
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
  rgba_v
}