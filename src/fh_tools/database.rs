use super::parser::Message;
use mysql::prelude::Queryable;
use mysql::*;

pub struct FhDb {
    host: String,
    user: String,
    password: String,

    pool: Option<Pool>,
    conn: Option<PooledConn>,

    insert_limit: i32,
}
impl FhDb {
    pub fn new(host: String, user: String, password: String) -> FhDb {
        return FhDb {
            host: host,
            user: user,
            password: password,
            pool: None,
            conn: None,
            insert_limit: 10,
        };
    }
    pub fn connect(&mut self) {
        let url = format!("mysql://{}:{}@{}:3306", self.user, self.password, self.host);
        self.pool = Some(Pool::new(url).expect("Could not connect"));
        match &self.pool {
            Some(x) => self.conn = Some(x.get_conn().expect("Error during connection")),
            None => println!("Error during connect"),
        }
    }
    pub fn insert_vals(&mut self, table_name: String, msgs: Vec<Message>) {
        let head = format!(
            "INSERT INTO {}(SEQ,DATE,TIME,SYMBOL,VOLUME,PRICE,VENUE) VALUES",
            table_name
        );
        let mut query = head.clone();
        let mut inserts = 0;
        for msg in msgs.iter() {
            query += &msg.make_sql_val_string();
            query += ",";
            inserts += 1;
            if inserts >= self.insert_limit {
                self.execute_insertion(&mut query);
                inserts = 0;
                query = head.clone();
            }
        }
        if inserts > 0 {
            self.execute_insertion(&mut query);
        }
    }
    fn execute_insertion(&mut self, query: &mut String) -> bool {
        query.pop();
        match &mut self.conn {
            Some(x) => {
                x.query_drop(query.to_string()).expect(query);
                return true;
            }
            None => {
                println!("Issue during insert");
                return false;
            }
        }
    }
}
