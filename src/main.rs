use std::sync::{Arc, Mutex};
use std::thread;

mod fh_tools;
use fh_tools::database::FhDb;
use fh_tools::function::monitor_parser;
use fh_tools::ingestor::Ingestor;
use fh_tools::parser::Parser;
use std::env;

fn main() {
    let mut db = FhDb::new(
        "localhost".to_string(),
        env::var("USER").expect("User not found"),
        env::var("DB_PASS").expect("Password not found"),
    );
    db.connect();

    let arc_db = Arc::new(Mutex::new(db));
    let p = Parser::new(500);
    let arc_p = Arc::new(Mutex::new(p));

    let arc_db_c = arc_db.clone();
    let arc_p_c = arc_p.clone();
    thread::spawn(move || {
        monitor_parser(arc_db_c, arc_p_c);
    });
    let arc_p_c = arc_p.clone();
    let mut i = Ingestor {
        addr: "localhost".to_string(),
        port: 3333,
        point_storage: arc_p_c,
    };
    let _ = i.listen();
}
