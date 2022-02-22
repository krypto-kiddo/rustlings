// Lmao I should be studying, I've got an exam tomorrow (on DNN - Deep Neural Networks)
// Ofcourse I haven't studied anything, AI is boring       *moon face emoji*

// Code Copied and Solved by Krypto Kiddo


// if1.rs


pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `rustlings hint if1` for hints
    if a>b{
        a
    }
    else{
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
