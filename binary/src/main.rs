use constants::SomeStruct;

fn function_that_does_something(b: bool) -> SomeStruct {
    SomeStruct {
        some_field: if b { 42 } else { 24 },
    }
}

fn main() {
    println!("The constant is {}", constants::SOME_CONSTANT);

    let some_struct = function_that_does_something(true);

    println!("some_field is {:?}", some_struct.some_field);
}

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn function_never_returns_zero_struct() {
        let input : bool = kani::any();
        let output = function_that_does_something(input);

        assert!(output.some_field != 0);
    }
}
