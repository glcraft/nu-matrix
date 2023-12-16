#[test]
fn jrpc_stop() {
    let request = Request::new(Method::Stop, None);
    let request = serde_json::to_string(&request).expect("Failed to serialize into JSON");
    assert_eq!(request, r#"{"jsonrpc":"2.0","method":"Stop"}"#);
}
