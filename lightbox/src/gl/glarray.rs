use gl;

pub struct GlArray {

  id: u32

}
impl GlArray {

  pub fn new() -> GlArray {
    let mut id = 0;

    unsafe {
      gl::GenVertexArrays(1, &mut id);
    }

    return GlArray {
      id: id
    }
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::BindVertexArray(0);
    }
  }

  pub fn set_float_attrib_ptr(&self, index: u32, length: i32, offset: u32) {
    unsafe {
      if offset == 0 {
        gl::VertexAttribPointer(index, length, gl::FLOAT, gl::FALSE, 
          length * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, std::ptr::null());
      } else {
        gl::VertexAttribPointer(index, length, gl::FLOAT, gl::FALSE, 
          length * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei, offset as *const std::os::raw::c_void);
      }
      gl::EnableVertexAttribArray(index);
    }
  }

}
