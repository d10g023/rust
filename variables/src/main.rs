fn main() {
    // variable must be declared with let keyword and snake case
    let variable_in_snake_case = 5;
    println!("The value of variable_in_snake_case is: {} \n", variable_in_snake_case);

    // variable typed by compiler
    let variable_typed_by_compiler = 5;
    println!("The value of a is: {} \n", variable_typed_by_compiler);
    

    // variable typed by user
    let variable_typed_by_user: u32 = 5;
    println!("The value of b is: {} \n", variable_typed_by_user);

    // variable mutable
    let mut variable_mutable = 5;
    variable_mutable = 10;
    println!("The value of c is: {} \n", variable_mutable);

    // variable shadowing
    let variable_shadowing = 5;
    let variable_shadowing = "Hello World!";
    println!("The value of a is: {}\n", variable_shadowing);

    // variable inside a scope
    let variable_inside_scope = 5;
    {   
        println!("The value of variable_inside_scope outside the scope is: {}", variable_inside_scope);
        let variable_inside_scope = 10;
        println!("The value of variable_inside_scope inside the scope is: {}", variable_inside_scope);
    }
    println!("The value of variable_inside_scope outside the scope is: {}\n", variable_inside_scope);

    // constant
    const CONSTANT_DEFINITION: u32 = 5;
    println!("The value of E is: {}\n", CONSTANT_DEFINITION);
    
}