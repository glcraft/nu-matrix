use nu_matrix_common::methods::Method;
use crate::{
    context::PerSessionContext as Context,
    matrix::Matrix,
};

pub fn run(ctx: &mut Context, method: Method) -> Result<(), ()> {
    match method {
        Method::NewIdentity(x, y) => {
            ctx.add_matrix(Matrix::identity((x, y)));
            Ok(())
        }
        _ => unimplemented!(),
    }
}