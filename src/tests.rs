use super::*;

#[test]
fn test_cross_product() {
    let v1 = Vec3::new(1f64, 0f64, 0f64);
    let v2 = Vec3::new(0f64, 1f64, 0f64);

    let cross = v1.cross(&v2);

    assert_eq!(cross, Vec3::new(0f64, 0f64, 1f64));
}

#[test]
fn test_magnitude() {
    let v = Vec3::new(1f64, 2f64, 2f64);

    let mag = v.magnitude();

    assert_eq!(mag, 3f64);
}

#[test]
fn test_unit() {
    let v = Vec3::new(5f64, 0f64, 0f64);

    let unit = v.unit();

    assert_eq!(unit, Vec3::new(1f64, 0f64, 0f64));
}

#[test]
fn test_proj() {
    let u = Vec3::new(5f64, 5f64, 0f64);
    let v = Vec3::new(1f64, 0f64, 0f64);

    let proj = u.proj(&v);

    assert_eq!(proj, Vec3::new(5f64, 0f64, 0f64));
}

#[test]
fn test_angle() {
    let v1 = Vec3::new(1f64, 0f64, 0f64);
    let v2 = Vec3::new(0f64, 1f64, 0f64);

    let angle = v1.angle(&v2);

    assert_eq!(angle, std::f64::consts::PI / 2f64);
}

#[test]
fn test_add() {
    let v1 = Vec3::new(1f64, 2f64, 3f64);
    let v2 = Vec3::new(4f64, 5f64, 6f64);

    let sum = v1.add(&v2);

    assert_eq!(sum, Vec3::new(5f64, 7f64, 9f64));
}

#[test]
fn test_into() {
    let v1: Vec3 = [1f64, 2f64, 3f64].into();

    assert_eq!(v1, Vec3::new(1f64, 2f64, 3f64));
}