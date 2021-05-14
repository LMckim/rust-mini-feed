use chrono::NaiveDateTime;
use std::mem::swap;

pub struct Message {
    pub seq: u64,
    pub time: u64,
    pub symbol: String,
    pub volume: u32,
    pub price: f32,
    pub venue: String,
}
impl Message {
    pub fn new() -> Message {
        return Message {
            seq: 0,
            time: 0,
            symbol: String::new(),
            volume: 0,
            price: 0.0,
            venue: String::new(),
        };
    }
    pub fn make_sql_val_string(&self) -> String {
        let dt = NaiveDateTime::from_timestamp(self.time as i64, 0);
        return format!(
            "({},'{}','{}','{}',{},{},'{}')",
            self.seq,
            dt.format("%Y-%m-%d"),
            dt.format("%H:%M:%S"),
            self.symbol,
            self.volume,
            self.price,
            self.venue
        );
    }
}
pub struct Parser {
    pub raw_parse_lim: u16,
    pub raw: Vec<[u8; 125]>,
    parsed: Vec<Message>,
}
impl Parser {
    pub fn new(raw_lim: u16) -> Parser {
        return Parser {
            raw_parse_lim: raw_lim,
            raw: Vec::new(),
            parsed: Vec::new(),
        };
    }
    pub fn add_message(&mut self, msg_buf: [u8; 125]) {
        self.raw.push(msg_buf);
        if self.raw.len() >= self.raw_parse_lim.into() {
            self.parse_raw_queue();
        }
    }
    pub fn get_p_count(&self) -> usize {
        return self.parsed.len();
    }
    pub fn get_parsed_messages(&mut self) -> Vec<Message> {
        let mut t = Vec::new();
        swap(&mut t, &mut self.parsed);
        return t;
    }
    fn parse_message(&msg_buf: &[u8; 125]) -> Message {
        let mut new_parsed = Message::new();

        let pieces = std::str::from_utf8(&msg_buf)
            .expect("could not convert")
            .split('\x1e');

        for i in pieces {
            // println!("{}", i);
            let mut section = i.split('=');
            let key = section.next().unwrap().trim_matches('\x02');
            let val = section.next().unwrap().trim_matches('\x03');

            match &key[..] {
                "42" => new_parsed.seq = val.parse().unwrap(),
                "32" => new_parsed.time = val.parse().unwrap(),
                "50" => new_parsed.symbol = val.to_string(),
                "51" => new_parsed.volume = val.parse().unwrap(),
                "52" => new_parsed.price = val.parse().unwrap(),
                "53" => new_parsed.venue = val[..3].to_string(),
                _ => println!("Unknown value {}", key),
            }
        }
        return new_parsed;
    }
    fn parse_raw_queue(&mut self) {
        for msg in self.raw.iter() {
            self.parsed.push(Parser::parse_message(msg));
        }
        self.raw.clear();
        // println!("{}", self.parsed.len());
    }
}
