use crate::camera::CameraConfig;
use crate::hittable::World;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera_config: CameraConfig,
    pub world: World,
}
