pub fn select_route(model: &str) -> String {
    if model.contains("llama") {
        "edge-gpu".to_string()
    } else {
        "cpu-fallback".to_string()
    }
}
