fn main() {
    let mut x = 5; // Error: Variable is never mutated
    let unused_variable = 10; // Warning: Unused variable

    // Unused variable
    #[allow(unused_variables)]
    let _unused_variable = 20; // Warning: Unused variable

    // Unused assignment (Error: Variable is assigned but its value is never used)
    #[allow(unused_assignments)]
    let y = {
        x = 10;
        x
    };

    // Single match
    match x {
        5 => println!("x is 5"),
        _ => (),
    } // Warning: Single match
}