use std::any::Any;

pub enum EventType {
  None,
  WindowClose, WindowResize,
  KeyPressed, KeyReleased, KeyTyped,
  MousePressed, MouseReleased, MouseScrolled, MouseMoved
}

pub enum EventCategory {
  None,
  App,
  Keyboard,
  Mouse
}

pub trait Event {

  fn get_type(&self) -> EventType;
  fn get_category(&self) -> EventCategory;

  fn get_data(&self) -> Option<Box<dyn Any>>;
  
}

/* Application Events */
pub struct WindowCloseEvent { }
impl Event for WindowCloseEvent {

  fn get_type(&self) -> EventType {
    return EventType::WindowClose;
  }
  fn get_category(&self) -> EventCategory {
    return EventCategory::App;
  }

  fn get_data(&self) -> Option<Box<dyn Any>> {
    return None;
  }
  
}

pub struct WindowResizeEvent {

  m_width: f32,
  m_height: f32

}
impl WindowResizeEvent {

  pub fn new(width: f32, height: f32) -> WindowResizeEvent {
    return WindowResizeEvent {
      m_width: width,
      m_height: height
    };
  }

}
impl Event for WindowResizeEvent {

  fn get_type(&self) -> EventType {
    return EventType::WindowResize;
  }
  fn get_category(&self) -> EventCategory {
    return EventCategory::App;
  }

  fn get_data(&self) -> Option<Box<dyn Any>> {
    return Some(Box::new((self.m_width, self.m_height)));
  }
  
}

/* Key Events */
pub struct KeyPressedEvent {

  

}
