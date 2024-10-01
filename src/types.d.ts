type PersonalMetrics = {
  journal_entry_id: number;
  fitness: number;
  mental: number;
  dietary: number;
  social: number;
  professional: number;
};

type JournalEntry = {
  id: number;
  date: string;
  text: string;
};

interface NewPersonalMetrics {
  journal_entry_id: number;
  fitness: number;
  mental: number;
  dietary: number;
  social: number;
  professional: number;
}

interface JournalEntry {
  created_at: string;
  text: string;
}
