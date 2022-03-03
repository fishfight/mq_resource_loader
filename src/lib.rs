pub mod error;
pub mod animations;
pub mod json;
pub mod helpers;
pub mod character;
pub mod resources;
pub mod animated_sprite;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
