// Code by Krypto Kiddo
// Functions Ex2 : Taking parameters in a function and using for loop

fn main(){
    call_me(3);
}

fn call_me(num:i32){
    for i in 0..num {
        println!("Ring! Call number is {}",i+1);
    }
}
