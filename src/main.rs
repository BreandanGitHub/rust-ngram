use rusqlite::{Connection, Result, named_params};

//https://stackoverflow.com/questions/10963316/correct-way-to-store-uni-bi-trigrams-ngrams-in-rdbms
//Requires constraint: ngram_word.seq >0 AND ngram_word.seq <= (select ngram.n FROM ngram ng WHERE ng.ngram_id = ngram_word.ngram_id)

mod db;

fn main() -> Result<()> {
    let conn = Connection::open("ngram.db")?;

    db::init_db(&conn);

    conn.execute(
        "INSERT INTO word(the_word) VALUES ('the'),('man'),('who'),('sold'),('the'),('world');",
        (),
    )?;
    
    conn.execute(
        "INSERT INTO ngram(n) VALUES (6);",
        (),
    )?;
    let ngram_id = conn.last_insert_rowid();
    //println!("{}", ngram_id);
    
    conn.execute(
        "INSERT INTO ngram_word (ngram_id, seq, word_id) VALUES
            (:ngram_id, 1, 1), 
            (:ngram_id, 2, 2), 
            (:ngram_id, 3, 3), 
            (:ngram_id, 4, 4), 
            (:ngram_id, 5, 5), 
            (:ngram_id, 6, 6);",
        named_params! {
            ":ngram_id": ngram_id
        },
    )?;
    
    let mut stmt = conn.prepare(
        "SELECT *
         FROM ngram_word nw
         JOIN word w ON w.word_id = nw.word_id
         WHERE ngram_id = 1
         ORDER BY seq;")?;

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? { 
        let ngram_id: u32 = row.get(0)?; 
        let seq: u32 = row.get(1)?; 
        let word_id: u32 = row.get(2)?; 
        let the_word: String = row.get(4)?; 
        println!("ngram_id: {} seq: {} word_id: {} the_word: {}", ngram_id, seq, word_id, the_word);
    }

    Ok(())
}