use std::io;

// Função para calcular o fatorial de um número usando recursão
fn fatorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fatorial(n - 1)
    }
}

fn main() {
    // Solicita ao usuário um número inteiro
    println!("Digite um número inteiro: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada do usuário");

    // Converte a entrada do usuário em um número inteiro
    let n: u32 = input.trim().parse().expect("Entrada inválida");

    // Calcula e imprime o fatorial do número inserido pelo usuário
    let f = fatorial(n);
    println!("O fatorial de {} é {}.", n, f);
}
