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
        let mut sorted = Vec::new();
        println!("All the albums!");
        for k in release.album.keys() {
            sorted.push(k);
        }
        sorted.sort();
        for x in sorted {
            println!(
                " ~ release . {} ~ title . {}",
                x,
                release.album.get(&x).expect("Album not found")
            );
        }
    }
    Ok(())
}

pub async fn get_album(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetAlbum { album_id: 1 });
    let response = client.album(request).await?;
    let message = response.into_inner();
    println!("Album id : {:?}", message.id);
    println!("Album name :{:?}", message.name);
    println!("Album release_type : {:?}", message.release_type);
    println!("Album release date{:?}", message.release_date);
    for tracks in message.track_listing.iter() {
        for (k, v) in tracks.tracks.iter() {
            println!(" ~ track . {} ~ title . {}", k, v);
        }
    }
    Ok(())
}

pub async fn get_eps(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEps {});
    let response = client.eps(request).await?;
    let message = response.into_inner();
    for release in message.eps {
        println!("All the eps!");
        for (k, v) in release.ep.iter() {
            println!(" ~ release . {} ~ title . {}", k, v);
        }
    }
    Ok(())
}

pub async fn get_ep(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEp { ep_id: 1 });
    let response = client.ep(request).await?;
    let message = response.into_inner();
    println!("EP id : {:?}", message.id);
    println!("EP name :{:?}", message.name);
    println!("EP release_type : {:?}", message.release_type);
    println!("EP release date{:?}", message.release_date);
    for tracks in message.track_listing.iter() {
        for (k, v) in tracks.tracks.iter() {
            println!(" ~ track . {} ~ title . {}", k, v);
        }
    }
    Ok(())
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
