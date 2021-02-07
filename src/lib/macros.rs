/// Helper `macro` to perform `tokio_test::block_on` calls
/// during library testing
#[macro_export]
macro_rules! bo {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
