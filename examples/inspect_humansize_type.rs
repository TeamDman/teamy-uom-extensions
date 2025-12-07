fn main() {
    // Print type names to help determine correct signature
    println!("DECIMAL type: {}", std::any::type_name_of_val(&humansize::DECIMAL));
    println!("BINARY type:  {}", std::any::type_name_of_val(&humansize::BINARY));
}
