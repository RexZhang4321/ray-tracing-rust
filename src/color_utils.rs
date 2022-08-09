use crate::vec3::Color;

// color is the sum of all samples
// samples_per_pixel is the number of samples
// the final color to write is the avg of all samples
pub fn write_color(buffer: &mut String, color: &Color, samples_per_pixel: i32) {
    let ir: i32 = (256.0 * (color.x / samples_per_pixel as f32).clamp(0.0, 0.999)) as i32;
    let ig: i32 = (256.0 * (color.y / samples_per_pixel as f32).clamp(0.0, 0.999)) as i32;
    let ib: i32 = (256.0 * (color.z / samples_per_pixel as f32).clamp(0.0, 0.999)) as i32;

    buffer.push_str(&format!("{} {} {}\n", ir, ig, ib));
}
