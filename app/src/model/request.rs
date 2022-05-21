use std::{future::Future, mem::replace};

use anyhow::Result;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{channel, error::TryRecvError, Receiver},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct Request<T: Send + 'static, R: Send + 'static> {
    inner: Option<JoinHandle<Result<R>>>,
    channel: Receiver<T>,
    pub status: RequestStatus<T, R>,
}

impl<T: Send + 'static, R: Send + 'static> Request<T, R> {
    pub fn new(handle: JoinHandle<Result<R>>, channel: Receiver<T>) -> Request<T, R> {
        Self {
            inner: Some(handle),
            channel,
            status: RequestStatus::default(),
        }
    }

    pub fn simple<F>(runtime: &Runtime, future: impl FnOnce() -> F + Send + 'static) -> Self
    where
        F: Future<Output = Result<R>> + Send + 'static,
        F::Output: 'static,
    {
        let (tx, rx) = channel(1);
        Self::new(
            runtime.spawn(async move {
                let result = future().await;
                drop(tx);
                result
            }),
            rx,
        )
    }

    pub fn peek(&mut self, runtime: &Runtime) -> &mut Self {
        match self.channel.try_recv() {
            Ok(item) => self.status = RequestStatus::Last(Some(item)),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                self.status = RequestStatus::Finished({
                    let inner = self.inner.take().unwrap();
                    runtime.block_on(async move { inner.await }).unwrap()
                })
            }
        }

        self
    }
}

#[derive(Debug)]
pub enum RequestStatus<T, R> {
    Last(Option<T>),
    Finished(Result<R>),
}

impl<T, R> RequestStatus<T, R> {
    pub fn take(&mut self) -> Self {
        replace(self, Self::Last(None))
    }
}

impl<T, R> Default for RequestStatus<T, R> {
    fn default() -> Self {
        RequestStatus::Last(None)
    }
}
