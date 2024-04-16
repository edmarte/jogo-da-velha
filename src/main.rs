use std::io;

fn board(vector: [[char; 3]; 3]){
    println!(" {} | {} | {}", vector[0][0], vector[0][1], vector[0][2]);
    println!("----------");
    println!(" {} | {} | {}", vector[1][0], vector[1][1], vector[1][2]);
    println!("----------");
    println!(" {} | {} | {}", vector[2][0], vector[2][1], vector[2][2]);
}
fn welcome(){
    println!("Bem-vindo ao Jogo da Velha!");
    println!(" 1 | 2 | 3");
    println!("----------");
    println!(" 4 | 5 | 6");
    println!("----------");
    println!(" 7 | 8 | 9");
}
fn main() {
    let mut vector: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut input = String::new();
    let c = 'X';
    
    welcome();
 
    println!("Digite uma posição [1-9] para jogar:");
    io::stdin().read_line(&mut input).expect("error to read the input!");
    let play:u32 = input.trim().parse().expect("number invalid!");
    
    match play {
        1 => vector[0][0] = c,
        2 => vector[0][1] = c,
        3 => vector[0][2] = c,
        4 => vector[1][0] = c,
        5 => vector[1][1] = c,
        6 => vector[1][2] = c,
        7 => vector[2][0] = c,
        8 => vector[2][1] = c,
        9 => vector[2][2] = c,
        _ => println!("Posição inválida!")
    }

    board(vector);
}
