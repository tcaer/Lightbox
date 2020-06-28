/*
* A wrapper for unsafe GL functions that are exposed to the client app
*/

pub fn set_viewport(width: i32, height: i32) {
  unsafe {
    gl::Viewport(0, 0, width, height);
  }
}
