pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let diagonal_1_1 = matrix[0][0] * matrix[1][1] * matrix[2][2];
    let diagonal_1_2 = matrix[0][1] * matrix[1][2] * matrix[2][0];
    let diagonal_1_3 = matrix[0][2] * matrix[1][0] * matrix[2][1];
    let diagonal_2_1 = matrix[0][2] * matrix[1][1] * matrix[2][0];
    let diagonal_2_2 = matrix[0][1] * matrix[1][0] * matrix[2][2];
    let diagonal_2_3 = matrix[0][0] * matrix[1][2] * matrix[2][1];

    diagonal_1_1 + diagonal_1_2 + diagonal_1_3 - (diagonal_2_1 + diagonal_2_2 + diagonal_2_3)
}
