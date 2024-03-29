mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, app2!");
}

#[wasm_bindgen]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[wasm_bindgen]
pub fn trim(pixels_data: Vec<u8>, w: u32) -> Vec<u32> {


let l = pixels_data.len();
    
    let mut top = 99999;
    let mut left = 99999;
    let mut right = 99999;
    let mut bottom = 99999;
    let mut x;
    let mut y;


    
  for i in (0..l).step_by(4) {
    if pixels_data[i + 3] != 0 {
      x = (i as u32 / 4) % w;
      y = !!(i as u32 / 4 / w);

      if top == 99999 {
        top = y;
      }

      if left == 99999 {
        left = x;
      } else if x < left {
        left = x;
      }

      if right == 99999 {
        right = x;
      } else if right < x {
        right = x;
      }

      if bottom == 99999 {
        bottom = y;
      } else if bottom < y {
        bottom = y;
      }
    }
  } 

        return vec![top, left, right, bottom];
    }


    #[wasm_bindgen]
pub fn trim2(
    canvas: &OffscreenCanvas,
    ctx: &CanvasRenderingContext2d,
) -> Vec<u32> {
    //set_panic_hook();
    let width = canvas.width();
    let height = canvas.height();

    // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let _vec_data = data.data().to_vec();

let l = _vec_data.len();
    
    let mut top = 99999;
    let mut left = 99999;
    let mut right = 99999;
    let mut bottom = 99999;
    let mut x;
    let mut y;


    
  for i in (0..l).step_by(4) {
    if _vec_data[i + 3] != 0 {
      x = (i as u32 / 4) % width;
      y = !!(i as u32 / 4 / width);

      if top == 99999 {
        top = y;
      }

      if left == 99999 {
        left = x;
      } else if x < left {
        left = x;
      }

      if right == 99999 {
        right = x;
      } else if right < x {
        right = x;
      }

      if bottom == 99999 {
        bottom = y;
      } else if bottom < y {
        bottom = y;
      }
    }
  } 

        return vec![top, left, right, bottom];
}

 #[wasm_bindgen]
pub fn trim3(
    ctx: &CanvasRenderingContext2d, width: u32, height: u32
) -> Vec<u32> {
    let data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let _vec_data = data.data().to_vec();

let l = _vec_data.len();
    
    let mut top = 99999;
    let mut left = 99999;
    let mut right = 99999;
    let mut bottom = 99999;
    let mut x;
    let mut y;


    
  for i in (0..l).step_by(4) {
    if _vec_data[i + 3] != 0 {
      x = (i as u32 / 4) % width;
      y = !!(i as u32 / 4 / width);

      if top == 99999 {
        top = y;
      }

      if left == 99999 {
        left = x;
      } else if x < left {
        left = x;
      }

      if right == 99999 {
        right = x;
      } else if right < x {
        right = x;
      }

      if bottom == 99999 {
        bottom = y;
      } else if bottom < y {
        bottom = y;
      }
    }
  } 

        return vec![top, left, right, bottom];
}

   