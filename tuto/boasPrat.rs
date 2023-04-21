// Função que verifica se um número é par
fn eh_par(numero: i32) -> bool {
    numero % 2 == 0
}

// Função principal que usa a função eh_par
fn main() {
    let numero = 10;
    if eh_par(numero) {
        println!("{} é um número par.", numero);
    } else {
        println!("{} é um número ímpar.", numero);
    }
}
