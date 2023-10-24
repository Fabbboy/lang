#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodePos {
  pub line: usize,
  pub col: usize,
}

impl CodePos {
  pub fn new(line: usize, col: usize) -> CodePos {
    CodePos {
      line: line,
      col: col,
    }
  }
  pub fn zero() -> CodePos {
    CodePos { line: 0, col: 0 }
  }
}
