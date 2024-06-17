use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn matrix_multiply(size: usize) -> Vec<f64> {
  let mut matrix_a = vec![vec![0.0; size]; size];
  let mut matrix_b = vec![vec![0.0; size]; size];
  let mut result_matrix = vec![vec![0.0; size]; size];

  // Initialize matrix A and matrix B with random numbers
  for i in 0..size {
      for j in 0..size {
          matrix_a[i][j] = rand::random();
          matrix_b[i][j] = rand::random();
      }
  }

  // Perform matrix multiplication
  for i in 0..size {
      for j in 0..size {
          let mut sum = 0.0;
          for k in 0..size {
              sum += matrix_a[i][k] * matrix_b[k][j];
          }
          result_matrix[i][j] = sum;
      }
  }

  // Flatten the result matrix to a single vector for easier handling in JavaScript
  let flat_result_matrix = result_matrix.iter().flat_map(|r| r.clone()).collect();

  flat_result_matrix
}

#[wasm_bindgen]
pub fn draw() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.rect(10.0, 10.0, 100.0, 100.0);
    context.fill();
    context.stroke();
}