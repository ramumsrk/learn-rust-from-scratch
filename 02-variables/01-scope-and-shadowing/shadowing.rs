fn main() {
    let outer = 10;
    println!( "The value of variable outer is '{}'" , outer );
    {
        let outer = 11;
        println!( "The value of variable outer is '{}'" , outer );
    }
}
