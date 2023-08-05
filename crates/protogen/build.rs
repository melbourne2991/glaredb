fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "proto/metastore/arrow.proto",
                "proto/metastore/catalog.proto",
                "proto/metastore/service.proto",
                "proto/metastore/storage.proto",
                "proto/metastore/options.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
