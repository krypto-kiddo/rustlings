// Code by Krypto Kiddo
// Move Semantics Excercise 1 : Making mutable vectors and pushing data 


fn main(){
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("{} has length {} content '{:?}'","vec1",vec1.len(), vec1);
    vec1.push(69);
    println!("{} has length {} content '{:?}'","vec1",vec1.len(), vec1);
    
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(77);
    vec.push(17);
    vec.push(212);
    vec.push(14);
    
    vec
}
