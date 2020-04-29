macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
);

fn main() {
    let names = map! {
            1 => "one",
            2 => "two"
        };
    println!("{} -> {:?}", 1, names.get(&1));
    println!("{} -> {:?}", 10, names.get(&10));
}
