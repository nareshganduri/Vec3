use vec3::Vec3;

fn main() {
    let x = Vec3::new(1f64, 2f64, 3f64);
    let y = Vec3::new(4f64, 5f64, 6f64);

    println!("{}", x.magnitude());
    println!("{}", x.dot(&y));
    println!("{:?}", x.cross(&y));

    let z = x.unit();
    println!("{:?}", z);
    println!("{}", z.magnitude());

    let proj = x.proj(&y);
    println!("{:?}", proj);

    let angle = x.angle(&y);
    println!("{}", angle);
}
