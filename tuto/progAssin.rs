use tokio::time::{sleep, Duration};

async fn tarefa() {
    sleep(Duration::from_secs(2)).await;
    println!("Tarefa completada!");
}

#[tokio::main]
async fn main() {
    println!("Iniciando tarefa...");
    tokio::spawn(tarefa());
    println!("Continuando com outras tarefas...");
    sleep(Duration::from_secs(1)).await;
    println!("Tarefas concluídas.");
}
