// Code by Krypto Kiddo
// Move Semantics Excercise 5 : Move Semantics

// Woah pointers, I thought this shit wouldn't follow after the days of C and Cpp 


// In this one I learnt that you can only borrow once into a variable. Once you borrow it somewhere else it doesnt' remain in the previous one. 
// Thats why you gotta operate before lending it away to another borrower
// Kinda like NFTs
// I mean its a crypto language after all xD

fn main(){
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 100;
    assert_eq!(x,1200);
}
