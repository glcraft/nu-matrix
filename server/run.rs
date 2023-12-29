use nu_matrix_common::methods::{Method, Response};
use crate::{
    context::PerSessionContextInstance as ContextInstance,
    matrix::Matrix,
};

pub fn run(ctx: ContextInstance, method: Method) -> Result<Response, ()> {
    match method {
        Method::NewIdentity(x, y) => {
            let new_id = ctx.lock().unwrap().add_matrix(Matrix::identity((x, y)));
            Ok(Response::NewIdentity{ id: new_id })
        }
        _ => unimplemented!(),
    }
}