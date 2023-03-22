use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    draw_welcome();
    let secret_number: i32 = randon_secret_number();
    let level: i32 = choose_level();
    let tries: i32 = number_tries(level);
    if tries != 0 {
        let correct: bool = play(tries, secret_number);
        draw_finish_the_game(correct, tries);
    }

    print_slower("Fim de Jogo!!!");

}

fn randon_secret_number() -> i32 {
    let mut number: i32 = 0;
    unsafe {
        let seconds: i32 = time(0);
        srand(seconds);
        number = rand();
    }
    
    number % 100
}

fn choose_level() -> i32 {
    print_slower("Qual o nível de dificuldade?");
    print_slower("(1) Fácil (2) Médio (3) Difícil");
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let level: i32 = input_line.trim().parse().expect("Input não é inteiro");
    level
}

fn number_tries(level: i32) -> i32 {
    match level {
        1 => {
            print_slower("Você escolheu o nível Fácil\n");
            20
        },
        2 => {
            print_slower("Você escolheu o nível Médio\n");
            15
        },
        3 => {
            print_slower("Você escolheu o nível difícil\n");
            6
        }
        _ => {
            print_slower("Alternativa não existe!\n");
            0
        }
    }
}

fn play(tries: i32, secret_number: i32) -> bool {
    let mut correct: bool = false;
    for i in 0..tries {
        println!("Tentativa {} de {}", i + 1, tries);
        print_slower("Qual é o seu chute? ");

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let guess: i32 = input_line.trim().parse().expect("Input não é inteiro");
        
        println!("Seu chute foi: {}", guess);

        if guess < 0 {
            print_slower("Você não pode chutar números negativos\n");
            continue;
        }

        correct = guess == secret_number;
        if correct {
            break;
        } else if guess > secret_number {
            print_slower("Seu chute foi maior que o número secreto\n");
        } else {
            print_slower("Seu chute foi menor que o número secreto\n");
        }
    }

    correct
}

fn draw_welcome() {
    println!("\n");
	println!("    /\\                                                        /\\     ");
	println!("   |  |                                                      |  |      ");
	println!("  /----\\                       Bem vindo                    /----\\   ");
	println!(" [______]                Jogo de adivinhação               [______]    ");
	println!("  |    |         _____                        _____         |    |     ");
	println!("  |[]  |        [     ]                      [     ]        |  []|     ");
	println!("  |    |       [_______][ ][ ][ ][][ ][ ][ ][_______]       |    |     ");
	println!("  |    [ ][ ][ ]|     |  ,----------------,  |     |[ ][ ][ ]    |     ");
	println!("  |             |     |/'    ____..____    '\\|     |             |    ");
	println!("   \\ []         |     |    /'    ||    '\\   |     |        []   /    ");
	println!("    |      []   |     |   |o     ||     o|   |     |  []       |       ");
	println!("    |           |  _  |   |     _||_     |   |  _  |           |       ");
	println!("    |   []      | (_) |   |    (_||_)    |   | (_) |       []  |       ");
	println!("    |           |     |   |     (||)     |   |     |           |       ");
	println!("    |           |     |   |      ||      |   |     |           |       ");
	println!("  /''           |     |   |o     ||     o|   |     |           ''\\    ");
	println!(" [_____________[_______]--'------''------'--[_______]_____________]    ");
	println!("\n");
}

fn draw_finish_the_game(correct: bool, tries: i32) {
    if correct {
        println!("");
		println!("             OOOOOOOOOOO               ");
        println!("         OOOOOOOOOOOOOOOOOOO           ");
        println!("      OOOOOO  OOOOOOOOO  OOOOOO        ");
        println!("    OOOOOO      OOOOO      OOOOOO      ");
        println!("  OOOOOOOO  #   OOOOO  #   OOOOOOOO    ");
        println!(" OOOOOOOOOO    OOOOOOO    OOOOOOOOOO   ");
        println!("OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO  ");
        println!("OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO  ");
        println!("OOOO  OOOOOOOOOOOOOOOOOOOOOOOOO  OOOO  ");
        println!(" OOOO  OOOOOOOOOOOOOOOOOOOOOOO  OOOO   ");
        println!("  OOOO   OOOOOOOOOOOOOOOOOOOO  OOOO    ");
        println!("    OOOOO   OOOOOOOOOOOOOOO   OOOO     ");
        println!("      OOOOOO   OOOOOOOOO   OOOOOO      ");
        println!("         OOOOOO         OOOOOO         ");
        println!("             OOOOOOOOOOOO              ");
        println!("");
        print_slower("Parabéns! Você acertou!");
        println!("\n");
        println!("Você acertou em {} tentativas. Até a próxima\n", tries);
    } else {
        println!("");
		println!("       \\|/ ____ \\|/    ");   
        println!("        @~/ ,. \\~@      ");   
        println!("       /_( \\__/ )_\\    ");   
        println!("          \\__U_/        ");
        println!("");
        print_slower("Você perdeu! Tente novamente!");
        println!("\n");
    }
}

fn print_slower(input: &str) {
    for item in input.chars() {
        print!("{}", item);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(20));
    }
    println!("");
}

extern "C" {
    fn rand() -> i32;
    fn srand(input: i32) -> i32;
    fn time(input: i32) -> i32;
}
