#![feature(proc_macro_hygiene, decl_macro)]

extern crate uuid;

#[macro_use]
extern crate rocket;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod figure;
mod store;

use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::http::Status;

use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use self::figure::Figure;

fn render(id: Uuid) -> Option<NamedFile> {
    let figure = store::load_fig(&id)?;
    let path = figure.compile();
    Some(NamedFile::open(path).unwrap())
}

#[get("/<id>", format = "image/*")]
fn display(id: Uuid) -> Option<NamedFile> {
    render(id)
}

#[get("/<id>?img", format = "text/html")]
fn display_redirect(id: Uuid) -> Option<NamedFile> {
    render(id)
}

#[get("/<id>", format = "text/html", rank = 2)]
fn editor(id: Uuid) -> &'static str {
    "Test"
}

#[get("/", format = "text/html")]
fn create() -> Redirect {
    let uuid: uuid::Uuid = uuid::Uuid::new_v4();

    store::store_fig(&uuid, &Figure::default());

    let uuid: String = uuid.to_hyphenated().to_string();

    Redirect::to(format!("/{}", uuid))
}

#[post("/<id>", format = "application/json", data = "<figure>")]
fn update(id: Uuid, figure: Json<Figure>) -> Option<Status> {
    store::store_fig(&id, &figure)?;
    Some(Status::Ok)
}

fn main() {
    let base_uri = std::env::vars().find(|(k, _)| k == "APP_ROOT")
        .map_or("/".to_string(), |(_, v)| v);

    rocket::ignite().mount(base_uri.as_str(), routes![
        create,
        display,
        display_redirect,
        editor,
        update
    ]).launch();
}
