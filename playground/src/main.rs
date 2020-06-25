extern crate lbx;

fn main() {
  let mut window = lbx::Window::create("lbx window", 1280, 720);
  let mut event_stack = lbx::EventStack::new();

  while !window.should_close() {
    window.update();

    window.poll_events(&mut event_stack);

    if let Some(raw_data) = event_stack.get_event_by_type(lbx::EventType::WindowResize) {
      if let Some((width, height)) = raw_data.downcast_ref::<(i32, i32)>() {
        println!("{}, {}", width, height);
      }
    }
  }
}
