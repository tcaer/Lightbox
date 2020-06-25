extern crate lbx;

fn main() {
  let mut window = lbx::Window::create("lbx window", 1280, 720);
  let mut event_stack = lbx::EventStack::new();

  while !window.should_close() {
    window.poll_events(&mut event_stack);

    /*if let Some(raw_data) = event_stack.get_event_by_type(lbx::EventType::WindowResize) {
      if let Some((width, height)) = raw_data.downcast_ref::<(i32, i32)>() {
        println!("{}, {}", width, height);
      }
    }*/

    /*let keys_pressed = event_stack.get_events_by_type(lbx::EventType::KeyPressed);
    for raw_key in keys_pressed.iter() {
      if let Some((key, repeat_count)) = raw_key.downcast_ref::<(lbx::KeyCode, u32)>() {
        println!("Key Pressed Event: {}, {}", *key as i32, repeat_count);
      }
    }*/

    window.update();
  }
}
