use action::USBBuffer;
use state::KeyState;

pub trait Layout {
  fn process_action(
    &self,
    row: usize,
    col: usize,
    state: KeyState,
    buf: &mut USBBuffer,
  );
}
