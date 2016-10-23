pub trait Matrix<T> {
  fn new() -> Self;
  fn get(&self, row: usize, col: usize) -> T;
  fn put(&mut self, row: usize, col: usize, val: T);
  fn get_num_rows(&self) -> usize;
  fn get_num_columns(&self) -> usize;
}

#[macro_export]
macro_rules! gen_matrix {
  ($name: ident, $rows: expr, $cols: expr, $T: ty) => {
    struct $name {
      data: [$T; $rows * $cols]
    }
    impl $name {
      #[allow(dead_code)]
      fn new_with_data(data: [$T; $rows * $cols]) -> $name {
        $name { data: data }
      }
    }
    impl Matrix<$T> for $name {
      fn new() -> $name {
        $name { data: [<$T as Default>::default(); $rows * $cols] }
      }
      fn get(&self, row: usize, col: usize) -> $T {
        let c = self.get_num_columns();
        self.data[col + (row * c)]
      }
      fn put(&mut self, row: usize, col: usize, val: $T) {
        let c = self.get_num_columns();
        self.data[col + (row * c)] = val;
      }
      fn get_num_rows(&self) -> usize { $rows }
      fn get_num_columns(&self) -> usize { $cols }
    }
  }
}

#[cfg(test)]
mod tests {
  use matrix::Matrix;
  #[test]
  fn basic() {
    gen_matrix!(Matrix2x3, 2, 3, u32);
    let m = Matrix2x3 { data: [0, 1, 2, 3, 4, 5] };
    assert_eq!(0, m.get(0, 0));
    assert_eq!(1, m.get(0, 1));
    assert_eq!(4, m.get(1, 1));
    assert_eq!(5, m.get(1, 2));
    assert_eq!(2, m.get_num_rows());
    assert_eq!(3, m.get_num_columns());
    let mut m = Matrix2x3 { data: [0, 1, 2, 3, 4, 5] };
    m.put(0, 0, 10);
    assert_eq!(10, m.get(0, 0));
  }
}
