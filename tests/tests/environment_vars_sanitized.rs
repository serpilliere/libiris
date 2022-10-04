use common::{common_test_setup, get_worker_bin_path};
use iris_broker::{Policy, Worker};

#[test]
fn environment_vars_sanitized() {
    common_test_setup();
    let worker_binary = get_worker_bin_path();
    let mut worker = Worker::new(
        &Policy::nothing_allowed(),
        &worker_binary,
        &[&worker_binary],
        &[],
        None,
        None,
        None,
    )
    .expect("worker creation failed");
    assert_eq!(worker.wait_for_exit(), Ok(0), "worker wait_for_exit failed");
}
