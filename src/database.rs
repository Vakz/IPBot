extern crate sqlite3;
extern crate time;

use std::vec::Vec;

use self::sqlite3::{
    DatabaseConnection,
    SqliteResult,
    StatementUpdate,
    ResultRow,
    ResultRowAccess,
    Query,
    SqliteErrorCode
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
            Ok(_) => Ok(()),
            Err(err) => {
                match err.kind {
                    SqliteErrorCode::SQLITE_CONSTRAINT => Err("A file with that name already exists"),
                    _ => {
                        println!("INSERT ERROR: {}", err.to_string());
                        Err("Insert failed due to unknown error")
                    }
                }
            },
        }
    }

    pub fn by_username(&mut self, username: String) -> Option<Vec<DBFile>> {
        let mut q = match self.conn.prepare("SELECT * FROM Files WHERE user=$1") {
            Ok(stmt) => stmt,
            Err(_) => return None
        };

        let res = q.query(&[&username], Database::to_file).unwrap().map(|r| r.unwrap());

        return Some(res.collect::<Vec<_>>());
    }

    pub fn get_exact(&mut self, file: String) -> Option<DBFile> {
        println!("Search for file: {}", file);

        let mut q = match self.conn.prepare("SELECT * FROM Files WHERE name=$1") {
            Ok(stmt) => stmt,
            Err(_) => return None
        };

        return q.query(&[&file], Database::to_file).ok()
        .and_then(|mut r| r.nth(0))
        .and_then(|r| r.ok());
    }

    fn to_file(row: &mut ResultRow) -> SqliteResult<DBFile> {
        Ok(
            DBFile {
                name: row.get("name"),
                dest: row.get("dest"),
                user: row.get("user"),
                time: row.get("inserted")
            }
        )
    }
}
