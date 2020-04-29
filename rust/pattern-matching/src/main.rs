macro_rules! foo {
    (x) => (println!("It is x!"));
    (y) => (println!("It is y!"));
}

macro_rules! bar {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

fn main() {
    foo!(x);
    bar!(y => 3);
}
