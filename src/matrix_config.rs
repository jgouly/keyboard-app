pub struct RowPins<'a, MC: MatrixConfig + 'a> {
  count: usize,
  conf: &'a MC,
}

impl<'a, MC: MatrixConfig + 'a> RowPins<'a, MC> {
  fn new(conf: &'a MC) -> RowPins<'a, MC> {
    RowPins {
      count: 0,
      conf: conf,
    }
  }
}

impl<'a, MC: MatrixConfig> Iterator for RowPins<'a, MC> {
  type Item = MC::InputPin;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count == self.conf.get_num_rows() {
      None
    } else {
      let idx = self.count;
      self.count += 1;
      Some(self.conf.get_row_pin(idx))
    }
  }
}

pub struct ColumnPins<'a, MC: MatrixConfig + 'a> {
  count: usize,
  conf: &'a MC,
}

impl<'a, MC: MatrixConfig + 'a> ColumnPins<'a, MC> {
  fn new(conf: &'a MC) -> ColumnPins<'a, MC> {
    ColumnPins {
      count: 0,
      conf: conf,
    }
  }
}

impl<'a, MC: MatrixConfig> Iterator for ColumnPins<'a, MC> {
  type Item = MC::OutputPin;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count == self.conf.get_num_columns() {
      None
    } else {
      let idx = self.count;
      self.count += 1;
      Some(self.conf.get_column_pin(idx))
    }
  }
}

pub trait MatrixConfig {
  type InputPin;
  type OutputPin;

  fn get_num_rows(&self) -> usize;
  fn get_num_columns(&self) -> usize;

  fn get_row_pin(&self, idx: usize) -> Self::InputPin;
  fn get_column_pin(&self, idx: usize) -> Self::OutputPin;

  fn rows(&self) -> RowPins<Self>
    where Self: Sized
  {
    RowPins::new(&self)
  }
  fn columns(&self) -> ColumnPins<Self>
    where Self: Sized
  {
    ColumnPins::new(&self)
  }
}

#[cfg(test)]
mod tests {
  use super::MatrixConfig;
  #[test]
  fn basic() {
    struct TestMatrixConfig {}
    impl MatrixConfig for TestMatrixConfig {
      type InputPin = u32;
      type OutputPin = u32;
      fn get_num_rows(&self) -> usize {
        2
      }
      fn get_num_columns(&self) -> usize {
        3
      }
      fn get_row_pin(&self, idx: usize) -> Self::InputPin {
        idx as u32
      }
      fn get_column_pin(&self, idx: usize) -> Self::OutputPin {
        idx as u32
      }
    }
    let m = TestMatrixConfig {};
    let mut result = Vec::new();
    let columns = m.columns();
    for c in columns {
      let rows = m.rows();
      result.push(c);
      for r in rows {
        result.push(r);
      }
    }
    assert_eq!(result, [0, 0, 1, 1, 0, 1, 2, 0, 1]);
  }
}
