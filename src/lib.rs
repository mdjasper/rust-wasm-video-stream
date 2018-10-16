#![feature(const_vec_new)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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