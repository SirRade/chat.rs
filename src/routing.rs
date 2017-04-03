extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use self::iron::{Request, Response, status, IronResult};
use self::iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::Path;
use templating::{generate_site_without_data, Section};
use quiz::controller as quizctrl;

pub fn create_chain() -> Chain {

    let router =
        router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         get_quiz: get "/quiz" => quizctrl::get_quiz,
                         post_quiz: post "/quiz" => quizctrl::post_quiz,
                         get_quiz_play: get "/quiz/play" => quizctrl::get_play,
                         post_quiz_play: post "/quiz/play" => quizctrl::post_play,
                         get_quiz_admin: get "/quiz/admin" => quizctrl::get_admin,
                         post_quiz_admin_post: post "/quiz/admin" => quizctrl::post_admin,);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount
        .mount("/css", Static::new(Path::new("res/public/css")))
        .mount("/js", Static::new(Path::new("res/public/js")))
        .mount("/fonts", Static::new(Path::new("res/public/fonts")));

    Chain::new(mount)
}

fn respond_with_file(filename: &str, section: Option<&Section>) -> IronResult<Response> {
    let site_template = generate_site_without_data(filename, section);
    Ok(Response::with((site_template, status::Ok)))
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    respond_with_file("index", Some(&Section::Home))
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    respond_with_file("contact/contact", Some(&Section::Contact))
}
