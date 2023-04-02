// En este archivo se implementan métodos para iterar sobre una matriz.
// Meramente para hacer for-each sobre una matriz. Excede a la materia.

use super::{Matrix, MatrixItem};

impl<'a> IntoIterator for &'a Matrix {
    type Item = (usize, usize, MatrixItem);
    type IntoIter = MatrixIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIntoIterator {
            matrix: &self,
            index: 0,
        }
    }
}

pub struct MatrixIntoIterator<'a> {
    matrix: &'a Matrix,
    index: usize,
}

impl<'a> Iterator for MatrixIntoIterator<'a> {
    type Item = (usize, usize, MatrixItem);

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.matrix.data.get(self.index);
        if element.is_none() {
            return None;
        }
        let row = self.index / self.matrix.cols;
        let col = self.index % self.matrix.cols;
        self.index += 1;
        Some((row, col, *element.unwrap()))
    }
}
