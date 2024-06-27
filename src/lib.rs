#![allow(warnings)]
pub mod helper;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}",helper::helpers::apply_escape_sequence("random string",31));
    }
}
