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

    /// Retorna el determinante de la matriz.
    pub fn determinant(&self) -> Result<f64, &'static str> {
        // La matriz debe ser cuadrada
        if !self.is_square() {
            return Err("El determinante solo se puede calcular para matrices cuadradas.");
        }

        let rows = self.rows;
        let cols = self.cols;

        // Copio la matriz original
        let mut matrix = Matrix::new(rows, cols);
        for (i, j, val) in self {
            matrix.set(i, j, val).unwrap();
        }

        // El resultado de la determinante es un float
        let mut determinante = 1.0;

        // Evaluo cada fila
        for k in 0..rows {
            // Comparo la fila k con las siguientes
            for i in k + 1..rows {
                // Busco la relacion que hay entre la fila k y la fila i
                let factor = matrix.get(i, k).unwrap() / matrix.get(k, k).unwrap();

                // Resto a cada columna de la fila i los valores de la fila k multiplicados por el factor
                // fj + factor * fi
                for j in k..cols {
                    let new_value = matrix.get(i, j).unwrap() - factor * matrix.get(k, j).unwrap();
                    matrix.set(i, j, new_value).unwrap();
                }

                // Si el factor es negativo multiplico el determinante por -1
                // if factor.abs() != factor {
                //     determinante = -determinante;
                // }

                // matrix.set(i, k, 0.0).unwrap();
            }

            // A  ctualizo el determinante
            determinante *= matrix.get(k, k).unwrap();
        }
        // println!("Final Matrix: {}",matrix);
        // tranquilamente aca se puede hacer un for k in 0..rows determinante *= matrix[k][k]
        Ok(determinante)
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
            for i in k + 1..rows {
                let factor = original.get(i, k).unwrap() / original.get(k, k).unwrap();
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
            let factor = 1.0 / original.get(k, k).unwrap();
            for i in 0..cols {
                original
                    .set(k, i, factor * original.get(k, i).unwrap())
                    .unwrap();
                inverse
                    .set(k, i, factor * inverse.get(k, i).unwrap())
                    .unwrap();
            }
        }


        /*// Llevo el trianulo superor (inferior de la traspuesnta) a 0
        for k in (1..rows).rev() {
            // println!("Fila k {}", k);
            let mut i: usize = k;
            while i > 0 {
                i -= 1;
                // println!("Fila i {}", i);
                println!("Inversa: {}",inverse);
                let factor = original.get(i, k).unwrap();
                // println!("Factor: {} posicion {} {}\n", factor, i, k);
                for j in (0..cols).rev() {
                    // println!("i {}, j {}",i,j);
                    let new_value_original =
                        original.get(i, j).unwrap() - factor * original.get(k, j).unwrap();
                    let new_value_inverse =
                        inverse.get(i, j).unwrap() - factor * inverse.get(k, j).unwrap();
                    // println!("original[{}][{}] = {}",i,j,new_value_original);
                    // println!("inverse[{}][{}] = {}",i,j,new_value_inverse);
                    original.set(i, j, new_value_original).unwrap();
                    inverse.set(i, j, new_value_inverse).unwrap();
                    // println!("Original Movimiento Columna {} {}",j,original);
                    // println!("Invertida Movimiento Columna {} {}",j,inverse);
                }
            }
        } */

        // let mut t = Matrix::new(rows,cols);
        // for (row,col,val) in &original.transpose() {
        //     t.set(row, col, val).unwrap();
        // }


        println!("Pre triangulo superior");
        println!("Original: {}",original);
        // println!("Traspuesta: {}",t);
        println!("Inversa: {}",inverse);


        for k in (0..rows).rev() {
            println!("\nEvaluo de la fila k {} hacia atras", k);
            for i in (0..k).rev() {
                println!("Evaluo fila i {}",i);

                println!("Original: {}",original);
                println!("Inversa: {}",inverse);


                let factor = original.get(i, k).unwrap();
                println!("Busco el factor {}",factor);
                println!("f{} - {}*f{}",i,factor,k);
                for j in 0..cols {
                    let n_v_original = original.get(i,j).unwrap() - factor * original.get(k,j).unwrap();
                    let n_v_inverse = inverse.get(i,j).unwrap() - factor * inverse.get(k,j).unwrap();
                    original.set(i, j, n_v_original).unwrap();
                    inverse.set(i, j, n_v_inverse).unwrap();
                }
            }
        }



        println!("Original: {}",original);
        // println!("Traspuesta: {}",t);
        println!("Inversa: {}",inverse);

        Ok(inverse)
    } 
}
