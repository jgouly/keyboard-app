use layout::Layout;
use matrix::Matrix;
use state::KeyState;

const MAX_PACKET_SIZE: usize = 6;
pub struct USBBuffer {
  pub data: [u32; MAX_PACKET_SIZE],
  pub mods: u32,
  pub count: usize,
}

impl USBBuffer {
  pub fn new() -> USBBuffer {
    USBBuffer {
      data: [0; 6],
      mods: 0,
      count: 0,
    }
  }
  pub fn push(&mut self, val: u32) {
    self.data[self.count] = val;
    self.count += 1;
  }
  pub fn push_mod(&mut self, val: u32) {
    self.mods |= val;
  }
}

pub fn process_actions<SM: Matrix<T = KeyState>, L: Layout>(
  states: SM,
  layout: &L,
) -> USBBuffer {
  let mut buf = USBBuffer::new();
  for c in 0..states.get_num_columns() {
    for r in 0..states.get_num_rows() {
      let state = states.get(r, c);
      if state != KeyState::None {
        layout.process_action(r, c, state, &mut buf);
      }
    }
  }
  buf
}

#[cfg(test)]
fn private_basic() {
  gen_matrix!(Matrix2x3u32, 2, 3, u32);
  gen_matrix!(Matrix2x3KS, 2, 3, KeyState);
  let states = Matrix2x3KS::new_with_data([
    KeyState::None,
    KeyState::Pressed,
    KeyState::None,
    KeyState::None,
    KeyState::None,
    KeyState::None,
  ]);
  struct TestLayout {
    data: Matrix2x3u32,
  }
  impl Layout for TestLayout {
    fn process_action(
      &self,
      r: usize,
      c: usize,
      _: KeyState,
      buf: &mut USBBuffer,
    ) {
      buf.push(self.data.get(r, c));
    }
  }
  let layout = TestLayout {
    data: Matrix2x3u32::new_with_data([0, 1, 2, 3, 4, 5]),
  };
  let buf = process_actions(states, &layout);
  assert_eq!(buf.data, [1, 0, 0, 0, 0, 0]);
}

#[cfg(test)]
mod tests {
  use super::private_basic;
  #[test]
  fn basic() {
    private_basic();
  }
}
