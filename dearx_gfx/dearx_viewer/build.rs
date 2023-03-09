fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile(
            &["proto/dearx_viewer.proto"],
            &Vec::<std::path::PathBuf>::new(),
        )
        .unwrap();
}
