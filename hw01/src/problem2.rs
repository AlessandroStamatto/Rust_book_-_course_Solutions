/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (n_linhas1, n_colunas1) = (mat1.len(), mat1[0].len());
    let (n_linhas2, n_colunas2) = (mat2.len(), mat2[0].len());
    assert_eq!(n_colunas1, n_linhas2);

    let linha = vec![0.0; n_colunas2];
    let mut res = vec![linha; n_linhas1];

    for i in 0..n_linhas1 {
    	for j in 0..n_colunas2 {
    		for k in 0..n_colunas1 {
    			res[i][j] += mat1[i][k] * mat2[k][j];
    		}
    	}
    }

    res
}