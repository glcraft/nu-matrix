use crate::matrix::Matrix;
#[test]
fn matrix_identity() {
    let matrix = Matrix::identity((3, 3));
    assert_eq!(matrix.data, vec![
        1.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, 
        0.0, 0.0, 1.0
    ]);
}