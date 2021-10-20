use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use grpc_web_client::Client;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn go_main() -> std::result::Result<(), JsValue> {
    let client = Client::new("http://127.0.0.1:50051".to_string());
    let mut client = GreeterClient::new(client);
    
    let request = tonic::Request::new(HelloRequest {
        name: "WebTonic".into(),
    });

    let response = client.say_hello(request).await.map_err(|_| JsValue::from_str("failed to say hello"))?;
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(&format!("RESPONSE={:?}", response)));

    body.append_child(&val)?;

    Ok(())
}