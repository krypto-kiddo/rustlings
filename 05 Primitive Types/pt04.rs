// Code by Krypto Kiddo
// Primitive Types Ex 4 : Array Slicing

#[test]
fn slice_array(){
    let arr = [1,2,3,4,5];
    let nice_slice = &arr[1..4];
    assert_eq!([2,3,4],nice_slice);
}
