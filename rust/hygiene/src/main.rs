macro_rules! foo {
    () => (let x = 3;)
}

// macro_rules! foo {
//     ($v:ident) => {
//         let $v = 3;
//     };
// }

fn main() {
    foo!();
    println!("{}", x);
}
