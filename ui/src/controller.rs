use crate::request::{Pong, RawRequest};

pub struct Controller {
    receiver: tokio::sync::mpsc::Sender<RawRequest>,
    handle: tokio::task::JoinHandle<()>,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<RawRequest>(128);

        let handle = tokio::spawn(async move {
            // TODO: fixme
            let mut system = optics::system::System::default();

            while let Some(request) = rx.recv().await {
                let _ = (request.func)(&mut system);
            }

            log::info!("Thread finished");
        });

        Self {
            receiver: tx,
            handle,
        }
    }

    // if T can fail T::Output should be a Result<Ok, Err>
    fn request<T: 'static + Send, F: 'static + Send + FnOnce(&mut optics::system::System) -> T>(
        &self,
        func: F,
    ) -> impl Future<Output = T> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        let raw_request = RawRequest {
            func: Box::new(move |sys| {
                if tx.send(func(sys)).is_err() {
                    log::error!("Sending result to closed channel. This should never happen");
                }
            }),
        };

        async {
            self.receiver
                .send(raw_request)
                .await
                .expect("Sending request to closed channel. This should never happen");

            rx.await.expect("Result not set")
        }
    }

    pub async fn join(self) {
        drop(self.receiver);
        self.handle.await.expect("Join handle panicked");
    }

    pub fn ping(&self) -> impl Future<Output = Pong> {
        self.request(|_| Pong)
    }

    pub fn test(&self) -> impl Future<Output = String> {
        self.request(|_| "test".to_string())
    }
}
