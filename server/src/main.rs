use shared::{some_shared_function, ClientRequest};

fn main() {
    println!("Server started");
    some_shared_function();

    let _: ClientRequest = ClientRequest::GetHello;
}
