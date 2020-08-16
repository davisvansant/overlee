#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = overlee_client::DiscographyClient::connect("http://[::]:8080").await?;

    println!(" ~ Hello...");
    println!(" ~ hello hello hello");
    overlee_client::get_albums(&mut client).await?;
    overlee_client::get_eps(&mut client).await?;
    overlee_client::get_singles(&mut client).await?;
    overlee_client::get_album(&mut client).await?;

    Ok(())
}
