use std::mem::replace;

use anyhow::Result;
use tokio::{
    runtime::Runtime,
    sync::mpsc::{channel, error::TryRecvError, Receiver},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct Request<T, R> {
    inner: Option<JoinHandle<Result<R>>>,
    channel: Receiver<T>,
    pub status: RequestStatus<T, R>,
}

impl<T, R> Request<T, R> {
    pub fn new(handle: JoinHandle<Result<R>>, channel: Receiver<T>) -> Request<T, R> {
        Self {
            inner: Some(handle),
            channel,
            status: RequestStatus::default(),
        }
    }

    pub fn simple(handle: JoinHandle<Result<R>>) -> Self {
        Self::new(handle, channel(1).1)
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
