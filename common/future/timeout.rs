use futures::Future;
use std::task::Poll;

#[pin_project::pin_project]
pub struct Timeout<F>(#[pin] tokio::time::Timeout<F>);

impl<F> Future for Timeout<F> 
where
    F: Future,
    Self: Sized,
{
    type Output = Option<F::Output>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        match me.0.poll(cx) {
            Poll::Ready(Ok(v)) => Poll::Ready(Some(v)),
            Poll::Ready(Err(_)) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl <F> Timeout<F> 
where
    F: Future
{
    pub fn new(future: F, wait_for: std::time::Duration) -> Self {
        Self(tokio::time::timeout(wait_for, future))
    }
}

pub fn timeout<F: Future>(future: F, wait_for: std::time::Duration) -> Timeout<F> {
    Timeout::new(future, wait_for)
}