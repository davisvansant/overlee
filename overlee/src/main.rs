use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = overlee_client::DiscographyClient::connect("http://[::]:8080").await?;
    let mut get_album_id = String::new();
    let mut get_ep_id = String::new();

    println!(" ~ Hello...");
    println!(" ~ hello hello hello");
    overlee_client::get_albums(&mut client).await?;
    println!(" ~ select album to lookup");
    io::stdin().read_line(&mut get_album_id)?;
    let parsed_album = get_album_id.trim().parse::<u32>()?;
    let get_album = overlee_client::get_album(parsed_album, &mut client).await;

    match get_album {
        Ok(response) => println!("{:?}", response),
        Err(error) => println!("{:?}", error),
    }

    overlee_client::get_eps(&mut client).await?;
    println!(" ~ select ep to lookup");
    io::stdin().read_line(&mut get_ep_id)?;

    let parsed_ep = get_ep_id.trim().parse::<u32>()?;
    let get_ep = overlee_client::get_ep(parsed_ep, &mut client).await;

    match get_ep {
        Ok(response) => println!("{:?}", response),
        Err(error) => println!("{:?}", error),
    }
    overlee_client::get_singles(&mut client).await?;

    Ok(())
}
