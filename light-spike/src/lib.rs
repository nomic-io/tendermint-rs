#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(
     unreachable_pub,
     // missing_docs,
 )]

pub mod components;
pub mod macros;
pub mod operations;
pub mod predicates;
pub mod prelude;
pub mod store;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
