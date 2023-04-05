// En este archivo se implementa la estructura de datos `Matrix` y sus métodos.

mod display;
mod iter;

/// Cada elemento de la matriz es un `double` (punto flotante de 64 bits)
type MatrixItem = f64;

/// Internamente, cada matriz se almacena como un vector de
/// MxN elementos, donde M es el número de filas y N el número de columnas.
#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<MatrixItem>,
}

impl Matrix {
    /// Crea una matriz de MxN elementos, todos inicializados en 0.
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Crea una matriz de 1x1 elemento, inicializado con el valor `scalar`.
    pub fn from_scalar(scalar: MatrixItem) -> Matrix {
        Matrix {
            rows: 1,
            cols: 1,
            data: vec![scalar],
        }
    }

    /// Crea una matriz a partir de un vector de vectores. Útil cuando se
    /// quiere crear una matriz a partir de datos de entrada.
    pub fn from_2d(nested_vec: Vec<Vec<MatrixItem>>) -> Result<Matrix, &'static str> {
        let rows = nested_vec.len();
        if rows == 0 {
            return Ok(Matrix {
                rows: 0,
                cols: 0,
                data: Vec::new(),
            });
        }

        let cols = nested_vec[0].len();
        let mut matrix = Matrix::new(rows, cols);
        for (i, row) in nested_vec.iter().enumerate() {
            if row.len() != cols {
                return Err("Todas las filas deben tener la misma cantidad de columnas");
            }

            for (j, &val) in row.iter().enumerate() {
                matrix.data[i * cols + j] = val;
            }
        }
        Ok(matrix)
    }

    /// Obtiene el elemento en la posición (row, col).
    pub fn get(&self, row: usize, col: usize) -> Result<MatrixItem, &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Indice fuera de rango");
        }

        Ok(self.data[row * self.cols + col])
    }

    /// Cambia el elemento en la posición (row, col) con el valor `val`.
    pub fn set(&mut self, row: usize, col: usize, val: MatrixItem) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Indice fuera de rango");
        }

        self.data[row * self.cols + col] = val;
        Ok(())
    }

    /// Obtiene el número de filas de la matriz.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Obtiene el número de columnas de la matriz.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Retorna `true` si la matriz es cuadrada.
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Retorna `true` si la matriz es una matriz identidad.
    pub fn is_identity(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        for (i, j, val) in self {
            if i == j && val != 1.0 {
                return false;
            } else if i != j && val != 0.0 {
                return false;
            }
        }
        true
    }

    /// Retorna `true` si es una matriz 1x1.
    pub fn is_number(&self) -> bool {
        self.rows == 1 && self.cols == 1
    }

    /// Suma dos matrices y retorna una nueva matriz.
    pub fn add(&self, right: &Matrix) -> Result<Matrix, &'static str> {
        if self.rows != right.rows || self.cols != right.cols {
            return Err("La suma de matrices solo está definida para matrices de igual dimensión");
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for (i, j, val) in self {
            result.set(i, j, val + right.get(i, j)?)?;
        }
        Ok(result)
    }

    /// Multiplica dos matrices (MxN y NxP) y retorna una nueva matriz (MxP).
    pub fn mul(&self, right: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != right.rows {
            return Err(
                "La multiplicación de matrices solo está definida para matrices de MxN y NxP",
            );
        }

        // El resultado de la multiplicación de matrices es una matriz MxP.
        let mut result = Matrix::new(self.rows, right.cols);

        for m in 0..result.rows {
            for p in 0..result.cols {
                let mut sum = 0_f64;

                // Suma de los productos de los elementos de la fila i de la matriz izquierda
                // con los elementos de la columna j de la matriz derecha.
                for n in 0..self.cols {
                    sum += self.get(m, n)? * right.get(n, p)?;
                }
                result.set(m, p, sum)?;
            }
        }

        Ok(result)
    }

    /// Retorna la traspuesta de la matriz.
    pub fn transpose(&self) -> Matrix {
        // La traspuesta de una matriz MxN es una matriz NxM.
        let mut result = Matrix::new(self.cols, self.rows);
        for (i, j, val) in self {
            result.set(j, i, val).unwrap();
        }
        result
    }

    /// Retorna la determinante
    pub fn determinante(&self) -> Result<f64, &'static str> {
        // La matriz debe ser cuadrada para calcular el determinante
        if !self.is_square() {
            return Err(
                "El determinante solo se puede calcular para matrices cuadradas.",
            );
        }

        let rows = self.rows;
        let mut det = 1.0;

        // Copia de la matriz para no modificar la original
        // ? wow, self,data.clone() esta buenisimo, lo pasa a un vector
        let mut matrix = self.data.clone();
        
        for k in 0..rows-1 {
            // Busqueda la fila con el valor absoluto máximo en la columna k
            let mut i_max = k;
            for i in k+1..rows {
                if matrix[i*rows + k].abs() > matrix[i_max*rows + k].abs() {
                    i_max = i;
                }
            }

            // Intercambio la fila i_max con la fila k si i_max != k
            if i_max != k {
                for j in k..rows {
                    let tmp = matrix[k*rows + j];
                    matrix[k*rows + j] = matrix[i_max*rows + j];
                    matrix[i_max*rows + j] = tmp;
                }
                det = -det;
            }

            // Si el elemento diagonal es cero, la matriz es singular y el determinante es cero
            if matrix[k*rows + k] == 0.0 {
                return Ok(0.0);
            }

            // Eliminación gaussiana de la columna k
            for i in k+1..rows {
                let factor = matrix[i*rows + k] / matrix[k*rows + k];
                for j in k+1..rows {
                    matrix[i*rows + j] -= factor * matrix[k*rows + j];
                }
                matrix[i*rows + k] = 0.0;
            }

            // Actualizar el determinante
            det *= matrix[k*rows + k];
        }

        det *= matrix[(rows-1)*rows + rows-1];
        Ok(det)
    } 


    

}
