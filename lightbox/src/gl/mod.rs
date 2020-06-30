use gl;
mod buffer;

pub use buffer::GlVertexBuffer;
pub use buffer::GlIndexBuffer;

/*
* A wrapper for unsafe GL functions that are exposed to the client app and other internal modules
*/
pub fn clear_color_depth_buffers() {
  unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe {
    gl::ClearColor(r, g, b, a);
  }
}

pub fn set_viewport(width: i32, height: i32) {
  unsafe {
    gl::Viewport(0, 0, width, height);
  }
}
