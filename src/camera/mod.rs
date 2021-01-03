extern crate euler;
use euler::{Trs, Vec3, Quat};

pub struct CameraData
{
    transform : Trs
}

impl CameraData
{
    pub fn new(translation : &Vec3, rotation: &Quat) -> CameraData
    {
        let t = Trs::new(translation.clone(), rotation.clone(), Vec3{ x:1.0, y:1.0, z:1.0 });
        CameraData { transform : t }
    }
}