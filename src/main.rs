mod router;
mod cache;

fn main() {
    let route = router::select_route("llama3");
    println!("{}", route);
}
