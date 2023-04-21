struct Pessoa {
    nome: String,
    idade: i32,
}

impl Pessoa {
    fn novo(nome: String, idade: i32) -> Self {
        Self { nome, idade }
    }

    fn aniversario(&mut self) {
        self.idade += 1;
    }
}

fn main() {
    let mut pessoa = Pessoa::novo("João".to_string(), 20);
    pessoa.aniversario();
    println!("{} tem {} anos.", pessoa.nome, pessoa.idade); // Output: "João tem 26 anos."
}
