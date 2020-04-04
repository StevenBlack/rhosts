// A place for utility functions
pub fn sep(n: usize) {
    println!("{}", "-".repeat(n));
}

#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("===> {}", std::any::type_name::<T>())
}
