use action::USBBuffer;
use keycode::USBKeyCode;
use keycode::USBKeyMod;
use matrix::Matrix;
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

#[derive(Copy, Clone)]
pub enum Action {
  None,
  Key(USBKeyCode),
  Mod(USBKeyMod),
}

impl Default for Action {
  fn default() -> Action {
    Action::None
  }
}

impl From<USBKeyCode> for Action {
  fn from(val: USBKeyCode) -> Action {
    Action::Key(val)
  }
}

impl From<USBKeyMod> for Action {
  fn from(val: USBKeyMod) -> Action {
    Action::Mod(val)
  }
}

pub struct DefaultLayout<M: Matrix> {
  pub map: M,
}

impl<M> Layout for DefaultLayout<M>
where
  M: Matrix<T = Action>,
{
  fn process_action(
    &self,
    row: usize,
    col: usize,
    state: KeyState,
    buf: &mut USBBuffer,
  ) {
    match self.map.get(row, col) {
      Action::Key(code) => {
        if state == KeyState::Pressed || state == KeyState::Held {
          buf.push(code as u32);
        }
      }
      Action::Mod(code) => {
        if state == KeyState::Pressed || state == KeyState::Held {
          buf.push_mod(code as u32);
        }
      }
      Action::None => {}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn basic() {
    gen_matrix!(ActionMap, 2, 2, Action);
    let m = DefaultLayout::<ActionMap> {
      map: ActionMap::new_with_data([
        Action::Key(USBKeyCode::A),
        Action::Key(USBKeyCode::A),
        Action::Key(USBKeyCode::A),
        Action::Mod(USBKeyMod::LShift),
      ]),
    };
    let mut buf = USBBuffer::new();
    m.process_action(0, 0, KeyState::Pressed, &mut buf);
    assert_eq!(buf.data, [USBKeyCode::A as u32, 0, 0, 0, 0, 0]);
    let mut buf = USBBuffer::new();
    m.process_action(1, 1, KeyState::Pressed, &mut buf);
    assert_eq!(buf.mods, USBKeyMod::LShift as u32);
  }
}
