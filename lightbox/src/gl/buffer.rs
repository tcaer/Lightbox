use gl;

pub struct GlVertexBuffer {

  id: u32
  
}
impl GlVertexBuffer {

  pub fn new(data: Vec<f32>, size: isize) -> GlVertexBuffer {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);

      gl::BindBuffer(gl::ARRAY_BUFFER, id);
      gl::BufferData(gl::ARRAY_BUFFER, size, &data[0] as *const f32 as *const std::os::raw::c_void, gl::STATIC_DRAW);
    }

    return GlVertexBuffer {
      id: id
    };
  }

  pub fn new_empty(size: isize) -> GlVertexBuffer {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);

      gl::BindBuffer(gl::ARRAY_BUFFER, id);
      gl::BufferData(gl::ARRAY_BUFFER, size, std::ptr::null(), gl::DYNAMIC_DRAW);
    }

    return GlVertexBuffer {
      id: id
    };
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }
  }
  
  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
  }

  pub fn set_data(&self, data: Vec<f32>, size: isize) {
    self.bind();

    unsafe {
      gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, &data[0] as *const f32 as *const std::os::raw::c_void);
    }
  }

}

pub struct GlIndexBuffer {

  id: u32

}
impl GlIndexBuffer {

  pub fn new(data: Vec<u32>, size: isize) -> GlIndexBuffer {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);

      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data[0] as *const f32 as *const std::os::raw::c_void, gl::STATIC_DRAW);
    }

    return GlIndexBuffer {
      id: id
    };
  }

  pub fn bind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }
  }
  
  pub fn unbind(&self) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
  }

}
