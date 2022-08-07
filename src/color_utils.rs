use crate::vec3::Color;

pub fn write_color(buffer: &mut String, color: &Color) {
    let ir: i32 = (255.999_f32 * color.x) as i32;
    let ig: i32 = (255.999_f32 * color.y) as i32;
    let ib: i32 = (255.999_f32 * color.z) as i32;

    buffer.push_str(&format!("{} {} {}\n", ir, ig, ib));
}