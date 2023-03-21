#[test]
fn simple() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let task = tokio::task::spawn(async {});
        task.await.unwrap();
    });
}
