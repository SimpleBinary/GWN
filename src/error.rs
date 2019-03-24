pub trait Report {
    fn position(&self) -> (u32, u32);
    fn message(&self) -> &String;
    fn place(&self) -> String;
    fn report_in(&self, source: &String) {
        let (line_number, col_number) = self.position();
        let mut line_contents = "";

        for (i, line) in source.lines().enumerate() {
            line_contents = line;
            if i == line_number as usize {
                break;
            }
        }

        let mut col_space = String::new();
        for i in (1..col_number) {
            col_space.push(' ');
        }

        eprintln!("[line {}] Error{}:\n    {}\n    {}^\n{}\n", line_number, self.place(), line_contents, col_space, *self.message())
    }
}