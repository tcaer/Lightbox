use gl;
use std::ffi::CString;

struct GlShader {

  id: u32
  
}
impl GlShader {

  pub fn new(vert_src: &str, frag_src: &str) -> GlShader {
    let id = GlShader::compile_shaders(&vert_src, &frag_src);

    return GlShader {
      id: id
    };
  }

  fn compile_shaders(vert_src: &str, frag_src: &str) -> u32 {
    let (mut program_id, mut vert_id, mut frag_id) = (0, 0, 0);

    unsafe {
      program_id = gl::CreateProgram();
    }

    if !GlShader::compile_shader(&mut vert_id, &vert_src, gl::VERTEX_SHADER) {
      GlShader::debug_shader(&mut vert_id);
      return program_id;
    } else {
      unsafe {
        gl::AttachShader(program_id, vert_id);
      }
    }

    if !GlShader::compile_shader(&mut frag_id, &frag_src, gl::FRAGMENT_SHADER) {
      GlShader::debug_shader(&mut frag_id);
      return program_id;
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

        gl::DeleteProgram(program_id);
        gl::DeleteShader(vert_id);
        gl::DeleteShader(frag_id);

        return program_id;
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
    }
  }

}
