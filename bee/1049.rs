use std::io;


fn main(){

    let mut st1 = String::new();
    io::stdin().read_line(&mut st1).expect("erro");
    
    let mut st2 = String::new();
    io::stdin().read_line(&mut st2).expect("erro");
    
    let mut st3 = String::new();
    io::stdin().read_line(&mut st3).expect("erro");

    //antes estava assim st1.as_str() //as_str para poder comparar string ali dentro
    match st1.trim(){     //mas nao precisa do as_str quando usa o trim, trim remove uns espacos e caracter de nova linhas antes de comparar
        "vertebrado" => match st2.trim() {
            "ave" => match st3.trim() {
                "carnivoro" => println!("aguia"),
                "onivoro" => println!("pomba"),
                _ => println!("invalido"),
            },
            "mamifero" => match st3.trim() {
                "onivoro" => println!("homem"),
                "herbivoro" => println!("vaca"),
                _ => println!("invalido") ,
            
            },
            _ => println!("invalido"),
        },
        "invertebrado" => match st2.trim() {
            "inseto" => match st3.trim() {
                "hematofago" => println!("pulga"),
                "herbivoro" => println!("lagarta"),
                _ => println!("invalido"),
            },
            "anelideo" => match st3.trim() {
                "hematofago" => println!("sanguessuga"),
                "onivoro" => println!("minhoca"),
                _ => println!("invalido"),
            
            },
            _ => println!("invalido"),
        },
        _ => println!("invalido"),
    }

}