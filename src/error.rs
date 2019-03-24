pub trait Report {
    fn position(&self) -> (u32, u32);
    fn message(&self) -> &String;
    fn place(&self) -> String;
    fn report_in(&self, source: &Vec<char>) {
        eprintln!("[line {}] Error{}:\n{}", self.position().0, self.place(), *self.message())
    }
}