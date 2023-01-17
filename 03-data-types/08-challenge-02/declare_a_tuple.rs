fn main() {
    let persons : ( &str , i8 , &str , i8 , &str , i8 ) = ( "Alex" , 21 , "Abe" , 22 , "Anna" , 23 );
    println!( "{}:{}, {}:{}, {}:{}" , persons.0 , persons.1 , persons.2 , persons.3 , persons.4 , persons.5 ); 
}
