use discord_presence::Client;

fn main() {
    let mut client = Client::new(1003450375732482138);

    client.start();

    client
        .set_activity(|a| {
            a.state("Rust")
                .details("Programming")
                .assets(|a| a.large_image("rust"))
        })
        .unwrap();

    println!("Made it to the final line");
}
