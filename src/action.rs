use matrix::Matrix;
use state::KeyState;

const MAX_PACKET_SIZE: usize = 6;
struct USBBuffer {
  data: [u32; MAX_PACKET_SIZE],
  count: usize,
}

impl USBBuffer {
  fn new() -> USBBuffer {
    USBBuffer {
      data: [0; 6],
      count: 0,
    }
  }
  fn push(&mut self, val: u32) {
    self.data[self.count] = val;
    self.count += 1;
  }
}

fn process_single_action(action: u32, _: KeyState, buf: &mut USBBuffer) {
  buf.push(action);
}

fn process_actions<SM: Matrix<KeyState>, LM: Matrix<u32>>(states: SM, layout: LM) -> USBBuffer {
  let mut buf = USBBuffer::new();
  for c in 0..layout.get_num_columns() {
    for r in 0..layout.get_num_rows() {
      let state = states.get(r, c);
      match state {
        KeyState::None => {}
        _ => {
          let action = layout.get(r, c);
          process_single_action(action, state, &mut buf);
        }
      }
    }
  }
  buf
}

#[cfg(test)]
fn private_basic() {
  gen_matrix!(Matrix2x3u32, 2, 3, u32);
  gen_matrix!(Matrix2x3KS, 2, 3, KeyState);
  let states = Matrix2x3KS::new_with_data([KeyState::None,
                                           KeyState::Pressed,
                                           KeyState::None,
                                           KeyState::None,
                                           KeyState::None,
                                           KeyState::None]);
  let layout = Matrix2x3u32::new_with_data([0, 1, 2, 3, 4, 5]);
  let buf = process_actions(states, layout);
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
