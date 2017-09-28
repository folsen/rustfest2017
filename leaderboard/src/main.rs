#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate parking_lot;
extern crate pretty_env_logger;
extern crate url;

use std::collections::{BTreeMap, HashMap};

use futures::future::FutureResult;
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use hyper::{Get, Post, StatusCode};
use parking_lot::RwLock;
use url::Url;

macro_rules! ok_or_400 {
	($code: expr) => {
		match $code {
			Ok(ok) => ok,
			Err(_) => return futures::future::ok(Response::new().with_status(StatusCode::BadRequest)),
		}
	}
}

macro_rules! some_or_400 {
	($code: expr) => {
		match $code {
			Some(some) => some,
			None => return futures::future::ok(Response::new().with_status(StatusCode::BadRequest)),
		}
	}
}

#[derive(Default)]
struct Leaderboard {
	scores: RwLock<HashMap<String, u64>>,
}

impl Service for Leaderboard {
	type Request = Request;
	type Response = Response;
	type Error = hyper::Error;
	type Future = FutureResult<Response, hyper::Error>;

	fn call(&self, req: Request) -> Self::Future {
		futures::future::ok(match (req.method(), req.path()) {
			(&Get, "/") => {
				let unlocked_scores = self.scores.read();
				let scores = unlocked_scores.iter()
					.map(|(user, score)| (*score, user))
					.collect::<BTreeMap<_, _>>()
					.into_iter()
					.map(|(score, user)| format!("{}: {}", user, score))
					.collect::<Vec<_>>()
					.join("\n");

				Response::new()
					.with_header(ContentLength(scores.len() as u64))
					.with_body(scores)
			},
			(&Post, "/submit") => {
				let url = ok_or_400!(Url::parse(req.uri().as_ref()));
				let queries: HashMap<_, _> = url.query_pairs().collect();
				let name = some_or_400!(queries.get("name"));
				let score = some_or_400!(queries.get("score"));
				let score_u64 = ok_or_400!(score.parse());
				self.scores.write().insert(name.to_string(), score_u64);
				Response::new().with_status(StatusCode::Ok)
			},
			_ => {
				Response::new().with_status(StatusCode::NotFound)
			}
		})
	}

}


fn main() {
	pretty_env_logger::init().unwrap();
	let addr = "127.0.0.1:1337".parse().unwrap();

	let server = Http::new().bind(&addr, || Ok(Leaderboard::default())).unwrap();
	println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
	server.run().unwrap();
}
