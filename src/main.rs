extern crate itertools;

use itertools::Itertools;
use std::fmt;

#[derive(Debug)]
struct Coord {
  x: u32,
  y: u32,
}

#[derive(Debug)]
struct Cell<T> {
  value: T,
  coords: Coord,
}

#[derive(Debug)]
struct Row<T> {
  id: u32,
  cells: Vec<Cell<T>>,
}

#[derive(Debug)]
struct Matrix<T> {
  rows: Vec<Row<T>>,
}

impl<T> Matrix<T> {
  fn new<I>(input: I) -> Matrix<T>
  where
    T: fmt::Debug,
    I: IntoIterator<Item = Vec<T>>,
  {
    let mut rows = vec![];

    for (row_index, row) in input.into_iter().enumerate() {
      let mut cells: Vec<Cell<T>> = vec![];

      for (col_index, col) in row.into_iter().enumerate() {
        let coords = Coord {
          x: col_index as u32,
          y: row_index as u32,
        };

        cells.push(Cell { value: col, coords })
      }

      rows.push(Row {
        id: row_index as u32,
        cells,
      });
    }

    Matrix { rows }
  }
}

impl<T> fmt::Display for Matrix<T>
where
  T: fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut rows = String::new();

    for row in self.rows.iter() {
      let cells = row
        .cells
        .iter()
        .map(|cell| format!("{:?}", cell.value))
        .join(", ");

      rows.push_str(&format!("[{}]\n", cells));
    }

    write!(f, "{}", rows)
  }
}

macro_rules! matrix {
  () => {
    Matrix::new(vec![])
  };

  ($( $( $x: expr ),*);*) => {
    Matrix::new(vec![ $( vec![ $($x),* ] ),* ])
  }
}

fn main() {
  let fib_matrix = matrix![
    0, 1, 1;
    2, 3, 5;
    8, 13, 21
  ];

  println!("{}", fib_matrix);
}
