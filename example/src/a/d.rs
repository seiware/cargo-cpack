#[macro_export]
macro_rules! macro_a_d {
    () => {
        print!("macro: ");
        $crate::a::d::print_a_d()
    };
}

pub fn print_a_d() {
    println!("from a::d::print_a_d()");
}
