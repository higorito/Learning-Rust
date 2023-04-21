use std::io;

fn converter_para_int(entrada: & String) ->i32{
    let x = entrada.trim().parse::<i32>().unwrap(); //unwrap desenvelopa(pega valor dentro do container)
    x           //o retorno da funcao
}

fn converter_para_float_32(entrada: & String) ->f32{
    let x = entrada.trim().parse::<f32>().unwrap(); //unwrap desenvelopa(pega valor dentro do container)
    x           //o retorno da funcao
}

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("erro");


    let mut b = String::new();

    io::stdin().read_line(&mut b).expect("erro");

    let soma = converter_para_float_32(&b) * converter_para_float_32(&a);    //qualquer operacao

    println!("SOMA = {}",soma);

}
