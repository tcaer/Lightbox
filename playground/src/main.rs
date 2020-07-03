extern crate lbx;

const VERT_SOURCE: &str = r#"
#version 330 core

layout(location = 0) in vec3 a_pos;

void main() {
  gl_Position = vec4(a_pos, 1.0);
}
"#;

const FRAG_SOURCE: &str = r#"
#version 330 core

out vec4 frag_color;

void main() {
  frag_color = vec4(0.0, 1.0, 0.0, 1.0);
}
"#;

fn main() {
  let mut window = lbx::Window::create("lbx window", 1280, 720);
  let mut event_stack = lbx::EventStack::new();
  let mut camera = lbx::OrthoCamera::new_default(1280.0/720.0);

  lbx::gl::clear_color(0.1, 0.1, 0.1, 0.1);

  let vertices = vec![0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0];
  let indices = vec![0, 1, 3, 1, 2, 3];

  let shader = lbx::gl::GlShader::new("flat_color", VERT_SOURCE, FRAG_SOURCE);
  shader.bind();

  let vao = lbx::gl::GlArray::new();
  vao.bind();
  let _vbo = lbx::gl::GlVertexBuffer::new(&vertices);
  let _ebo = lbx::gl::GlIndexBuffer::new(&indices);

  vao.set_float_attrib_ptr(0, 3, 0);

  while !window.should_close() {
    window.poll_events(&mut event_stack);
    lbx::gl::clear_color_depth_buffers();

    if let Some(raw_data) = event_stack.get_event_by_type(lbx::EventType::WindowResize) {
      if let Some((width, height)) = raw_data.downcast_ref::<(i32, i32)>() {
        lbx::gl::set_viewport(*width, *height);
      }
    }

    shader.bind();
    
    vao.bind();
    lbx::gl::draw_elements(6);

    shader.unbind();

    window.update();
  }
}
