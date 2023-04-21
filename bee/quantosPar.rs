use std::io;


fn converter_para_float_32(entrada: & String) ->f32{
    let x = entrada.trim().parse::<f32>().unwrap(); 
    x           
}

/* enum Numero{
    Integer(i32),
    Float(f32),
} */

fn main(){

    let mut vet: Vec<f64> = Vec::new();

    let mut par = 0;

    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("nao leu");

        let num = input.trim().parse::<f64>().expect("erro");
        vet.push(num);

        //essa parte de cima eh apenas para gravar no vetor, mas um jeito mais facil e ler e ja ir comparando

        if converter_para_float_32(&input)%2.0 == 0.0 {
            par +=1; 
        }

    }



    println!("{} valores pares",par);

}