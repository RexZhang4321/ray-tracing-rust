use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod color_utils;

use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    let mut buffer = File::create("image.ppm")?;

    write!(buffer, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        // println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Vec3 {
                x: i as f32 / (image_width - 1) as f32,
                y: j as f32 / (image_height - 1) as f32,
                z: 0.25
            };

            color_utils::write_color(&buffer, &color)?;
        }
    }

    Ok(())
}
