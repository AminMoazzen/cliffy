mod bivec2;
mod rotor2;
mod utils;
mod vec2;
mod vec3;
mod vec4;

pub use bivec2::*;
pub use rotor2::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
