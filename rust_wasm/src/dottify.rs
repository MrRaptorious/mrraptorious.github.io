
use image::{io::Reader as ImageReader, GenericImageView, ImageFormat, Rgba};
use imageproc::drawing::draw_filled_circle_mut;
use std::{cmp::max, io::Cursor};
use wasm_bindgen::prelude::*;
use web_sys::console;
use rayon::prelude::*;

#[wasm_bindgen]
pub fn init_logging() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    log::info!("Rust WASM logger initialized!");
}

#[wasm_bindgen(js_name = gray_image)]
pub fn gray_image(image_data: &[u8], output_format_str: String) -> Result<Vec<u8>, JsValue> {
    console::log_1(&"Starting image processing in Rust...".into());

    let format = match output_format_str.as_str() {
        "png" => ImageFormat::Png,
        "jpeg" => ImageFormat::Jpeg,
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unsupported output format: {output_format_str}"
            )));
        }
    };

    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|e| JsValue::from_str(&format!("Failed to guess image format: {e}")))?
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image: {e}")))?;

    console::log_1(&"Image decoded. Converting to grayscale...".into());

    let gray_img = img.grayscale();

    let mut processed_bytes = Vec::new();
    gray_img
        .write_to(&mut Cursor::new(&mut processed_bytes), format)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to encode image to {output_format_str}: {e}"
            ))
        })?;

    console::log_1(&"Image processing complete.".into());

    Ok(processed_bytes)
}

#[wasm_bindgen(js_name = dottify)]
pub fn dottify(image_data: &[u8], output_format_str: String, dot_size: u32) -> Result<Vec<u8>, JsValue> {
    let format = match output_format_str.as_str() {
        "png" => ImageFormat::Png,
        "jpeg" => ImageFormat::Jpeg,
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unsupported output format: {output_format_str}"
            )));
        }
    };

    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|e| JsValue::from_str(&format!("Failed to guess image format: {e}")))?
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image: {e}")))?;

    // todo here dottify

    // todo: guard min size
    // create image
    let image_dotsize = dot_size;
    let inter_dot_padding = 5;

    let num_dots_x: u32 = img.width()/(image_dotsize + 1);
    let num_dots_y: u32 = img.height()/(image_dotsize + 1);
    let dot_grid_size = image_dotsize + 1;

let dots: Vec<Rgba<u8>> = (0..(num_dots_x * num_dots_y)) // Erzeugt einen Iterator über alle Dot-Indizes
    .into_par_iter() // Macht den Iterator parallel
    .map(|index| {
        let x_dot = index % num_dots_x;
        let y_dot = index / num_dots_x;

        let mut sum_r: u32 = 0;
        let mut sum_g: u32 = 0;
        let mut sum_b: u32 = 0;

            // bild ausschnitt für avg color

            let view_width :u32 = image_dotsize;
            let view_heigt :u32 = image_dotsize;
            let pixel_count :u32 = view_heigt * view_width;

            let window_x : u32 = max(0, ((x_dot*dot_grid_size) as i32)-((view_width as i32)/2)) as u32;
            let window_y : u32 = max(0, ((y_dot*dot_grid_size)as i32)-((view_heigt as i32)/2)) as u32;

            for (_, _, pixel) in img.view(window_x,window_y , view_heigt, view_width).pixels() {
                let Rgba([r, g, b, _]) = pixel;

                sum_r += r as u32;
                sum_g += g as u32;
                sum_b += b as u32;
            }

            sum_r /= pixel_count;
            sum_g /= pixel_count;
            sum_b /= pixel_count;

            Rgba([sum_r as u8, sum_g as u8, sum_b as u8, 255])
        }).collect();


    // neues bild erstellen

    let new_width = num_dots_x * (image_dotsize + inter_dot_padding);
    let new_height = num_dots_y * (image_dotsize + inter_dot_padding);

    let mut dotted_image = image::RgbaImage::from_pixel(new_width, new_height,Rgba([0,0,0,255]));

    for (index, dot) in dots.iter().enumerate() {
        let x = index as u32 % num_dots_x;
        let y = index as u32 / num_dots_x;

        // anchro top left
        let offset = image_dotsize / 2;
        let x_center = x * (image_dotsize + inter_dot_padding) + offset ;
        let y_center = y * (image_dotsize + inter_dot_padding) + offset;

        draw_filled_circle_mut(&mut dotted_image, (x_center as i32,y_center as i32) , (image_dotsize as i32) /2, *dot);
    }

    let mut processed_bytes = Vec::new();
    dotted_image
        .write_to(&mut Cursor::new(&mut processed_bytes), format)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to encode image to {output_format_str}: {e}"
            ))
        })?;

    Ok(processed_bytes)
}
