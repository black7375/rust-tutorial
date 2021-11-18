//== Use =======================================================================
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}

use a::series::of;
fn test1() {
    of::nested_modules();
}

use a::series::of::nested_modules;
fn test2() {
    nested_modules();
}

//== Enum Use ==================================================================
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
fn enum1() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

// Glob
use TrafficLight::*;
fn enum1() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

//== Super =====================================================================
