use cargo_cpack_sample::a::c::print_c;
use cargo_cpack_sample::a::*;

fn main() {
    print_a();
    print_c();
}

// The following is the output when cargo-cpack is run with this file specified.
//
// > cargo cpack -f ./solution_3.rs
// use cargo_cpack_sample::a::c::print_c;
// use cargo_cpack_sample::a::*;
//
// fn main() {
//     print_a();
//     print_c();
// }
//
// pub mod cargo_cpack_sample {
//     pub fn macro_a_d() {}
//     pub fn macro_a_e() {}
//     pub mod a {
//         pub fn print_a() {
//             println!("from a::print_a()");
//         }
//         pub mod c {
//             pub fn print_c() {
//                 println!("from a::c::print_c()");
//             }
//         }
//     }
// }
