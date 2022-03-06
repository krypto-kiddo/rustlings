// Code by Krypto Kiddo
// Primitive Types Ex 2 : String methods is_alphabetic and is_numeric

fn main(){
    let my_first_initial = 'K';
    find_type(my_first_initial);
}

fn find_type(string:char){
    if string.is_alphabetic(){
        println!("Alphabetical!")
    }
    else if string.is_numeric(){
        println!("Numeric!");
    }
    else{
        println!("Neither Alphabetical nor Numeric!");
    }
}
