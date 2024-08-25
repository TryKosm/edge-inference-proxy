use edge_inference_proxy::router::select_route;

#[test]
fn routes_llama_to_edge_gpu() {
    assert_eq!(select_route("llama3"), "edge-gpu".to_string());
}
