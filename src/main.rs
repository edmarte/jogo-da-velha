use std::io;

fn board_array(array: [char; 9]){
    println!(" {} | {} | {}", array[0], array[1], array[2]);
    println!("-----------");
    println!(" {} | {} | {}", array[3], array[4], array[5]);
    println!("-----------");
    println!(" {} | {} | {}", array[6], array[7], array[8]);
    /*let mut count: i8 = 1;
    for index in array{
        if count == 3 || count == 6{
            println!(" {index}\n-----------");
        }else if count == 9{
            print!(" {}", index);
        }else{
            print!(" {} |", index);
        }
        count += 1;
    }
    println!();*/
}

fn welcome(){
    println!("Bem-vindo ao Jogo da Velha!");
    println!(" 1 | 2 | 3");
    println!("-----------");
    println!(" 4 | 5 | 6");
    println!("-----------");
    println!(" 7 | 8 | 9");
}

fn win_condition(array: [char; 9]) -> bool{
    if array[0] == array[1] && array[1] == array[2] && array[0] != ' '
    || array[3] == array[4] && array[4] == array[5] && array[3] != ' '
    || array[6] == array[7] && array[7] == array[8] && array[6] != ' '{
        return true;
    }else if array[0] == array[3] && array[3] == array[6] && array[0] != ' '
    || array[1] == array[4] && array[4] == array[7] && array[1] != ' '
    || array[2] == array[5] && array[5] == array[8] && array[2] != ' '{
        
        return true;
    }else if array[0] == array[4] && array[4] == array[8] && array[0] != ' '
    || array[2] == array[4] && array[4] == array[6] && array[2] != ' '{
       
        return true;
    }
    return false; 
}

fn main() {
    let mut array: [char; 9] = [' '; 9];
    let mut input = String::new();
    let mut c = 'X';

    welcome();
 
    loop {
        
        println!("Digite uma posição [1-9] para jogar:");
        io::stdin().read_line(&mut input).expect("Erro ao ler a entrada digitada!");
        let play:usize = input.trim().parse().expect("Número Invalido!");

        if array[play-1] == ' '{
            array[play-1] = c;

            let mut aux: i8 = 0;
            for index in array{
                if index == ' '{
                    aux+=1;
                }
            }
            if aux == 0{
                board_array(array);
                println!("Empatou!!!");
                return;
            }
            if win_condition(array){
                board_array(array);
                println!("Jogar '{}' Ganhou!!!", c);
                return;
            }
            
            if c == 'X'{
                c = 'O';
            }else if c == 'O'{
                c = 'X';
            }
        }else {
            println!("Lugar ocupado, por favor escolha outro!");
        }
        board_array(array);
        println!();

        input = String::new();
    }    
}