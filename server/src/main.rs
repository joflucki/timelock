use shared::ClientRequest;

fn main() {
    println!("Server started");
    some_shared_function();

    let _: ClientRequest = ClientRequest::GetHello;
}
