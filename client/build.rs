use std::env;

fn main() {

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "wasm32" {
        panic!("Only Wasm cfg supported for this package")
    }

    tonic_build::configure()
         .build_server(false)
         .compile(
            &["../proto/helloworld.proto"],
            &["../proto/"],
        )
        .unwrap();

}