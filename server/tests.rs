use std::ops::Deref;

use crate::matrix::Matrix;
#[test]
fn matrix_identity() {
    let matrix = Matrix::identity((3, 3));
    assert_eq!(matrix.deref(), vec![
        1.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, 
        0.0, 0.0, 1.0
    ]);
}

#[test]
fn until_future() {
    use std::time::Duration;
    use nu_matrix_common::future::timeout;

    let fut_func = || async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        1
    };
    
    let runtime =  tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(async {
        let wait1 = timeout(fut_func(), Duration::from_millis(50));
        let wait2 = timeout(fut_func(), Duration::from_millis(500));
        assert_eq!(wait1.await, None);
        assert_eq!(wait2.await, Some(1));
    })

    
}