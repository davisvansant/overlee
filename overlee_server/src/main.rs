use std::collections::HashMap;
use tonic::metadata::*;
use tonic::Code;
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
        println!("Incoming All Albums request ...");
        println!(" ~ IP address : {:?}", request.remote_addr().unwrap().ip());
        println!(" ~ Port : {:?}", request.remote_addr().unwrap().port());
        println!(" ~ Request Info : {:?}", request.get_ref());
        for v in request.metadata().values() {
            match v {
                ValueRef::Ascii(ref v) => println!(" ~ Request metadata : {:?} ", v),
                ValueRef::Binary(ref v) => println!(" ~ Request metadata : {:?}", v),
            }
        }

        let mut reply = the_rentals::AllAlbums { albums: Vec::new() };
        let mut all_albums = the_rentals::all_albums::Album {
            album: HashMap::new(),
        };
        all_albums
            .album
            .insert(1, String::from("Return of the Rentals"));
        all_albums
            .album
            .insert(2, String::from("Seven more Minutes"));
        all_albums
            .album
            .insert(3, String::from("Lost in Alphaville"));
        all_albums.album.insert(4, String::from("Q36"));
        reply.albums.push(all_albums);
        Ok(Response::new(reply))
    }

    async fn album(&self, request: Request<GetAlbum>) -> Result<Response<Release>, Status> {
        println!("Incoming Album Release request ...");
        println!(" ~ IP address : {:?}", request.remote_addr().unwrap().ip());
        println!(" ~ Port : {:?}", request.remote_addr().unwrap().port());
        println!(" ~ Requesting Album id : {:?}", request.get_ref().album_id);
        for v in request.metadata().values() {
            match v {
                ValueRef::Ascii(ref v) => println!(" ~ Request metadata : {:?}", v),
                ValueRef::Binary(ref v) => println!(" ~ Request metadata : {:?}", v),
            }
        }
        let release = match request.into_inner().album_id {
            1 => {
                let mut album = the_rentals::Release {
                    id: String::from("1"),
                    name: String::from("Return of the Rentals"),
                    release_type: 1,
                    release_date: String::from("1995.10.24"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(1, String::from("The Love I'm Searching For"));
                track_listing.tracks.insert(2, String::from("Waiting"));
                track_listing
                    .tracks
                    .insert(3, String::from("Friends of P."));
                track_listing.tracks.insert(4, String::from("Move On"));
                track_listing
                    .tracks
                    .insert(5, String::from("Please Let That Be You"));
                track_listing
                    .tracks
                    .insert(6, String::from("My Summer Girl"));
                track_listing
                    .tracks
                    .insert(7, String::from("Brilliant Boy"));
                track_listing.tracks.insert(8, String::from("Naive"));
                track_listing.tracks.insert(9, String::from("These Days"));
                track_listing
                    .tracks
                    .insert(10, String::from("Sweetness and Tenderness"));
                album.track_listing.push(track_listing);
                Ok(album)
            }
            2 => {
                let mut album = the_rentals::Release {
                    id: String::from("2"),
                    name: String::from("Seven More Minutes"),
                    release_type: 1,
                    release_date: String::from("1999.04.13"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing.tracks.insert(1, String::from("Getting By"));
                track_listing.tracks.insert(2, String::from("Hello, Hello"));
                track_listing
                    .tracks
                    .insert(3, String::from("She Says Its Alright"));
                track_listing.tracks.insert(4, String::from("The Cruise"));
                track_listing.tracks.insert(5, String::from("Barcelona"));
                track_listing
                    .tracks
                    .insert(6, String::from("Say Goodbye Forever"));
                track_listing.tracks.insert(7, String::from("Overlee"));
                track_listing.tracks.insert(8, String::from("Big Daddy C."));
                track_listing
                    .tracks
                    .insert(9, String::from("Keep Sleeping"));
                track_listing
                    .tracks
                    .insert(10, String::from("The Man with Two Brains"));
                track_listing
                    .tracks
                    .insert(11, String::from("Must Be Wrong"));
                track_listing.tracks.insert(12, String::from("Insomnia"));
                track_listing
                    .tracks
                    .insert(13, String::from("Its Alright (Reprise)"));
                track_listing
                    .tracks
                    .insert(14, String::from("My Head Is in the Sun"));
                track_listing
                    .tracks
                    .insert(15, String::from("Jumping Around"));
                album.track_listing.push(track_listing);
                Ok(album)
            }
            3 => {
                let mut album = the_rentals::Release {
                    id: String::from("3"),
                    name: String::from("Lost in Alphaville"),
                    release_type: 1,
                    release_date: String::from("2014.08.22"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(1, String::from("It's Time to Come Home"));
                track_listing
                    .tracks
                    .insert(2, String::from("Traces of Our Tears"));
                track_listing.tracks.insert(3, String::from("Stardust"));
                track_listing.tracks.insert(4, String::from("1000 Seasons"));
                track_listing.tracks.insert(5, String::from("Damaris"));
                track_listing
                    .tracks
                    .insert(6, String::from("Irrational Things"));
                track_listing
                    .tracks
                    .insert(7, String::from("Thought of Sound"));
                track_listing
                    .tracks
                    .insert(8, String::from("Song of Remembering"));
                track_listing.tracks.insert(9, String::from("Seven Years"));
                track_listing.tracks.insert(10, String::from("The Future"));
                album.track_listing.push(track_listing);
                Ok(album)
            }
            4 => {
                let mut album = the_rentals::Release {
                    id: String::from("4"),
                    name: String::from("Q36"),
                    release_type: 1,
                    release_date: String::from("2020.06.26"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(1, String::from("Shake Your Diamonds"));
                track_listing.tracks.insert(2, String::from("Nowhere Girl"));
                track_listing
                    .tracks
                    .insert(3, String::from("9th Configuration"));
                track_listing
                    .tracks
                    .insert(4, String::from("Tean Beat Cosmonaut"));
                track_listing
                    .tracks
                    .insert(5, String::from("Above This Broken World"));
                track_listing
                    .tracks
                    .insert(6, String::from("Forgotten Astronaut"));
                track_listing.tracks.insert(7, String::from("Conspiracy"));
                track_listing
                    .tracks
                    .insert(8, String::from("Breaking and Breaking and Breaking"));
                track_listing
                    .tracks
                    .insert(9, String::from("Great Big Blue"));
                track_listing
                    .tracks
                    .insert(10, String::from("Information (And The Island In The Sky)"));
                track_listing.tracks.insert(11, String::from("Spaceships"));
                track_listing
                    .tracks
                    .insert(12, String::from("Goodbye, Steve"));
                track_listing
                    .tracks
                    .insert(13, String::from("Invasion Night"));
                track_listing
                    .tracks
                    .insert(14, String::from("Another World"));
                track_listing
                    .tracks
                    .insert(15, String::from("Machine Love"));
                track_listing
                    .tracks
                    .insert(16, String::from("Elon Musk is Making Me Sad"));
                album.track_listing.push(track_listing);
                Ok(album)
            }
            _ => {
                let message =
                    String::from("The requested Album was not found... please try again!");
                let status = Status::new(Code::NotFound, message);
                Err(status)
            }
        };
        match release {
            Ok(response_ok) => Ok(Response::new(response_ok)),
            Err(response_error) => Err(response_error),
        }
    }

    async fn eps(&self, request: Request<GetEps>) -> Result<Response<AllEps>, Status> {
        // println!("All Eps! {:?}", request);
        println!("Incoming All Eps request ...");
        println!(" ~ IP address : {:?}", request.remote_addr().unwrap().ip());
        println!(" ~ Port : {:?}", request.remote_addr().unwrap().port());
        println!(" ~ Request Info : {:?}", request.get_ref());

        let mut reply = the_rentals::AllEps { eps: Vec::new() };
        let mut all_eps = the_rentals::all_eps::Ep { ep: HashMap::new() };
        all_eps
            .ep
            .insert(1, String::from("The Last Little Life EP"));
        all_eps.ep.insert(
            2,
            String::from("Songs About Time: Chapter One: The Story of a Thousand Seasons Past"),
        );
        all_eps.ep.insert(
            3,
            String::from("Songs About Time: Chapter Two: It's Time to Come Home"),
        );
        all_eps.ep.insert(
            4,
            String::from("Songs About Time: Chapter Three: The Future"),
        );
        reply.eps.push(all_eps);
        Ok(Response::new(reply))
    }

    async fn ep(&self, request: Request<GetEp>) -> Result<Response<Release>, Status> {
        println!("Incoming EP Release request ...");
        println!(" ~ IP address : {:?}", request.remote_addr().unwrap().ip());
        println!(" ~ Port : {:?}", request.remote_addr().unwrap().port());
        println!(" ~ Request Info : {:?}", request.get_ref());
        for v in request.metadata().values() {
            match v {
                ValueRef::Ascii(ref v) => println!(" ~ Request metadata : {:?} ", v),
                ValueRef::Binary(ref v) => println!(" ~ Request metadata : {:?}", v),
            }
        }
        let release = match request.into_inner().ep_id {
            1 => {
                let mut ep = the_rentals::Release {
                    id: String::from("1"),
                    name: String::from("The Last Little Life EP"),
                    release_type: 2,
                    release_date: String::from("2007.08.14"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(1, String::from("Last Romantic Day"));
                track_listing
                    .tracks
                    .insert(2, String::from("Little Bit of You in Everything"));
                track_listing
                    .tracks
                    .insert(3, String::from("Life Without a Brain"));
                track_listing
                    .tracks
                    .insert(4, String::from("Sweetness and Tenderness (New Version)"));
                ep.track_listing.push(track_listing);
                Ok(ep)
            }
            2 => {
                let mut ep = the_rentals::Release {
                    id: String::from("2"),
                    name: String::from(
                        "Songs About Time - Chapter One: The Story of a Thousand Seasons Past",
                    ),
                    release_type: 2,
                    release_date: String::from("2009.04.07"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(1, String::from("Song of Remembering"));
                track_listing
                    .tracks
                    .insert(2, String::from("Story of a Thousands Seasons Past"));
                track_listing.tracks.insert(3, String::from("All I Have"));
                track_listing.tracks.insert(4, String::from("Seven Years"));
                track_listing
                    .tracks
                    .insert(5, String::from("Thought of Sound"));
                track_listing
                    .tracks
                    .insert(6, String::from("Fall Into the Eve"));
                track_listing.tracks.insert(7, String::from("Colorado"));
                track_listing
                    .tracks
                    .insert(8, String::from("A Thousand Season Past (Spanish Version)"));
                ep.track_listing.push(track_listing);
                Ok(ep)
            }
            3 => {
                let mut ep = the_rentals::Release {
                    id: String::from("3"),
                    name: String::from("Songs About Time - Chapter Two: Its Time to Come Home"),
                    release_type: 2,
                    release_date: String::from("2009.06.07"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(9, String::from("Its Time to Come Home"));
                track_listing
                    .tracks
                    .insert(10, String::from("No Desire #2"));
                track_listing
                    .tracks
                    .insert(11, String::from("Girls on the Metro"));
                track_listing
                    .tracks
                    .insert(12, String::from("Late Night Confessions"));
                track_listing
                    .tracks
                    .insert(13, String::from("One Last Prayer"));
                track_listing
                    .tracks
                    .insert(14, String::from("A Otra Cosa Mariposa"));
                track_listing.tracks.insert(15, String::from("Damaris"));
                track_listing
                    .tracks
                    .insert(16, String::from("Late Night Confessions (French Version)"));
                ep.track_listing.push(track_listing);
                Ok(ep)
            }
            4 => {
                let mut ep = the_rentals::Release {
                    id: String::from("4"),
                    name: String::from("Songs About Time - Chapter Three: The Future"),
                    release_type: 2,
                    release_date: String::from("2009.10.20"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(17, String::from("A Rose is a Rose"));
                track_listing
                    .tracks
                    .insert(18, String::from("Irrational Things"));
                track_listing
                    .tracks
                    .insert(19, String::from("Traces of Our Tears"));
                track_listing.tracks.insert(20, String::from("The Future"));
                track_listing.tracks.insert(21, String::from("Stardust"));
                track_listing
                    .tracks
                    .insert(22, String::from("Borrow Each Other"));
                track_listing.tracks.insert(23, String::from("Honey Life"));
                track_listing
                    .tracks
                    .insert(24, String::from("A Rose is a Rose (Japanese Version)"));
                ep.track_listing.push(track_listing);
                Ok(ep)
            }
            5 => {
                let mut ep = the_rentals::Release {
                    id: String::from("5"),
                    name: String::from("Songs About Time - Chapter Four: Tokyo Blues"),
                    release_type: 2,
                    release_date: String::from("2011"),
                    track_listing: Vec::new(),
                };
                let mut track_listing = the_rentals::release::TrackListing {
                    tracks: HashMap::new(),
                };
                track_listing
                    .tracks
                    .insert(25, String::from("October Thirteen"));
                track_listing
                    .tracks
                    .insert(26, String::from("Feburary Twenty Four"));
                track_listing
                    .tracks
                    .insert(27, String::from("March Ten (Part Two)"));
                track_listing
                    .tracks
                    .insert(28, String::from("November Twenty Four"));
                track_listing
                    .tracks
                    .insert(29, String::from("September Eight"));
                track_listing
                    .tracks
                    .insert(30, String::from("December One"));
                track_listing
                    .tracks
                    .insert(31, String::from("November Ten"));
                track_listing.tracks.insert(32, String::from("June Two"));
                track_listing
                    .tracks
                    .insert(33, String::from("Feburary Ten"));
                track_listing
                    .tracks
                    .insert(34, String::from("January Twenty"));
                track_listing
                    .tracks
                    .insert(35, String::from("March Ten (Part One)"));
                track_listing.tracks.insert(36, String::from("March Three"));
                track_listing
                    .tracks
                    .insert(37, String::from("December Thirty One"));
                track_listing
                    .tracks
                    .insert(38, String::from("July Twenty Eight"));
                track_listing
                    .tracks
                    .insert(39, String::from("July Twenty One"));
                track_listing.tracks.insert(40, String::from("May Twelve"));
                track_listing
                    .tracks
                    .insert(41, String::from("Feburary Three"));
                track_listing
                    .tracks
                    .insert(42, String::from("October Twenty Seven"));
                ep.track_listing.push(track_listing);
                Ok(ep)
            }
            _ => {
                let message = String::from("The requested EP was not found... please try again!");
                let status = Status::new(Code::NotFound, message);
                Err(status)
            }
        };

        match release {
            Ok(response_ok) => Ok(Response::new(response_ok)),
            Err(response_error) => Err(response_error),
        }
    }

    async fn singles(&self, request: Request<GetSingles>) -> Result<Response<AllSingles>, Status> {
        println!("Incoming AllSingles request ...");
        println!(" ~ IP address : {:?}", request.remote_addr().unwrap().ip());
        println!(" ~ Port : {:?}", request.remote_addr().unwrap().port());
        println!(" ~ Request Info : {:?}", request.get_ref());

        let mut reply = the_rentals::AllSingles {
            singles: Vec::new(),
        };
        let mut all_singles = the_rentals::all_singles::Single {
            single: HashMap::new(),
        };

        all_singles
            .single
            .insert(String::from("1995.01"), String::from("Friends of P."));
        all_singles
            .single
            .insert(String::from("1996.01"), String::from("Waiting"));
        all_singles
            .single
            .insert(String::from("1999.01"), String::from("Getting By"));
        all_singles
            .single
            .insert(String::from("2008.01"), String::from("Colorado"));
        all_singles
            .single
            .insert(String::from("2014.01"), String::from("Thought of Sound"));
        all_singles
            .single
            .insert(String::from("2014.02"), String::from("1000 Seasons"));
        all_singles.single.insert(
            String::from("2017.01"),
            String::from("Elon Musk is Making Me Sad"),
        );
        all_singles
            .single
            .insert(String::from("2019.01"), String::from("Spaceships"));
        all_singles
            .single
            .insert(String::from("2019.02"), String::from("Forgotten Astronaut"));
        all_singles
            .single
            .insert(String::from("2019.03"), String::from("9th Configuration"));
        all_singles.single.insert(
            String::from("2019.04"),
            String::from("Breaking and Breaking and Breaking"),
        );
        all_singles
            .single
            .insert(String::from("2019.05"), String::from("Invasion Night"));
        all_singles
            .single
            .insert(String::from("2020.01"), String::from("Nowhere Girl"));
        all_singles
            .single
            .insert(String::from("2020.02"), String::from("Great Big Blue"));
        all_singles.single.insert(
            String::from("2020.03"),
            String::from("Above This Broken World"),
        );
        all_singles
            .single
            .insert(String::from("2020.04"), String::from("Teen Beat Cosmonaut"));
        all_singles
            .single
            .insert(String::from("2020.05"), String::from("Another World"));
        all_singles
            .single
            .insert(String::from("2020.06"), String::from("Conspiracy"));
        all_singles.single.insert(
            String::from("2020.07"),
            String::from("Elon Musk Is Making Me Sad (Q36 version)"),
        );
        all_singles.single.insert(
            String::from("2020.08"),
            String::from("Information (and the Island in the Sky)"),
        );
        all_singles
            .single
            .insert(String::from("2020.09"), String::from("Machine Love"));
        all_singles
            .single
            .insert(String::from("20200.10"), String::from("Goodbye, Steve"));
        all_singles.single.insert(
            String::from("20200.11"),
            String::from("Shake Your Diamonds"),
        );
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
