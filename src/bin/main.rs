use libnasu::run;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("nasu-main-proc")
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        run().await.unwrap();
    });
}
