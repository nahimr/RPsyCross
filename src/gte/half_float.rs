use half::f16;

pub fn to_half_float(x: f32) -> f16 {
    f16::from_f32(x)
}

pub fn from_half_float(x: f16) -> f32 {
    x.to_f32()
}
