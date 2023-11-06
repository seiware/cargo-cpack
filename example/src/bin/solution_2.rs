use cargo_cpack_sample::a::c::print_c;
use cargo_cpack_sample::a::print_a;
use cargo_cpack_sample::b::print_b;
use cargo_cpack_sample::macro_a_d;
use cargo_cpack_sample::print_lib;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    print_a();
    print_b();
    print_c();
    macro_a_d!();
    print_lib();
    println!("{:?}", HashMap::from([("key1", 0.4), ("key2", 0.7)]));
    println!("{:?}", HashSet::from([1, 2, 3]));
}

// The following is the output when cargo-cpack is run with this file specified.
//
// > cargo cpack -f ./solution_2.rs
// use std::collections::HashMap;
// use std::collections::HashSet;
// use cargo_cpack_sample::a::c::print_c;
// use cargo_cpack_sample::a::print_a;
// use cargo_cpack_sample::b::print_b;
// use cargo_cpack_sample::macro_a_d;
// use cargo_cpack_sample::print_lib;
//
// fn main() {
//     print_a();
//     print_b();
//     print_c();
//     macro_a_d!();
//     print_lib();
//     println!("{:?}", HashMap::from([("Mercury", 0.4), ("Venus", 0.7)]));
//     println!("{:?}", HashSet::from([1, 2, 3]));
// }
//
// pub mod cargo_cpack_sample {
//     pub fn macro_a_d() {}
//     pub fn macro_a_e() {}
//     pub fn print_lib() {
//         println!("from print_lib()");
//     }
//     pub mod a {
//         pub fn print_a() {
//             println!("from a::print_a()");
//         }
//         pub mod c {
//             pub fn print_c() {
//                 println!("from a::c::print_c()");
//             }
//         }
//         pub mod d {
//             #[macro_export]
//             macro_rules! macro_a_d {
//                 () => {
//                     print!("macro: ");
//                     $crate::cargo_cpack_sample::a::d::print_for_macro()
//                 };
//             }
//             pub fn print_a_d() {
//                 println!("from a::d::print_a_d()");
//             }
//         }
//     }
//     pub mod b {
//         pub fn print_b() {
//             println!("from b::print_b()");
//         }
//     }
// }
