mod bivec2;
mod bivec3;
mod rotor2;
mod rotor3;
mod vec2;
mod vec3;
mod vec4;
mod vector;

pub use bivec2::*;
pub use bivec3::*;
pub use rotor2::*;
pub use rotor3::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vector::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
