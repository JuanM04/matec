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
    // -----------------
    // Métodos estáticos
    // -----------------
    //
    // Estos métodos se llaman como `Matrix::new(...)`, `Matrix::add(...)`

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

    /// Crea una matriz identidad de MxM elementos.
    pub fn identity(size: usize) -> Matrix {
        let mut matrix = Matrix::new(size, size);
        for i in 0..size {
            matrix.data[i * size + i] = 1.0;
        }
        matrix
    }

    /// Crea una matriz elemental de tipo I.
    /// Permuta las filas `i` y `j`.
    pub fn elemental_i(size: usize, i: usize, j: usize) -> Result<Matrix, &'static str> {
        let mut matrix = Matrix::identity(size);
        matrix.set(i, i, 0.0)?;
        matrix.set(i, j, 1.0)?;
        matrix.set(j, j, 0.0)?;
        matrix.set(j, i, 1.0)?;
        Ok(matrix)
    }

    /// Crea una matriz elemental de tipo II.
    /// Multiplica la fila `i` por el escalar `scalar`.
    pub fn elemental_ii(size: usize, i: usize, scalar: f64) -> Result<Matrix, &'static str> {
        let mut matrix = Matrix::identity(size);
        matrix.set(i, i, scalar)?;
        Ok(matrix)
    }

    /// Crea una matriz elemental de tipo III.
    /// Le suma a la fila `i` el producto de la fila `j` por el escalar `scalar`.
    pub fn elemental_iii(
        size: usize,
        i: usize,
        j: usize,
        scalar: f64,
    ) -> Result<Matrix, &'static str> {
        let mut matrix = Matrix::identity(size);
        matrix.set(i, j, scalar)?;
        Ok(matrix)
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
                // Inicializa el elemento Cmp en 0.
                let mut sum: f64 = 0.0;

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

    // --------------------
    // Métodos de instancia
    // --------------------
    //
    // Dada una matriz ya inicializada (`let matrix = Matrix::new(2, 2);`), se pueden
    // llamar como `matrix.rows()` o `matrix.is_square()`. El parámetro `&self` es
    // la referencia a la matriz que se está usando.

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

    /// Calcula la potencia de una matriz cuadrada. Retorna una nueva matriz.
    pub fn pow(&self, exp: f64) -> Result<Matrix, String> {
        if !self.is_square() {
            return Err("La potencia solo está definida para matrices cuadradas".to_string());
        }
        if !nearly_equal(exp.fract(), 0.0) {
            return Err("La potencia solo está definida para exponentes enteros".to_string());
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
        // Una vez intercambiada, se resta a cada fila i > k la fila k multiplicada por Aik/Akk.
        // Así, los elementos de la columna k quedan en 0 para esas filas.
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

        // Finalmente, retorno el determinante
        Ok(determinant)
    }

    /// Retorna la inversa de la matriz.
    /// Se calcula obteniendo la forma escalonada reducida de Gauss-Jordan.
    pub fn inverse(&self) -> Result<Matrix, String> {
        // La matriz debe ser cuadrada (porque solo sabemos invertir matrices cuadradas)
        if !self.is_square() {
            return Err("La inversa de matrices rectangulares no está implementada".to_string());
        }

        // El determinante debe ser distinto de 0.
        // Calculamos primero el determinante porque, para matrices grandes, es mucho más
        // eficiente para determinar si la matriz tiene inversa.
        let determinant = self.determinant().unwrap_or(0.0);
        if nearly_equal(determinant, 0.0) {
            return Err("La matriz no tiene inversa porque su determinante es 0".to_string());
        }

        // número de filas y columnas
        let n = self.rows;
        // clono la matriz para no modificar la original
        let mut matrix = self.clone();
        // creo la matriz identidad de nxn
        let mut accum = Matrix::identity(n);

        // Recorro la diagonal.
        // Como la matriz es cuadrada, me basta con un único índice que va desde 0 a n-1.
        //
        // La estrategia es buscar la primera fila tal que Aik != 0 e intercambiarla con la fila k.
        // Si no existe tal fila, la matriz no tiene inversa.
        //
        // Una vez intercambiada, se divide cada elemento de la fila k por Akk. Así, Akk = 1.
        // Luego, se resta a cada fila i != k la fila k multiplicada por Aik. Así, los elementos
        // de la columna k quedan en 0 para esas filas.
        //
        // Todo esto para que quede una matriz identidad. A la par, se aplican las mismas
        // operaciones a la matriz acumuladora, que es la matriz identidad que se va
        // multiplicando por la inversa de la matriz original. Finalmente, la matriz
        // acumuladora será la inversa de la matriz original.
        for k in 0..n {
            // Obtengo el elemento de la diagonal (Akk, que será el pivote)
            let mut pivot = matrix.get(k, k).unwrap();
            if nearly_equal(pivot, 0.0) {
                // Busco la primera fila tal que Aik != 0
                let mut found = false;
                // Solo busco en las filas k+1 a n-1, ya que las filas anteriores ya están en 0
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
                    // Nota: este mensaje no se debería mostrar nunca, ya que el determinante
                    // debería ser 0. Como nadie quiere un bucle infinito, lo dejo por las dudas.
                    return Err(
                        "La matriz tiene una columna de ceros, por ende, no tiene inversa"
                            .to_string(),
                    );
                } else {
                    // Creo la matriz elemental que permuta la fila k con la fila i
                    let permutation = Matrix::elemental_i(n, k, i)?;
                    // Multiplico la matriz y la acumuladora por la matriz elemental
                    matrix = Matrix::multiply(&permutation, &matrix)?;
                    accum = Matrix::multiply(&permutation, &accum)?;
                }
            }

            // Ahora, toca dividir cada elemento de la fila k por Akk.
            // Creo la matriz elemental que divide la fila k por Akk
            let scale = Matrix::elemental_ii(n, k, 1.0 / pivot)?;
            // Multiplico la matriz y la acumuladora por la matriz elemental
            matrix = Matrix::multiply(&scale, &matrix)?;
            accum = Matrix::multiply(&scale, &accum)?;

            // Ahora, toca restar a cada fila i != k la fila k multiplicada por Aik.
            for i in 0..n {
                if i != k {
                    // factor = -Aik
                    let factor = -matrix.get(i, k).unwrap();

                    // Creo la matriz elemental que resta a la fila i la fila k multiplicada por Aik
                    let elimination = Matrix::elemental_iii(n, i, k, factor)?;
                    // Multiplico la matriz y la acumuladora por la matriz elemental
                    matrix = Matrix::multiply(&elimination, &matrix)?;
                    accum = Matrix::multiply(&elimination, &accum)?;
                }
            }
        }

        // Finalmente, retorno la matriz acumuladora
        Ok(accum)
    }
}
