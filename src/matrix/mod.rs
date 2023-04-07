// En este archivo se implementa la estructura de datos `Matrix` y sus métodos.
// Aquí se encuentran las implementaciones de
// - Suma de matrices
// - Multiplicación de matrices
// - Multiplicación de matrices por un escalar
// - Obtención de la matriz transpuesta
// - Obtención de la matriz inversa
// - Obtención del determinante de una matriz

use crate::utils::nearly_equal;

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

    /// Crea una matriz identidad de MxM elementos.
    pub fn identity(size: usize) -> Matrix {
        let mut matrix = Matrix::new(size, size);
        for i in 0..size {
            matrix.data[i * size + i] = 1.0;
        }
        matrix
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

        // Recorre la matriz y verifica que todos los elementos
        // en la diagonal principal sean 1 y los demás sean 0.
        for (i, j, val) in self {
            if i == j && !nearly_equal(val, 1.0) {
                return false;
            } else if i != j && !nearly_equal(val, 0.0) {
                return false;
            }
        }
        true
    }

    /// Suma dos matrices y retorna una nueva matriz.
    pub fn add(left: &Matrix, right: &Matrix) -> Result<Matrix, &'static str> {
        if left.rows != right.rows || left.cols != right.cols {
            return Err("La suma de matrices solo está definida para matrices de igual dimensión");
        }

        let mut result = Matrix::new(left.rows, left.cols);
        for i in 0..result.rows {
            for j in 0..result.cols {
                // Esto es Aij + Bij = Cij
                let val = left.get(i, j)? + right.get(i, j)?;
                result.set(i, j, val)?;
            }
        }
        Ok(result)
    }

    /// Multiplica dos matrices (MxN y NxP) y retorna una nueva matriz (MxP).
    pub fn multiply(left: &Matrix, right: &Matrix) -> Result<Matrix, &'static str> {
        if left.cols != right.rows {
            return Err(
                "La multiplicación de matrices solo está definida para matrices de MxN y NxP",
            );
        }

        // El resultado de la multiplicación de matrices es una matriz MxP.
        let mut result = Matrix::new(left.rows, right.cols);

        for m in 0..result.rows {
            for p in 0..result.cols {
                let mut sum = 0_f64;

                // Suma de los productos de los elementos de la fila i de la matriz izquierda
                // con los elementos de la columna j de la matriz derecha.
                for n in 0..left.cols {
                    // Esto es Amn * Bnp
                    sum += left.get(m, n)? * right.get(n, p)?;
                }
                result.set(m, p, sum)?;
            }
        }

        Ok(result)
    }

    /// Calcula la potencia de una matriz cuadrada. Retorna una nueva matriz.
    pub fn pow(&self, exp: f64) -> Result<Matrix, &'static str> {
        if !self.is_square() {
            return Err("La potencia solo está definida para matrices cuadradas");
        }
        if !nearly_equal(exp.fract(), 0.0) {
            return Err("La potencia solo está definida para exponentes enteros");
        }

        // Si el exponente es negativo, calcula la inversa de la matriz.
        let base = if exp < 0.0 {
            self.inverse()?
        } else {
            self.clone()
        };

        let exp = exp.abs() as usize;

        let mut result = Matrix::identity(self.rows);
        for _ in 0..exp {
            // Realiza la multiplicación de la matriz por la pase.
            result = Matrix::multiply(&base, &result)?;
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

    /// Multiplica la matriz por un escalar y retorna una nueva matriz.
    pub fn scale(&self, scalar: MatrixItem) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for (i, j, val) in self {
            // Multiplica cada elemento de la matriz por el escalar.
            result.set(i, j, val * scalar).unwrap();
        }
        result
    }
    /// Calcula y retorna el determinante de la matriz.
    /// Se calcula mediante eliminación gaussiana en vez de por
    /// expansión de cofactores debido a su eficiencia.
    pub fn determinant(&self) -> Result<MatrixItem, &'static str> {
        // La matriz debe ser cuadrada
        if !self.is_square() {
            return Err("El determinante solo está definida para matrices cuadradas.");
        }

        // Clono la matriz para no modificar la original
        let mut matrix = self.clone();
        let n = matrix.rows; // número de filas y columnas
        let mut determinant: f64 = 1.0;

        // Recorro la diagonal.
        // Como la matriz es cuadrada, me basta con un único índice que va desde 0 a n-1.
        //
        // La estrategia es buscar la primera fila tal que Aik != 0 e intercambiarla con la fila k.
        // Si no existe tal fila, el determinante es 0.
        //
        // Una vez intercambiada, se divide cada elemento de la fila k por Akk.
        // Luego, se resta a cada fila i > k la fila k multiplicada por Aik/Akk. Así, los elementos
        // de la columna k quedan en 0 para esas filas.
        //
        // Todo esto para que quede una matriz triangular superior. Así, el determinante es el
        // producto de los elementos de la diagonal.
        for k in 0..n {
            // Obtengo el elemento de la diagonal (Akk, que será el pivote)
            let mut pivot = matrix.get(k, k).unwrap();
            if nearly_equal(pivot, 0.0) {
                // Busco la primera fila tal que Aik != 0
                let mut found = false;
                let mut i = k + 1;
                while !found && i < n {
                    pivot = matrix.get(i, k).unwrap();
                    if nearly_equal(pivot, 0.0) {
                        i += 1;
                    } else {
                        found = true;
                    }
                }
                if !found {
                    // No existe tal fila, el determinante es 0
                    return Ok(0.0);
                } else {
                    // Intercambio la fila k con la fila i, intercambiando
                    // el valor de cada fila columna por columna.
                    for j in 0..n {
                        let tmp = matrix.get(k, j)?;
                        matrix.set(k, j, matrix.get(i, j)?)?;
                        matrix.set(i, j, tmp)?;
                    }
                    // Cambio el signo del determinante, ya que det(E tipo I) = -1
                    determinant = -determinant;
                }
            }

            // Ahora, toca restar a cada fila i > k la fila k multiplicada por Aik/Akk

            // Itero fila por fila
            for i in (k + 1)..n {
                // factor = Aik / Akk
                let factor = matrix.get(i, k)? / pivot;

                // Resto a cada elemento de la fila i la fila k multiplicada por el factor
                // Nótese que itero sobre las columnas k a n-1, ya que las columnas anteriores
                // ya están en 0.
                for j in k..n {
                    let new_value = matrix.get(i, j)? - factor * matrix.get(k, j)?;
                    matrix.set(i, j, new_value)?;
                }

                // No hace falta actualizar el determinante, ya que una operación tipo III
                // no cambia el valor del determinante.
            }

            // Como mi objetivo es calcular el determinante de una matriz triangular superior,
            // y ya sé que el valor de este elemento de la diagonal no cambiará, lo multiplico
            // directamente al determinante acumulado.
            determinant *= matrix.get(k, k).unwrap();
        }

        Ok(determinant)
    }

    /// Retorna la inversa calculada con Gauss Jhordan
    pub fn inverse(&self) -> Result<Matrix, &'static str> {
        if !self.is_square() || self.determinant() == Ok(0.0) {
            return Err(
                "La inversa de una matriz solo se puede calcular si su determinte es distinto de cero."
            );
        }

        // Configuro variables
        let rows = self.rows;
        let cols = self.cols;

        let mut original = Matrix::new(rows, cols);
        for (i, j, val) in self {
            original.set(i, j, val).unwrap();
        }

        let mut inverse = Matrix::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                if i == j {
                    inverse.set(i, j, 1.0).unwrap();
                } else {
                    inverse.set(i, j, 0.0).unwrap();
                }
            }
        }

        // Llevo el trianulo inferior a 0
        for k in 0..rows {
            // Busco el pivote
            let mut i_max = k;
            for i in k + 1..rows {
                // si el primer indice de la fila es mayor al de la fila k se rota para que
                //   en la fila k que de la fila con mayor indice
                if original.get(i, k).unwrap().abs() > original.get(i_max, k).unwrap().abs() {
                    for j in 0..cols {
                        let tmp = original.get(i_max, j).unwrap();
                        original.set(i_max, j, original.get(i, j).unwrap()).unwrap();
                        original.set(i, j, tmp).unwrap();
                        let tmp = inverse.get(i_max, j).unwrap();
                        inverse.set(i_max, j, inverse.get(i, j).unwrap()).unwrap();
                        inverse.set(i, j, tmp).unwrap();
                    }
                    i_max = i;
                }
            }

            // Llevo el triangulo inferior a 0
            for i in k + 1..rows {
                // obtengo la relacion entre la fila k y la fila i
                let factor = original.get(i, k).unwrap() / original.get(k, k).unwrap();
                // fi - factor * fk
                for j in k..cols {
                    let new_value_original =
                        original.get(i, j).unwrap() - factor * original.get(k, j).unwrap();
                    let new_value_inverse =
                        inverse.get(i, j).unwrap() - factor * inverse.get(k, j).unwrap();
                    original.set(i, j, new_value_original).unwrap();
                    inverse.set(i, j, new_value_inverse).unwrap();
                }
            }
        }

        // llevo la diagonal a 1
        for k in 0..rows {
            // obtengo el inverso
            let factor = 1.0 / original.get(k, k).unwrap();
            // multiplico cada undice de la fila por el factor para que quede en 1
            for i in 0..cols {
                original
                    .set(k, i, factor * original.get(k, i).unwrap())
                    .unwrap();
                inverse
                    .set(k, i, factor * inverse.get(k, i).unwrap())
                    .unwrap();
            }
        }
        // Llevo el trianulo superior a 0
        // recorro cada fila de abajo hacia arriba
        for k in (0..rows).rev() {
            // recorro las filas de arriba a k
            for i in (0..k).rev() {
                // obtengo el factor entre la fila k y la fila i
                let factor = original.get(i, k).unwrap();
                // resto a la fila i factor veces la fila k
                for j in 0..cols {
                    let new_value_original =
                        original.get(i, j).unwrap() - factor * original.get(k, j).unwrap();
                    let new_value_inverse =
                        inverse.get(i, j).unwrap() - factor * inverse.get(k, j).unwrap();
                    original.set(i, j, new_value_original).unwrap();
                    inverse.set(i, j, new_value_inverse).unwrap();
                }
            }
        }
        Ok(inverse)
    }
}
