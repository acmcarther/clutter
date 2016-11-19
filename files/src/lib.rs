extern crate clap;
extern crate runner;

use clap::ArgMatches;
use runner::TickableService;

pub struct FileService;

impl TickableService for FileService {
  fn build<'a>(_: &ArgMatches<'a>) -> FileService {
    FileService
  }
}

