// use std::collections::HashMap;
use std::error::Error;
use tonic::{transport::Channel, Request, Response, Status};

pub use the_rentals::discography_client::*;
use the_rentals::{
    AllAlbums, AllEps, AllSingles, GetAlbum, GetAlbums, GetEp, GetEps, GetSingles, Release,
};

#[derive(Default)]
struct TheRentals;

pub mod the_rentals {
    include!("../../overlee_build/proto/the_rentals.rs");
}

pub async fn get_albums(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetAlbums {});
    let response = client.albums(request).await?;
    let message = response.into_inner();
    for release in message.albums {
        println!("All the albums!");
        for (k, v) in release.album.iter() {
            println!(" ~ title . {} ~ release . {}", v, k);
        }
    }
    Ok(())
}

pub async fn get_album(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

pub async fn get_eps(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEps {});
    let response = client.eps(request).await?;
    let message = response.into_inner();
    for release in message.eps {
        println!("All the eps!");
        for (k, v) in release.ep.iter() {
            println!(" ~ title . {} ~ release . {}", v, k);
        }
    }
    Ok(())
}

pub fn get_ep(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

pub async fn get_singles(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetSingles {});
    let response = client.singles(request).await?;
    let message = response.into_inner();
    for release in message.singles {
        println!("All the singles!");
        for (k, v) in release.single.iter() {
            println!(" ~ title . {} ~ release . {}", v, k);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
