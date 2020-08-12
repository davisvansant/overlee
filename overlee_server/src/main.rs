use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};

use the_rentals::discography_server::*;
use the_rentals::{
    AllAlbums, AllEps, AllSingles, GetAlbum, GetAlbums, GetEp, GetEps, GetSingles, Release,
};

#[derive(Default)]
struct TheRentals;

pub mod the_rentals {
    include!("../../overlee_build/proto/the_rentals.rs");
}

#[tonic::async_trait]
impl Discography for TheRentals {
    async fn albums(&self, request: Request<GetAlbums>) -> Result<Response<AllAlbums>, Status> {
        println!("All Albums! {:?}", request);

        let mut reply = the_rentals::AllAlbums { albums: Vec::new() };
        let mut all_albums = the_rentals::all_albums::Album {
            album: HashMap::new(),
        };
        all_albums.album.insert(
            String::from("10.24.1995"),
            String::from("Return of the Rentals"),
        );
        all_albums.album.insert(
            String::from("4.13.1999"),
            String::from("Seven more Minutes"),
        );
        all_albums.album.insert(
            String::from("8.22.2014"),
            String::from("Lost in Alphaville"),
        );
        all_albums
            .album
            .insert(String::from("6.26.2020"), String::from("Q36"));
        reply.albums.push(all_albums);
        Ok(Response::new(reply))
    }

    async fn album(&self, request: Request<GetAlbum>) -> Result<Response<Release>, Status> {
        println!("Release! {:?}", request);
        unimplemented!()
    }

    async fn eps(&self, request: Request<GetEps>) -> Result<Response<AllEps>, Status> {
        println!("All Eps! {:?}", request);

        let mut reply = the_rentals::AllEps { eps: Vec::new() };
        let mut all_eps = the_rentals::all_eps::Ep { ep: HashMap::new() };
        all_eps.ep.insert(
            String::from("08.14.2007"),
            String::from("The Last Little Life EP"),
        );
        all_eps.ep.insert(
            String::from("04.07.2009"),
            String::from("Songs About Time: Chapter One: The Story of a Thousand Seasons Past"),
        );
        all_eps.ep.insert(
            String::from("07.07.2009"),
            String::from("Songs About Time: Chapter Two: It's Time to Come Home"),
        );
        all_eps.ep.insert(
            String::from("10.20.2009"),
            String::from("Songs About Time: Chapter Three: The Future"),
        );
        reply.eps.push(all_eps);
        Ok(Response::new(reply))
    }

    async fn ep(&self, request: Request<GetEp>) -> Result<Response<Release>, Status> {
        println!("Release! {:?}", request);
        unimplemented!()
    }

    async fn singles(&self, request: Request<GetSingles>) -> Result<Response<AllSingles>, Status> {
        println!("Singles! {:?}", request);

        let mut reply = the_rentals::AllSingles {
            singles: Vec::new(),
        };
        let mut all_singles = the_rentals::all_singles::Single {
            single: HashMap::new(),
        };

        all_singles
            .single
            .insert(String::from("1995.1"), String::from("Friends of P."));
        all_singles
            .single
            .insert(String::from("1996.1"), String::from("Waiting"));
        all_singles
            .single
            .insert(String::from("1999.1"), String::from("Getting By"));
        all_singles
            .single
            .insert(String::from("2008.1"), String::from("Colorado"));
        all_singles
            .single
            .insert(String::from("2014.1"), String::from("Thought of Sound"));
        all_singles
            .single
            .insert(String::from("2014.2"), String::from("1000 Seasons"));
        all_singles.single.insert(
            String::from("2017.1"),
            String::from("Elon Musk is Making Me Sad"),
        );
        all_singles
            .single
            .insert(String::from("2019.1"), String::from("Spaceships"));
        all_singles
            .single
            .insert(String::from("2019.2"), String::from("Forgotten Astronaut"));
        all_singles
            .single
            .insert(String::from("2019.3"), String::from("9th Configuration"));
        all_singles.single.insert(
            String::from("2019.4"),
            String::from("Breaking and Breaking and Breaking"),
        );
        all_singles
            .single
            .insert(String::from("2019.5"), String::from("Invasion Night"));
        all_singles
            .single
            .insert(String::from("2020.1"), String::from("Nowhere Girl"));
        all_singles
            .single
            .insert(String::from("2020.2"), String::from("Great Big Blue"));
        all_singles.single.insert(
            String::from("2020.3"),
            String::from("Above This Broken World"),
        );
        all_singles
            .single
            .insert(String::from("2020.4"), String::from("Teen Beat Cosmonaut"));
        all_singles
            .single
            .insert(String::from("2020.5"), String::from("Another World"));
        all_singles
            .single
            .insert(String::from("2020.6"), String::from("Conspiracy"));
        all_singles.single.insert(
            String::from("2020.7"),
            String::from("Elon Musk Is Making Me Sad (Q36 version)"),
        );
        all_singles.single.insert(
            String::from("2020.8"),
            String::from("Information (and the Island in the Sky)"),
        );
        all_singles
            .single
            .insert(String::from("2020.9"), String::from("Machine Love"));
        all_singles
            .single
            .insert(String::from("2020.10"), String::from("Goodbye, Steve"));
        all_singles
            .single
            .insert(String::from("2020.11"), String::from("Shake Your Diamonds"));
        reply.singles.push(all_singles);
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:8080".parse().unwrap();
    let discography = TheRentals::default();

    println!("Overlee is listening on {}", addr);

    Server::builder()
        .add_service(DiscographyServer::new(discography))
        .serve(addr)
        .await?;

    Ok(())
}
