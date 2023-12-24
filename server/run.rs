use nu_matrix_common::methods::{Method, Response};
use crate::{
    context::PerSessionContext as Context,
    matrix::Matrix,
};

pub fn run(ctx: &mut Context, method: Method) -> Result<Response, ()> {
    match method {
        Method::NewIdentity(x, y) => {
            let new_id = ctx.add_matrix(Matrix::identity((x, y)));
            Ok(Response::NewIdentity{ id: new_id })
        }
        _ => unimplemented!(),
    }
}