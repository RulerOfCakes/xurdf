use xurdf::*;

pub fn main() {
    let urdf = parse_urdf_from_file("data/ant.urdf");
    println!("{:#?}", urdf);
}
