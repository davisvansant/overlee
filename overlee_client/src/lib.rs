use std::error::Error;
use tonic::transport::Channel;

pub use the_rentals::discography_client::*;
use the_rentals::{GetAlbum, GetAlbums, GetEp, GetEps, GetSingles};

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
                release.album.get(x).expect("Album not found")
            );
        }
    }
    Ok(())
}

pub async fn get_album(
    album_id: u32,
    client: &mut DiscographyClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetAlbum { album_id });
    let response = client.album(request).await?;
    let message = response.into_inner();
    println!("Album release lookup!");
    println!(" ~ Release ID . {}", message.id);
    println!(" ~ Release name . {}", message.name);
    println!(" ~ Release Type . {}", message.release_type);
    println!(" ~ Release Date . {}", message.release_date);
    for tracks in message.track_listing.iter() {
        let mut sorted_tracks = Vec::new();
        for track_id in tracks.tracks.keys() {
            sorted_tracks.push(track_id);
        }
        sorted_tracks.sort();
        println!("Track Listing");
        for track_id in sorted_tracks {
            println!(
                " ~ track . {} ~ title . {}",
                track_id,
                tracks
                    .tracks
                    .get(track_id)
                    .expect("Track listing not found")
            );
        }
    }
    Ok(())
}

pub async fn get_eps(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEps {});
    let response = client.eps(request).await?;
    let message = response.into_inner();
    for release in message.eps {
        let mut sorted_eps = Vec::new();
        println!("All the eps!");
        for release_id in release.ep.keys() {
            sorted_eps.push(release_id);
        }
        sorted_eps.sort();
        for release_id in sorted_eps {
            println!(
                " ~ release . {} ~ title . {}",
                release_id,
                release.ep.get(release_id).expect("Album not found")
            );
        }
    }
    Ok(())
}

pub async fn get_ep(
    ep_id: u32,
    client: &mut DiscographyClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetEp { ep_id });
    let response = client.ep(request).await?;
    let message = response.into_inner();
    println!("EP Release lookup!");
    println!(" ~ Release ID . {}", message.id);
    println!(" ~ Release name . {}", message.name);
    println!(" ~ Release type . {}", message.release_type);
    println!(" ~ Release date . {}", message.release_date);
    for tracks in message.track_listing.iter() {
        let mut sorted_tracks = Vec::new();
        for track_id in tracks.tracks.keys() {
            sorted_tracks.push(track_id);
        }
        sorted_tracks.sort();
        println!("Track Listing");
        for track_id in sorted_tracks {
            println!(
                " ~ track . {} ~ title . {}",
                track_id,
                tracks
                    .tracks
                    .get(track_id)
                    .expect("Track listing not found")
            );
        }
    }
    Ok(())
}

pub async fn get_singles(client: &mut DiscographyClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(GetSingles {});
    let response = client.singles(request).await?;
    let message = response.into_inner();
    for single in message.singles {
        println!("All the singles!");
        let mut sorted_singles = Vec::new();
        for release_date in single.single.keys() {
            sorted_singles.push(release_date);
        }
        sorted_singles.sort();
        for release_date in sorted_singles {
            println!(
                " ~ release . {} ~ title . {}",
                release_date,
                single.single.get(release_date).expect("Single not found")
            );
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
