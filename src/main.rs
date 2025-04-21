include!("bindings.rs");

fn main() {
    let result = unsafe {
        add(3, 4)
    };

    println!("The result of adding 3 and 4 is: {}", result);
}