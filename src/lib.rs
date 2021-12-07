mod bivec2;
mod bivec3;
mod bivector;
mod rot2;
mod rot3;
mod rotor;
mod vec2;
mod vec3;
mod vec4;
mod vector;

pub use bivec2::*;
pub use bivec3::*;
pub use bivector::*;
pub use rot2::*;
pub use rot3::*;
pub use rotor::*;
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
