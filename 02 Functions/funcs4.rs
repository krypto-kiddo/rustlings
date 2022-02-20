// Code by Krypto Kiddo
// Functions Ex4 : Returns and Return Types in functions

fn main(){
    let original_price : i32 = 51;
    println!("The sale price is {}",sale_price(original_price));
}

fn sale_price(price:i32) -> i32{
    if is_even(price){
        price - 10
    }
    else{
        price - 3
    }
}

fn is_even(num:i32) -> bool {
    num%2==0
}

// Somehow, you aren't allowed to put  a ; at the end of the return statement in the if else thingy
