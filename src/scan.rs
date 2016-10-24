use matrix::Matrix;
use matrix_config::MatrixConfig;

pub trait InputPin {
  fn read_input(&self) -> u32;
}

pub trait OutputPin {
  fn set_low(&self);
  fn set_high(&self);
}

pub fn single_scan<MC: MatrixConfig, RM: Matrix<u32>>(conf: &MC) -> RM
  where MC::InputPin: InputPin,
        MC::OutputPin: OutputPin
{
  let mut res = RM::new();
  for c in conf.columns().enumerate() {
    c.1.set_high();
    for r in conf.rows().enumerate() {
      res.put(r.0, c.0, r.1.read_input());
    }
    c.1.set_low();
  }
  res
}

#[cfg(test)]
pub fn private_basic() {
  struct TestMatrixConfig {};
  struct TestPin(u32);
  impl InputPin for TestPin {
    fn read_input(&self) -> u32 {
      self.0
    }
  }
  impl OutputPin for TestPin {
    fn set_low(&self) {}
    fn set_high(&self) {}
  }
  impl MatrixConfig for TestMatrixConfig {
    type InputPin = TestPin;
    type OutputPin = TestPin;
    fn get_num_rows(&self) -> usize {
      2
    }
    fn get_num_columns(&self) -> usize {
      3
    }
    fn get_row_pin(&self, idx: usize) -> Self::InputPin {
      TestPin(idx as u32)
    }
    fn get_column_pin(&self, idx: usize) -> Self::OutputPin {
      TestPin(idx as u32)
    }
  }
  gen_matrix!(M2, 2, 3, u32);
  let conf = TestMatrixConfig {};
  let x: M2 = single_scan(&conf);
  assert_eq!(x.data, [0, 0, 0, 1, 1, 1]);
}

#[cfg(test)]
mod tests {
  #[test]
  fn basic() {
    use super::private_basic;
    private_basic();
  }
}
