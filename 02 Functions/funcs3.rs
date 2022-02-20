// Code by Krypto Kiddo
// Functions Ex3 : Basically the same as the previous one but with an unsigned 32bit int

fn main(){
    call_me(5);
}

fn call_me(num:u32){
    for i in 0..num{
        println!("Ring! Call number is {}",i+1);
    }
}
