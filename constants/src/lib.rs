pub const SOME_CONSTANT: u32 = 42;


pub struct SomeStruct {
    pub some_field: u32,
}

#[cfg(kani)]
impl kani::Arbitrary for SomeStruct {
    fn any() -> Self {
        SomeStruct {
            some_field: kani::any()
        }
    }
}
