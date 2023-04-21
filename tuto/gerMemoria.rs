fn main() {
    let s1 = String::from("Olá");
    let s2 = s1;
    // println!("{}", s1); // Isso resultaria em um erro de compilação, pois s1 não é mais válido

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3); // Output: "s2 = Olá, s3 = Olá"
}
