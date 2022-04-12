pub type LogEntry = String;

pub struct Log {
    entries: Vec<(LogEntry, usize)>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            entries: vec![]
        }
    }

    pub fn new_entry(&mut self, entry: LogEntry) {
        match self.entries.iter().position(|(e, _)| entry.eq(e)) {
            Some(pos) => {
                let (entry, mut count) = self.entries.remove(pos);
                count += 1;
                self.entries.push((entry, count));
            }
            None => self.entries.push((entry, 1)),
        }
    }
}
