fn main() {
    tonic_build::configure()
        .compile(&["proto/auth.v1.proto", "proto/shop.v1.proto"], &["proto"])
        .expect("compile gRPC proto files.");
}
