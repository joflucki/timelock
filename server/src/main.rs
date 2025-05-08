use shared::ClientRequest;

fn main() {
    println!("Server started");

    let _: ClientRequest = ClientRequest::GetHello;
}
