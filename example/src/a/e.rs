#[macro_export]
macro_rules! macro_a_e {
    () => {
        $crate::a::e::print_a_e()
    };
}

pub fn print_a_e() {
    println!("macro: from a::e::print_a_e()");
}
