use matrix::Matrix;

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum KeyState {
  None,
  Pressed,
  Held,
  Released,
}

impl Default for KeyState {
  fn default() -> KeyState {
    KeyState::None
  }
}

fn process_key_state<RM, SM>(result: RM, previous_result: RM) -> SM
  where RM: Matrix<u32>,
        SM: Matrix<KeyState>
{
  let mut res = SM::new();
  for c in 0..result.get_num_columns() {
    for r in 0..result.get_num_rows() {
      let state = if result.get(r, c) == 1 {
        if previous_result.get(r, c) == 1 {
          KeyState::Held
        } else {
          KeyState::Pressed
        }
      } else {
        if previous_result.get(r, c) == 1 {
          KeyState::Released
        } else {
          KeyState::None
        }
      };
      res.put(r, c, state);
    }
  }
  res
}

#[cfg(test)]
fn private_basic() {
  gen_matrix!(Matrix2x3u32, 2, 2, u32);
  gen_matrix!(Matrix2x3KS, 2, 2, KeyState);
  let r0 = Matrix2x3u32::new();
  let r1 = Matrix2x3u32::new();
  let result: Matrix2x3KS = process_key_state(r0, r1);
  assert_eq!(result.data, [KeyState::None; 4]);

  let r0 = Matrix2x3u32::new_with_data([1, 0, 1, 0]);
  let r1 = Matrix2x3u32::new_with_data([0, 1, 1, 0]);
  let result: Matrix2x3KS = process_key_state(r0, r1);
  assert_eq!(result.data,
             [KeyState::Pressed, KeyState::Released, KeyState::Held, KeyState::None]);
}

#[cfg(test)]
mod tests {
  use super::private_basic;
  #[test]
  fn basic() {
    private_basic();
  }
}
