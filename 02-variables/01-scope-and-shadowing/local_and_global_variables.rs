fn main() {
    let global_variable = "Local";
    {
        let local_variable = "Local";
        println!( "Value of local variable: '{}'" , local_variable );
        println!( "Value of global variable: '{}'" , global_variable );
    }
    println!( "Value of local variable: '{}'" , local_variable );
    println!( "Value of global variable: '{}'" , global_variable );
}
