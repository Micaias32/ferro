fn main() {
    let source = "let a = 7;";
    let tokens = tokenize(source);
    println!("{:#?}", tokens);
}
