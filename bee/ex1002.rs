use std::io;

fn converter_para_float_32(entrada: & String) ->f64{
    let x = entrada.trim().parse::<f64>().unwrap(); //unwrap desenvelopa(pega valor dentro do container)
    x           //o retorno da funcao
}

fn main(){
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("erro");

    let area = 3.14159*(converter_para_float_32(&a).powf(2.0));

   // let area_arredondada = (area * 10_000.0).round() / 10_000.0;

    let num_formatado = format!("{:.4}", area);  //aqui iria a area arredondada

    println!("A={}",num_formatado);


}