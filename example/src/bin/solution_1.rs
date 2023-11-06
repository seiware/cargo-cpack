use cargo_cpack_sample::a::print_a;

fn main() {
    print_a();
}

// The following is the output when cargo-cpack is run with this file specified.
//
// > cargo cpack -f ./solution_1.rs
// fn main() {
//     print_a();
// }
//
// pub mod cargo_cpack_sample {
//     pub fn macro_d() {}
//     pub fn b() {}
//     pub mod a {
//         pub fn print_a() {
//             println!("from a::print_a()");
//         }
//     }
// }
