#[test]
fn simple() {
    let mut workspace = dearx_workspace::Workspace::<i32>::new();
    let id0 = workspace.observe(|_| {});
    let id1 = workspace.observe(|_| {});

    workspace.dispose_opservation(id0);
    workspace.dispose_opservation(id1);
}
