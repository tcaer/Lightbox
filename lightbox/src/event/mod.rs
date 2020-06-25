use crate::KeyCode;

use std::any::Any;

#[derive(PartialEq)]
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

pub struct EventStack {

  events: Vec<Box<dyn Event>>

}

impl EventStack {

  pub fn new() -> EventStack {
    return EventStack {
      events: Vec::new()
    }
  }

  pub fn add_event(&mut self, event: Box<dyn Event>) {
    self.events.push(event);
  }

  pub fn clear(&mut self) {
    self.events.clear();
  }

  pub fn get_size(&self) -> usize {
    return self.events.len();
  }

  /*
  * We have to return the data and not the actual event because it is not possible to move
  * the event out of the evenstack. Thus, we instead move the data, and leave it to the user
  * to cast it to the appropriate data type they want.
  */
  pub fn get_event_by_type(&self, event_type: EventType) -> Option<Box<dyn Any>> {
    for event in self.events.iter() {
      if event.get_type() == event_type {        
        return event.get_data();
      }
    }

    return None;
  }

  pub fn get_events_by_type(&self, event_type: EventType) -> Vec<Box<dyn Any>> {
    let mut events = Vec::new();
    
    for event in self.events.iter() {
      if event.get_type() == event_type {
        if let Some(data) = event.get_data() {
          events.push(data);
        }
      }
    }

    return events;
  }

  pub fn remove_event_by_type(&mut self, event_type: EventType) {
    self.events.retain(|event| event.get_type() != event_type);
  }

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

  m_width: i32,
  m_height: i32

}
impl WindowResizeEvent {

  pub fn new(width: i32, height: i32) -> WindowResizeEvent {
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

  m_key: KeyCode,
  m_repeat_count: u32

}
impl KeyPressedEvent {

  pub fn new(key: KeyCode, repeat_count: u32) -> KeyPressedEvent {
    return KeyPressedEvent {
      m_key: key,
      m_repeat_count: repeat_count
    };
  }

}
impl Event for KeyPressedEvent {

  fn get_type(&self) -> EventType {
    return EventType::KeyPressed;
  }
  fn get_category(&self) -> EventCategory {
    return EventCategory::Keyboard;
  }

  fn get_data(&self) -> Option<Box<dyn Any>> {
    return Some(Box::new((self.m_key, self.m_repeat_count)));
  }

}

pub struct KeyReleasedEvent {

  m_key: KeyCode

}
impl KeyReleasedEvent {

  pub fn new(key: KeyCode) -> KeyReleasedEvent {
    return KeyReleasedEvent {
      m_key: key
    };
  }

}
impl Event for KeyReleasedEvent {

  fn get_type(&self) -> EventType {
    return EventType::KeyReleased;
  }
  fn get_category(&self) -> EventCategory {
    return EventCategory::Keyboard;
  }

  fn get_data(&self) -> Option<Box<dyn Any>> {
    return Some(Box::new(self.m_key));
  }

}
