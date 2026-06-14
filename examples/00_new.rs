use ytmusic_rs::MusicClient;

fn main() {
    let client = MusicClient::new();
    println!("MusicClient created: {client:#?}");
}
