use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = overlee_client::DiscographyClient::connect("http://[::]:8080").await?;

    print!("{}[2J", 27 as char);
    println!(" ~ Hello...");
    println!(" ~ hello hello hello");

    loop {
        let mut lookup = String::new();

        println!(" ~ select album, eps, or singles to lookup");
        println!(" ~ 1 ~ albums");
        println!(" ~ 2 ~ eps");
        println!(" ~ 3 ~ singles");
        println!(" ~ 4 ~ exit");

        io::stdin().read_line(&mut lookup)?;
        let selected = lookup.trim().parse::<u32>()?;

        print!("{}[2J", 27 as char);

        match selected {
            1 => loop {
                println!(" ~ select album to lookup");

                overlee_client::get_albums(&mut client).await?;

                let mut get_album_id = String::new();

                io::stdin().read_line(&mut get_album_id)?;

                print!("{}[2J", 27 as char);

                let parsed_album = get_album_id.trim().parse::<u32>()?;
                let album = overlee_client::get_album(parsed_album, &mut client).await;

                if let Err(error) = album {
                    println!("{:?}", error);
                    continue;
                } else {
                    break;
                }
            },
            2 => loop {
                println!(" ~ select ep to lookup");

                overlee_client::get_eps(&mut client).await?;

                let mut get_ep_id = String::new();

                io::stdin().read_line(&mut get_ep_id)?;

                print!("{}[2J", 27 as char);

                let parsed_release = get_ep_id.trim().parse::<u32>()?;
                let ep = overlee_client::get_ep(parsed_release, &mut client).await;

                if let Err(error) = ep {
                    println!("{:?}", error);
                    continue;
                } else {
                    break;
                }
            },
            3 => {
                print!("{}[2J", 27 as char);
                overlee_client::get_singles(&mut client).await?;
            }
            4 => {
                print!("{}[2J", 27 as char);
                println!("~ Say goodbye (so say goodbye) ...");
                println!("~ So say goodbye forever (goodbye forever) ...");
                break Ok(());
            }
            _ => continue,
        };
    }
}
