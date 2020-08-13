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
    println!("Albums {:?}", response);

    Ok(())
}

pub async fn get_eps(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEps {});
    let response = client.eps(request).await?;
    println!("Albums {:?}", response);

    Ok(())
}

pub async fn get_singles(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetSingles {});
    let response = client.singles(request).await?;
    println!("Albums {:?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
