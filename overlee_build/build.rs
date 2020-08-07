fn main() {
    tonic_build::configure()
        .out_dir("./proto")
        .compile(&["./proto/the_rentals.proto"], &["./proto"])
        .expect("failed to compile protos");
}
