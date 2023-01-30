use psycross::gte::half_float::*;

#[test]
fn test_float_conversion() {
    let myFloat: f32 = 3.50;
    assert_eq!(3.50, from_half_float(to_half_float(myFloat)));
}
