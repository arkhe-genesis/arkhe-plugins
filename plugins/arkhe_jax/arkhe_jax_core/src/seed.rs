#[derive(Clone)]
pub struct Seed64(pub [u8; 64]);

impl Default for Seed64 {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl AsMut<[u8]> for Seed64 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
