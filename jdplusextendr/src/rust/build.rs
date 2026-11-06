fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;

    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    tonic_prost_build::configure().compile_protos(
        &[
            "../../../../jdplus-ws/src/main/proto/jdplus/main/ws/v1/toolkit_basic.proto",
            "../../../../jdplus-ws/src/main/proto/jdplus/main/ws/v1/toolkit_messages.proto",
        ],
        &[
            "../../../../jdplus-ws/src/main/proto",
        ],
    )?;

    Ok(())
}