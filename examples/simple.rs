extern crate termo;

fn main() {
    let term = termo::Terminal::new();

    term.text().bold().pos(2, 2).text("blah");
}
