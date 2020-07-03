use cgmath::{vec3, Vector3, Matrix4};

pub struct OrthoCamera {
  
  position: Vector3<f32>,
  aspect_ratio: f32,
  zoom_level: f32

}
impl OrthoCamera {

  pub fn new_default(aspect_ratio: f32) -> OrthoCamera {
    return OrthoCamera {
      position: vec3(0.0, 0.0, 0.0),
      aspect_ratio: aspect_ratio,
      zoom_level: 1.0
    };
  }

  pub fn set_position(&mut self, position: Vector3<f32>) {
    self.position = position;
  }

  pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
    self.aspect_ratio = aspect_ratio;
  }

  pub fn set_zoom_level(&mut self, zoom_level: f32) {
    self.zoom_level = zoom_level;
  }

  pub fn get_projection_matrix(&self) -> Matrix4<f32> {
    let proj_matrix = cgmath::ortho(-self.aspect_ratio * self.zoom_level, self.aspect_ratio * self.zoom_level,
      -self.zoom_level, self.zoom_level, -1.0, 1.0);
    let translation_matrix = Matrix4::from_translation(self.position);

    return proj_matrix * translation_matrix;
  }

}
