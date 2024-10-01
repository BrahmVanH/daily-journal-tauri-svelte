use std::time::SystemTime;

#[derive(Debug)]
pub struct PersonalMetrics {
  pub journal_entry_id: i64,
  pub financial: u8,
  pub fitness: u8,
  pub mental: u8,
  pub dietary: u8,
  pub social: u8,
  pub professional: u8
}


#[derive(Debug)]
pub struct JournalEntry {
  pub id: i64,
  pub date: String,
  pub text: String
}


#[derive(Debug)]
pub struct NewPersonalMetrics {
  pub journal_entry_id: i64,
  pub financial: u8,
  pub fitness: u8,
  pub mental: u8,
  pub dietary: u8,
  pub social: u8,
  pub professional: u8
}

#[derive(Debug)]
pub struct NewJournalEntry {
  pub created_at: String,
  pub text: String
}