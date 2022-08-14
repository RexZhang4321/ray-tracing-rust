use crate::vec3::Color;

// color is the sum of all samples
// samples_per_pixel is the number of samples
// the final color to write is the avg of all samples
pub fn write_color(buffer: &mut String, color: &Color, samples_per_pixel: i32) {
    let mut ir: f32 = color.x / samples_per_pixel as f32;
    let mut ig: f32 = color.y / samples_per_pixel as f32;
    let mut ib: f32 = color.z / samples_per_pixel as f32;

    // divide the color by the number of samples and gamma-correct for gamma=2.0 (sqrt)
    ir = ir.sqrt();
    ig = ig.sqrt();
    ib = ib.sqrt();

    buffer.push_str(
        &format!("{} {} {}\n",
        (256.0 * ir.clamp(0.0, 0.999)) as i32,
        (256.0 * ig.clamp(0.0, 0.999)) as i32,
        (256.0 * ib.clamp(0.0, 0.999)) as i32));
}
