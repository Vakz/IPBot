extern crate sqlite3;
extern crate time;

use std::vec::Vec;

use self::sqlite3::{
    DatabaseConnection,
    SqliteResult,
    StatementUpdate,
    ResultSet,
    ResultRow,
    ResultRowAccess,
    Query
};
use std::error::Error;
use self::time::Timespec;

pub struct Database {
    conn: DatabaseConnection
}

pub struct DBFile {
    pub name: String,
    pub dest: String,
    pub user: String,
    pub time: Timespec,
}

impl Default for DBFile {
    fn default() -> DBFile {
        DBFile {
            name: "".to_string(),
            dest: "".to_string(),
            user: "".to_string(),
            time: Timespec::new(0, 0)
        }
    }
}

impl Database {
    pub fn new() -> Result<Database, String> {
        match DatabaseConnection::new(self::sqlite3::access::ByFilename {
                filename: "db.sqlite", flags: Default::default()
            })
        {
            Ok(db) => Ok(Database { conn: db }),
            Err(err) => Err(err.description().to_string())
        }
    }

    pub fn insert(&mut self, file: DBFile) -> Result<(), &str> {
        let mut stmt = self.conn.prepare("INSERT INTO Files (name, dest, user) VALUES ($1, $2, $3)").unwrap();
        match stmt.update(&[&file.name, &file.dest, &file.user]) {
            Ok(i) if i == 0 => Err("Insert failed with unknown reason"),
            Ok(i) =>Ok(()),
            Err(err) => Err(err.desc),
        }
    }

    pub fn by_username(&mut self, username: String) -> Option<Vec<DBFile>> {
        unimplemented!();
    }

    pub fn get_exact(&mut self, file: String, user: String) -> Option<DBFile> {
        println!("Search for file: {}, by user: {}", file, user);
        let to_file = |row: &mut ResultRow| Ok(
            DBFile {
                name: row.get("name"),
                dest: row.get("dest"),
                user: row.get("user"),
                time: row.get("inserted")
            }
        );

        let mut q = match self.conn.prepare("SELECT * FROM Files WHERE user=$1 AND name=$2") {
            Ok(stmt) => stmt,
            Err(_) => return None
        };

        return q.query(&[&user, &file], to_file).ok()
        .and_then(|mut r| r.nth(0))
        .and_then(|r| r.ok());
    }
}
