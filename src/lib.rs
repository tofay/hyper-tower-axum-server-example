use std::task::Poll;

/// A hyper service converted into a tower service.
#[derive(Debug, Copy, Clone)]
pub struct HyperToTowerService<S> {
    service: S,
}

impl<S> HyperToTowerService<S> {
    /// Create a new `HyperToTowerService` from a hyper service.
    pub fn new(hyper_service: S) -> Self {
        Self {
            service: hyper_service,
        }
    }
}

impl<S, R> tower_service::Service<R> for HyperToTowerService<S>
where
    S: hyper::service::Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&mut self, req: R) -> Self::Future {
        self.service.call(req)
    }

    // Hyper services are always ready.
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
