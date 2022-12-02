use ezio::stdio;

fn main() {
    for l in stdio::stdin() {
        dbg!(l);
    }
}
