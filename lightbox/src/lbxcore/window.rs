use crate::glfw::{Context, Action};
use std::sync::mpsc::Receiver;

use crate::{EventStack, EventType, KeyCode};
use crate::input::{convert_key_to_glfw};
use crate::event::{WindowResizeEvent, KeyPressedEvent, KeyReleasedEvent};

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

  pub fn is_key_down(&self, key: KeyCode) -> bool {
    let action = self.glfw_window.get_key(convert_key_to_glfw(key));

    match action {
      glfw::Action::Press => return true,
      _ => return false
    }
  }

  pub fn poll_events(&self, event_stack: &mut EventStack) {
    event_stack.clear();

    for (_, event) in glfw::flush_messages(&self.glfw_events) {
      match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
          event_stack.remove_event_by_type(EventType::WindowResize);
          let resize_event = WindowResizeEvent::new(width, height);
          event_stack.add_event(Box::new(resize_event));
        },
        glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
          let raw_key = key as i32;
          match action {
            Action::Press => {
              if let Some(key) = num::FromPrimitive::from_i32(raw_key) {
                let pressed_event = KeyPressedEvent::new(key, 0);
                event_stack.add_event(Box::new(pressed_event));
              }
            },
            Action::Repeat => {
              if let Some(key) = num::FromPrimitive::from_i32(raw_key) {
                let pressed_event = KeyPressedEvent::new(key, 1);
                event_stack.add_event(Box::new(pressed_event));
              }
            },
            Action::Release => {
              if let Some(key) = num::FromPrimitive::from_i32(raw_key) {
                let released_event = KeyReleasedEvent::new(key);
                event_stack.add_event(Box::new(released_event));
              }
            }
          }
        },
        _ => {}
      }
    }
  }

}
