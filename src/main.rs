use futures::prelude::*;

// https://github.com/async-rs/async-std-hyper
pub mod compat {
    use async_std::prelude::*;
    use async_std::task;

    #[derive(Clone)]
    pub struct HyperExecutor;

    impl<F> hyper::rt::Executor<F> for HyperExecutor
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        fn execute(&self, fut: F) {
            task::spawn(fut);
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let client: hyper::Client<hyperlocal::UnixConnector> = hyper::Client::builder()
        .executor(compat::HyperExecutor)
        .build(hyperlocal::UnixConnector);
    for i in 0.. {
        println!("{}", i);
        let res = async_std::task::block_on(async {
            let _resp = client
                .get(hyperlocal::Uri::new("/var/run/docker.sock", "//events?").into())
                .await;
            client
                .get(hyperlocal::Uri::new("/var/run/docker.sock", "/events?").into())
                .await
        })
        .unwrap();
        async_std::task::spawn(res.into_body().try_for_each(|_| futures::future::ok(())));
    }
}
