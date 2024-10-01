 CREATE TABLE IF NOT EXISTS personal_metrics (
     journal_entry_id INTEGER PRIMARY KEY NOT NULL,
     financial INTEGER NOT NULL,
     fitness INTEGER NOT NULL,
     mental INTEGER NOT NULL,
     dietary INTEGER NOT NULL,
     social INTEGER NOT NULL,
     professional INTEGER NOT NULL
     FOREIGN KEY(journal_entry_id) REFERENCES journal_entry(id)
 )