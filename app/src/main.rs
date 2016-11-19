extern crate clap;
extern crate files;
extern crate web;
extern crate runner;

use clap::App;
use runner::Builder;
use runner::Handle;
use web::WebService;
use files::FileService;

fn main() {

  let app = WebService::args().into_iter()
    .chain(FileService::args())
    .fold(App::new("Clutter"), |app, arg| {
      app.arg(arg)
    });

  let matches = app.get_matches();

  let web_handle = WebService::start(&matches);
  let file_cleaner_handle = FileService::start(&matches);

  web_handle.block_until_finished();
  file_cleaner_handle.terminate();
}
