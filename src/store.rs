use crate::figure::Figure;
use std::fs::File;

use uuid::Uuid;

pub fn load_fig(id: &Uuid) -> Option<Figure> {
    let file = File::open(format!("/store/{}.json", id)).ok()?;
    serde_json::from_reader(file).ok()?
}

pub fn store_fig(id: &Uuid, fig: &Figure) -> Option<()> {
    let file = File::create(format!("/store/{}.json", id)).ok()?;
    serde_json::to_writer_pretty(file, fig).ok()?;
    Some(())
}
