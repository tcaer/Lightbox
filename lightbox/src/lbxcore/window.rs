use crate::glfw::Context;
use std::sync::mpsc::Receiver;

use crate::{EventStack, EventType};
use crate::event::{WindowResizeEvent};

pub struct Window {

  glfw_context: glfw::Glfw,
  glfw_window: glfw::Window,
  glfw_events: Receiver<(f64, glfw::WindowEvent)>

}

impl Window {

  pub fn create(title: &str, width: u32, height: u32) -> Window {
    let mut glfw_context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw_context.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw_context.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw_context.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw_context.create_window(width, height, title, glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window!");

    window.make_current();
    window.set_all_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    return Window {
      glfw_context: glfw_context,
      glfw_window: window,
      glfw_events: events
    };
  }

  pub fn update(&mut self) {
    self.glfw_window.swap_buffers();
    self.glfw_context.poll_events();
  }

  pub fn should_close(&self) -> bool {
    return self.glfw_window.should_close();
  }

  pub fn get_size(&self) -> (i32, i32) {
    return self.glfw_window.get_size();
  }

  pub fn poll_events(&self, event_stack: &mut EventStack) {
    event_stack.clear();

    for (_, event) in glfw::flush_messages(&self.glfw_events) {
      match event {
        glfw::WindowEvent::Size(width, height) => {
          event_stack.remove_event_by_type(EventType::WindowResize);
          let resize_event = WindowResizeEvent::new(width, height);
          event_stack.add_event(Box::new(resize_event));
        },
        _ => {}
      }
    }
  }

}
