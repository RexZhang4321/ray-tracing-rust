use std::fs::File;
use std::io::prelude::*;
use crate::vec3::Vec3;

pub fn write_color(mut buffer: &File, color: &Vec3) -> std::io::Result<()> {
    let ir: i32 = (255.999_f32 * color.x) as i32;
    let ig: i32 = (255.999_f32 * color.y) as i32;
    let ib: i32 = (255.999_f32 * color.z) as i32;

    write!(buffer, "{} {} {}\n", ir, ig, ib)?;

    Ok(())
}