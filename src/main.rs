use image::{ImageBuffer, Rgb};

use circle_packing::Circle;
use circle_packing::Drawable;



fn main() {
    println!("Hello, world!");

    let output = "a.png";

    let (width, height) = (1000, 1000);

    let c1 = Circle {center: (500.0, 500.0), rad: 250.0};

    // ImageBuffer can generate images by iteration over pixels
    // (x, y) is the pixel and the return of the closure is the Rgb color value
    let imgbuf = ImageBuffer::from_fn(width, height, |row, col| {
        // RgbImage generates the image from u8 slices so we have to convert c
        let r = (0.5 * u8::MAX as f32) as u8;
        let g = (1.0 * u8::MAX as f32) as u8;
        let b = (0.7 * u8::MAX as f32) as u8;



        if c1.is_colored(row, col) {
            return Rgb([0,0,0]) 
        }

        
        Rgb([r, g, b])
    });

    imgbuf.save(output).unwrap();
}
