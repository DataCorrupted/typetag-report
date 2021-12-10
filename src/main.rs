/// For conveniently changing the implementation of numerals
pub type Numeral = Box<dyn Value>;

#[typetag::serde(tag = "type")]
pub trait Value {
    /// The minumum value of this type
    fn min(&self) -> f64;
}

macro_rules! impl_value_for {
    ($t: ty) => {
        #[typetag::serde]
        impl Value for $t {
            /// The minumum value of this type
            fn min(&self) -> f64 {
                0f64
            }
        }
    };
}

// Since `into()` is used, you can't auto `impl Value for` `usize`, `u64`, or `i64`.
// See the explanation [here](https://stackoverflow.com/questions/35974890/from-and-into-traits-and-conversion-of-usize-to-f64)
// So we changed all of them to `as`, it may lose precision on `usize`, `u64`, and `i64`.
impl_value_for!(u8);

fn main() {}
