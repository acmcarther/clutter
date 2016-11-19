extern crate clap;
extern crate runner;
extern crate iron;
extern crate params;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate rand;
extern crate plugin;
extern crate files;

use clap::ArgMatches;
use runner::TickableService;

// File upload

// File retrieval

// Authentication

// Url shortening

pub struct WebService;

impl TickableService for WebService {
  fn build<'a>(_: &ArgMatches<'a>) -> WebService {
    WebService
  }
}
