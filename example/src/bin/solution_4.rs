use cargo_cpack_sample::a::c::d::print_a_c_d;

fn main() {
    print_a_c_d();
}

// The following is the output when cargo-cpack is run with this file specified.
//
// > cargo cpack -f ./solution_1.rs
// use cargo_cpack_sample::a::c::d::print_a_c_d;
//
// fn main() {
//     print_a_c_d();
// }
//
// pub mod cargo_cpack_sample {
//     pub fn macro_a_d() {}
//     pub fn macro_a_e() {}
//     pub mod a {
//         pub mod c {
//             pub mod d {
//                 pub fn print_a_c_d() {
//                     println!("from a::c::d::print_a_c_d()");
//                 }
//             }
//         }
//     }
// }
