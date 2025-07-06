use rusqlite::Connection;
//use std::fs;


// Enable foreign key constraints
// conn.execute("PRAGMA foreign_keys = ON;", [])?;

pub fn init_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ngram (
	        ngram_id INTEGER NOT NULL UNIQUE,
	        n INTEGER NOT NULL,
	        PRIMARY KEY(ngram_id AUTOINCREMENT)
        )",
        (),
    ).expect("Failed to create ngram table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS word (
            word_id	INTEGER NOT NULL UNIQUE,
            the_word TEXT NOT NULL,
            PRIMARY KEY(word_id AUTOINCREMENT)
        )",
        (),
    ).expect("Failed to create word table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ngram_word (
            ngram_id INTEGER NOT NULL,
            seq	INTEGER NOT NULL,
            word_id	INTEGER,
            PRIMARY KEY(ngram_id,seq),
            FOREIGN KEY(ngram_id) REFERENCES ngram(ngram_id),
            FOREIGN KEY(word_id) REFERENCES word(word_id)
        )",
        (),
    ).expect("Failed to create ngram_word table");

}