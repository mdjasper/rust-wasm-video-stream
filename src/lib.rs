#![feature(const_vec_new)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(rgba: &mut [i32]) {
  let len: i32 = rgba.len() as i32;

  for x in (0..len).step_by(4) {
    let r = rgba[x as usize];
    let g = rgba[(x+1) as usize];
    let b = rgba[(x+2) as usize];

    if b > g && b > r{
        rgba[(x) as usize] = r;
        rgba[(x+1) as usize] = g;
        rgba[(x+2) as usize] = b;
    } else {
        let average = (r+b+g)/3;
        rgba[(x) as usize] = average;
        rgba[(x+1) as usize] = average;
        rgba[(x+2) as usize] = average;
    }
  }
}