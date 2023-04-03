// En este archivo se implementan métodos para imprimir una matriz en pantalla.
// Se encarga de que se vea lindo y bien justificado. Excede a la materia.

use super::Matrix;
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_number() {
            write!(f, "{}", self.data[0])
        } else {
            let mut elements = vec![vec![String::new(); self.cols]; self.rows];
            for (row, col, val) in self {
                elements[row][col] = format!("{}", val);
            }

            let mut widths = vec![0; self.cols];
            for row in 0..self.rows {
                for col in 0..self.cols {
                    widths[col] = widths[col].max(elements[row][col].len());
                }
            }

            write!(f, "\n")?;
            for row in 0..self.rows {
                for col in 0..self.cols {
                    if col == 0 {
                        write!(f, "\n")?;
                    }
                    let val = &elements[row][col];
                    let padding = 3 + widths[col] - val.len();
                    write!(f, "{}{}", " ".repeat(padding), val)?;
                }
            }
            write!(f, "\n")
        }
    }
}
