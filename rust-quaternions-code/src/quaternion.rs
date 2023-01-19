use core::fmt::*;

pub struct Quaternion
{
    data: (f32, f32, f32, f32),
}

pub trait QuaternionOperations
{
    fn add(first_quat: &Quaternion, second_quat: &Quaternion) -> Quaternion;
    fn product(first_quat: &Quaternion, second_quat: &Quaternion) -> Quaternion;
    fn rotation(rotation: &(f32, f32, f32), deg_angle: &f32);
    fn conjugate(quaternion: &mut Quaternion);
}

impl Quaternion
{
    pub fn new(q: f32, i: f32, j: f32, k: f32) -> Self
    {
        Self { data: (q, i, j, k) }
    }
}

impl Display for Quaternion
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        let dat = &self.data;
        write!(
            f,
            "([q]: {}, [i]: {}, [j]: {}, [k]: {})",
            dat.0, dat.1, dat.2, dat.3
        )
    }
}

impl QuaternionOperations for Quaternion
{
    // TODO : Complete add
    fn add(first_quat: &Quaternion, second_quat: &Quaternion) -> Quaternion
    {
        Quaternion {
            data: (1f32, 1f32, 1f32, 1f32),
        }
    }

    // TODO : Complete product
    fn product(first_quat: &Quaternion, second_quat: &Quaternion) -> Quaternion
    {
        Quaternion {
            data: (1f32, 1f32, 1f32, 1f32),
        }
    }

    // TODO : Complete rotation
    fn rotation(rotation_vector: &(f32, f32, f32), deg_angle: &f32) {}

    // TODO : Complete conjugate
    fn conjugate(quaternion: &mut Quaternion) {}
}
