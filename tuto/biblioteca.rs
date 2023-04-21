// Importa a biblioteca rand
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let numero_aleatorio = rng.gen_range(1..=10);
    println!("O número aleatório é {}", numero_aleatorio);
}
