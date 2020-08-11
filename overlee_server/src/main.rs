use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};

use crate::the_rentals::all_albums::Album;
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
        all_albums
            .album
            .insert(String::from("1995"), String::from("Return of the Rentals"));
        all_albums
            .album
            .insert(String::from("1999"), String::from("Seven more Minutes"));
        all_albums
            .album
            .insert(String::from("2014"), String::from("Lost in Alphaville"));
        all_albums
            .album
            .insert(String::from("2020"), String::from("Q36"));
        reply.albums.push(all_albums);
        Ok(Response::new(reply))
    }

    async fn album(&self, request: Request<GetAlbum>) -> Result<Response<Release>, Status> {
        println!("Release! {:?}", request);
        unimplemented!()
    }

    async fn eps(&self, request: Request<GetEps>) -> Result<Response<AllEps>, Status> {
        println!("All Eps! {:?}", request);
        // unimplemented!()
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
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:8080".parse().unwrap();
    let discography = TheRentals::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(DiscographyServer::new(discography))
        .serve(addr)
        .await?;

    Ok(())
}
