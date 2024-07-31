fn main() {
    tonic_build::compile_protos("proto/CommonMessages_External.proto").unwrap();
    tonic_build::compile_protos("proto/CommonModelMessages_External.proto").unwrap();
    tonic_build::compile_protos("proto/CSMessages_External.proto").unwrap();
    tonic_build::compile_protos("proto/CSModelMessages_External.proto").unwrap();
}
