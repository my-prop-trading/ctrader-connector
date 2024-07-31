fn main() {
    tonic_build::compile_protos("src/manager/proto/CommonMessages_External.proto").unwrap();
    tonic_build::compile_protos("src/manager/proto/CommonModelMessages_External.proto").unwrap();
    tonic_build::compile_protos("src/manager/proto/CSMessages_External.proto").unwrap();
    tonic_build::compile_protos("src/manager/proto/CSModelMessages_External.proto").unwrap();
}
