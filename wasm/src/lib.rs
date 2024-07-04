use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;
use reqwest::Client;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageBuffer, Luma};
use js_sys::Array;
use reqwest::header::{HeaderMap, HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN};

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

#[wasm_bindgen]
pub async fn get_image_diff_array(url1: &str, url2: &str) -> Result<JsValue, JsValue> {
    // Create an HTTP client
    let client = Client::new();

    // Fetch the first image from the URL
    let resp1 = client.get(url1).send().await.map_err(|err| {
        console::log_1(&format!("Error fetching first image: {:?}", err).into());
        JsValue::from_str("Failed to fetch first image")
    })?;

    let bytes1 = resp1.bytes().await.map_err(|err| {
        console::log_1(&format!("Error reading first image bytes: {:?}", err).into());
        JsValue::from_str("Failed to read first image bytes")
    })?;

    // Fetch the second image from the URL
    let resp2 = client.get(url2).send().await.map_err(|err| {
        console::log_1(&format!("Error fetching second image: {:?}", err).into());
        JsValue::from_str("Failed to fetch second image")
    })?;

    let bytes2 = resp2.bytes().await.map_err(|err| {
        console::log_1(&format!("Error reading second image bytes: {:?}", err).into());
        JsValue::from_str("Failed to read second image bytes")
    })?;

    // Load the first image from the bytes
    let img1 = ImageReader::new(std::io::Cursor::new(bytes1))
        .with_guessed_format()
        .unwrap()
        .decode()
        .map_err(|err| {
            console::log_1(&format!("Error decoding first image: {:?}", err).into());
            JsValue::from_str("Failed to decode first image")
        })?;

    // Load the second image from the bytes
    let img2 = ImageReader::new(std::io::Cursor::new(bytes2))
        .with_guessed_format()
        .unwrap()
        .decode()
        .map_err(|err| {
            console::log_1(&format!("Error decoding second image: {:?}", err).into());
            JsValue::from_str("Failed to decode second image")
        })?;

    // Ensure both images are of the same size
    if img1.dimensions() != img2.dimensions() {
        return Err(JsValue::from_str("Images are not of the same size"));
    }

    // Convert the images to grayscale
    let gray_img1 = img1.to_luma8();
    let gray_img2 = img2.to_luma8();

    // Create an empty buffer for the difference image
    let mut diff_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(gray_img1.width(), gray_img1.height());

    // Compute the absolute difference between the pixel values
    for (x, y, pixel) in diff_img.enumerate_pixels_mut() {
        let p1 = gray_img1.get_pixel(x, y)[0] as i16;
        let p2 = gray_img2.get_pixel(x, y)[0] as i16;
        let diff = (p1 - p2).abs() as u8;
        *pixel = Luma([diff]);
    }

    // Convert the diff image to an array of grayscale values
    let mut diff_values = Array::new();
    for pixel in diff_img.pixels() {
        diff_values.push(&JsValue::from(pixel[0]));
    }

    Ok(diff_values.into())
}
