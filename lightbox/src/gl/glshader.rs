use gl;
use std::ffi::CString;
use cgmath::{Vector2, Vector3, Vector4, Matrix, Matrix3, Matrix4};

#[allow(dead_code)]
pub struct GlShader {

  id: u32,
  name: String
  
}
impl GlShader {

  pub fn new(name: &str, vert_src: &str, frag_src: &str) -> GlShader {
    let id = GlShader::compile_shaders(&vert_src, &frag_src);

    return GlShader {
      id: id,
      name: name.to_string()
    };
  }

  pub fn bind(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      gl::UseProgram(0);
    }
  }

  pub fn set_int(&self, name: &str, value: i32) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::Uniform1i(location, value);
    }
  }

  pub fn set_int_array(&self, name: &str, values: &Vec<i32>, count: i32) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::Uniform1iv(location, count, values.as_ptr());
    }
  }

  pub fn set_float2(&self, name: &str, value: &Vector2<f32>) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::Uniform2f(location, value.x, value.y);
    }
  }

  pub fn set_float3(&self, name: &str, value: &Vector3<f32>) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::Uniform3f(location, value.x, value.y, value.z);
    }
  }

  pub fn set_float4(&self, name: &str, value: &Vector4<f32>) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::Uniform4f(location, value.x, value.y, value.z, value.w);
    }
  }

  pub fn set_mat3(&self, name: &str, value: &Matrix3<f32>) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr());
    }
  }

  pub fn set_mat4(&self, name: &str, value: &Matrix4<f32>) {
    let c_name = CString::new(name.as_bytes()).unwrap();

    unsafe {
      let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
      gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
    }
  }

  fn compile_shaders(vert_src: &str, frag_src: &str) -> u32 {
    #[allow(unused_assignments)]
    let (mut program_id, mut vert_id, mut frag_id) = (0, 0, 0);

    unsafe {
      program_id = gl::CreateProgram();
    }

    if !GlShader::compile_shader(&mut vert_id, &vert_src, gl::VERTEX_SHADER) {
      GlShader::debug_shader(&mut vert_id);
      return 0;
    } else {
      unsafe {
        gl::AttachShader(program_id, vert_id);
      }
    }

    if !GlShader::compile_shader(&mut frag_id, &frag_src, gl::FRAGMENT_SHADER) {
      GlShader::debug_shader(&mut frag_id);
      return 0;
    } else {
      unsafe {
        gl::AttachShader(program_id, frag_id);
      }
    }

    unsafe {
      gl::LinkProgram(program_id);

      let mut success = gl::FALSE as gl::types::GLint;
      gl::GetShaderiv(program_id, gl::COMPILE_STATUS, &mut success);

      if success != gl::FALSE as gl::types::GLint {
        let mut max_size = 0;
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut max_size);

        let mut info_log = Vec::with_capacity(max_size as usize);
        gl::GetProgramInfoLog(program_id, max_size, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
        println!("ERROR:\nProgram failed to link.\n{}", std::str::from_utf8(&info_log).unwrap());

        gl::DeleteProgram(program_id);
        gl::DeleteShader(vert_id);
        gl::DeleteShader(frag_id);

        return 0;
      }

      gl::DetachShader(program_id, vert_id);
      gl::DeleteShader(vert_id);
      gl::DetachShader(program_id, frag_id);
      gl::DeleteShader(frag_id);
    }

    return program_id;
  }

  fn compile_shader(id: &mut u32, src: &str, shader_type: gl::types::GLenum) -> bool {
    unsafe {
      *id = gl::CreateShader(shader_type);
      let c_str = CString::new(src.as_bytes()).unwrap();

      gl::ShaderSource(*id, 1, &c_str.as_ptr(), std::ptr::null());
      gl::CompileShader(*id);

      let mut success = gl::FALSE as gl::types::GLint;
      gl::GetShaderiv(*id, gl::COMPILE_STATUS, &mut success);

      return success != gl::FALSE as gl::types::GLint;
    }
  }

  fn debug_shader(id: &mut u32) {
    unsafe {
      let mut max_size = 0;
      gl::GetShaderiv(*id, gl::INFO_LOG_LENGTH, &mut max_size);

      let mut info_log = Vec::with_capacity(max_size as usize);
      gl::GetShaderInfoLog(*id, max_size, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
      println!("ERROR:\nShader failed to compile.\n{}", std::str::from_utf8(&info_log).unwrap());
    }
  }

}
impl Drop for GlShader {

  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }

}
