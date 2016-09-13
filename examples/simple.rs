extern crate termo;
use std::io;

fn main() {
    let stdout = io::stdout();
    let stdin = io::stdin();

    {
        let mut term = termo::Terminal::new(&stdout, &stdin);

        term.text().bold().pos(2, 2).text("blah");
    }
}
