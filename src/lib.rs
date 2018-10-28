#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod contests;
pub mod users;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
