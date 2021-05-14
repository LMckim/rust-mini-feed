use super::database::FhDb;
use super::parser::{Message, Parser};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn monitor_parser(db_arc: Arc<Mutex<FhDb>>, p_arc: Arc<Mutex<Parser>>) {
    loop {
        let mut p = p_arc.lock().unwrap();
        let mut db = db_arc.lock().unwrap();
        println!("Current: {}", p.get_p_count());
        let cnt = p.get_p_count();
        if cnt > 500 {
            let parsed: Vec<Message> = p.get_parsed_messages();
            drop(p);
            db.insert_vals("TESTING.TEST_FH".to_string(), parsed);
        } else {
            drop(p);
        }

        thread::sleep(Duration::new(1, 0));
    }
}
