use std::{io, thread};
use std::io::Write;
use std::time::Duration;
use rand::Rng;

fn loose_yourself_icon() {
    let skull = r#####"
          ______
       .-"      "-.
      /            \
     |              |
     |,  .-.  .-.  ,|
     | )(__/  \__)( |
     |/     /\     \|
     (_     ^^     _)
      \__|IIIIII|__/
       | \IIIIII/ |
       \          /
        `--------`
    "#####;

    // Utilisation du Rouge (31) pour le Game Over
    println!("\x1b[1;31m{}\x1b[0m", skull);
    println!("\x1b[1;31m   GAME OVER \x1b[0m");
}
fn draw_win() {
    let win_art = r#####"
      ___________
     '._==_==_=_.'
     .-\:      /-.
    | (|:.     |) |
     '-|:.     |-'
       \::.    /
        '::.  .'
          ) (
        _.' '._
       `-------`

  ___  ___  ________  ___  ___      ___       __   ___  ________
 /  / /  / /  __   / /  / /  /     /  /      /  / /  / /   __   \
/  /_/  / /  /_/  / /  /_/  /     /  /  __  /  / /  / /  /  /  /
\___   / /  __   / /  __   /     /  /  /  \/  / /  / /  /  /  /
   /  / /__/  /_/ /__/  /_/     /____/ \_____/ /__/ /__/  /__/
  /__/
    "#####;

    // Utilisation du Jaune/Or (33) et du Gras (1) pour l'effet de victoire
    println!("\x1b[1;33m{}\x1b[0m", win_art);
}

fn draw_hollow_italic_title() {
    // Séquence ANSI : 1=Gras, 3=Italique, 32=Vert (le vert ressort mieux pour le style "Game")
    let title = r#####"
  ________  ___  ___  _______   ________   ________
 /  _____/ /  / /  / /  ____/  /  _____/  /  _____/
/  /  __  /  / /  / /  /___   /  /____   /  /____
/  /_/ / /  /_/  / /  ____/  /____   /  /____   /
\_____/  \______/ /______/  /_______/  /_______/

  ________  ________  ___      ___  _______
 /  _____/ /  __   / /   \    /   / /  ____/
/  /  __  /  /_/  / /  /\ \  /  /  /  /___
/  /_/ / /  __   / /  /  \ \/  /  /  ____/
\_____/ /__/  /_/ /__/    \___/  /_______/
    "#####;

    println!("\x1b[1;3;36m{}\x1b[0m", title);
}

fn jester() {
    let art = r#####"
                _             _             _
               ( )           ( )           ( )
                X             X             X
               / \           / \           / \
        _______| |___________| |___________| |_______
       /      /   \         /   \         /   \      \
      /      /  _  \       /  _  \       /  _  \      \
     |      /  / \  \     /  / \  \     /  / \  \      |
      \____/  /   \  \___/  /   \  \___/  /   \  \____/
              v    v       v     v       v    v
                    _________________
                 _.'                 '._
               /     _____     _____     \
              /     /  _  \   /  _  \     \
             |     |  (O)  | |  (O)  |     |
             |      \_____/   \_____/      |
              \             _             /
               \      _    (_)    _      /
                '.   \ \         / /   .'
                  '._ \ \_______/ / _.'
                     \ \/\/\/\/\/\/ /
                      \____________/
                          |    |
              ____________|    |____________
             /                              \
            |    /\    /\    /\    /\    /\    |
             \__/  \__/  \__/  \__/  \__/  \__/
    "#####;

    // Utilisation du Rouge Vif (91) pour accentuer la malveillance
    println!("\x1b[1;91m{}\x1b[0m", art);
}

fn draw_grim_reaper_final() {
    let reaper = r#####"
             ___
          .-'   '-.
         /         \
         | _  _    |       _________________________________
         |(@)(@)   |      /                                /
         |  __     |     /          game is over...       /
         |_______  |    /________________________________/
       _.'  `---`  '._      //
     .'             '.     //
    /  /           \  \   //
   /  /             \  \ //
  |  /               \  /
  |  |   |       |   | /
  |  |   |       |   |/
  |  |   |       |   |
  |  |   |       |   |
  |  |   |       |   |
  |  |   |       |   |
  |  |   |       |   |
  /  |   |       |   \
 |___|___|_______|___|
    "#####;


    println!("\x1b[1;91m{}\x1b[0m", reaper);
}

fn draw_rising_soul() {
    let tomb_art = r#####"
             .  '  .
           '   (O)   '
          :  ~  |  ~  :
           '  . * .  '
              |   |
           ___|___|___
          /           \
         /    R.I.P.   \
        |     JOUEUR    |
        |               |
     ___|_______________|___
    /                       \
   /      _____________      \
  /      /             \      \
 /______/               \______\
    "#####;

    // Code ANSI : 1 (Gras), 36 (Cyan pour l'âme/spectre)
    println!("\x1b[1;36m{}\x1b[0m", tomb_art);
}



fn draw_animated_lost() {
    let art = r#####"
  ___ ___  ________  ___  ___      ___       ________  ________  _________
 /  //  / /  __   / /  / /  /     /  /      /  _____/ /  _____/ /____  ___/
/  /_/  / /  /_/  / /  /_/  /     /  /     /  /____  /  /____      /  /
\___   / /  __   / /  __   /     /  /     /____   / /____   /     /  /
   /  / /__/  /_/ /__/  /_/     /_____/ /_______/ /_______/     /__/
  /__/

      ___       __  ___  ________  ________  _______   ___       ________
     /  /      /  |/  / /  _____/ /  _____/ /  ____/  /  /      /  _____/
    /  /      /      / /  /____  /  /____  /  /___   /  /      /  /____
   /  /      /  /|_/  / /____   / /____   / /  ___/  /  /      /____   /
  /_____/  /__/  /__/ /_______/ /_______/ /_______/ /_____/  /_______/
    "#####;

    // 1=Gras, 31=Rouge dramatique
    print!("\x1b[1;31m");

    for c in art.chars() {
        print!("{}", c);
        // On force l'affichage immédiat sur le terminal
        io::stdout().flush().unwrap();

        // Délai de 2 millisecondes pour un effet fluide mais rapide
        thread::sleep(Duration::from_millis(200));
    }

    // Réinitialise le style à la fin
    println!("\x1b[0m");
}





fn random(range:i32) -> i32 {
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(0..range);

    secret
}

fn start(floor:i32, mut retry:i32){
    let guess_me = random(floor);
    let mut  found = false;
    println!("ready to loose yourself.....");
    loose_yourself_icon();
    println!("start matching ....");
    while !found && retry>0 {
        let mut tryer = String::new();
        retry-=1;
        println!("try for your soul .....");
        io::stdin().read_line(&mut tryer).expect("wrong value, missed life");
        let counter:i32 = match tryer.trim().parse() {
            Ok(num)=>num,
            Err(_) => {
                println!("loosed life !");
                return;
            }
        };

        if counter ==guess_me {
            draw_win();
            found = true;
        }
        if counter< guess_me {
            println!("the guess number is greater than your input");
        }
        if counter >guess_me {
            println!("the guess number in lower than your input");
        }
    }
    if found {
        println!("congratulation..... ");
        println!("you are safe");
    }
    else {
        thread::sleep(Duration::from_secs(1));
        jester();
        println!("give me yourself mother fucker ");
        thread::sleep(Duration::from_secs(2));
        draw_grim_reaper_final();
        thread::sleep(Duration::from_secs(2));
        draw_rising_soul();
        thread::sleep(Duration::from_secs(2));
        draw_animated_lost();


    }

}

fn menu(){
    draw_hollow_italic_title();
    let mut cont = true;
    println!("welcome to the guess game. ");
    while cont {
        let mut level = String::new();
        println!("choose yours game level");
        thread::sleep(Duration::from_secs(1));
        println!("1-easy");
        thread::sleep(Duration::from_secs(1));
        println!("2-medium");
        thread::sleep(Duration::from_secs(1));
        println!("3-hard");
        thread::sleep(Duration::from_secs(1));
        println!("4-extreme");
        thread::sleep(Duration::from_secs(1));
        println!("5-crazy");
        thread::sleep(Duration::from_secs(1));
        println!("6-what the fuck");
        thread::sleep(Duration::from_secs(1));
        println!("7-loose yourself");
        thread::sleep(Duration::from_secs(1));
        println!("8-exit");
        println!("enter your choice");
        io::stdin().read_line(&mut  level).expect("wrong value, try again and be assure that thee value is a valid number");
        let readable:i32  = match level.trim().parse() {
            Ok(number)=>number,
            Err(_)=>{
                println!("Veuillez entrer un nombre valide !");
                return;
            }
        };
        if readable==8 {
            break;
        }
        if readable>8 {
            println!("wrong choice....");
            continue;
        }
        party(readable);
    }


}

fn party(level:i32){
    match level{
        1 =>{
            println!("welcome to the easy party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 10 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(10,5);

        },
        2 =>{
            println!("welcome to the medium party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 15 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(15,5);

        },
        3 =>{
            println!("welcome to the hard party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 30 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(30,5);

        },
        4 =>{
            println!("welcome to the extreme party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 45 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(45,5);

        },
        5 =>{
            println!("welcome to the crazy party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 60 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(60,5);

        },
        6 =>{
            println!("welcome to the what the fuck party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 100 and you have 5 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(100,5);

        },

        7 =>{
            println!("welcome to the loose yourself party");
            thread::sleep(Duration::from_secs(2));
            println!("the number will be chosen between 0 and 1000 and you have 3 chances to find the right number");
            thread::sleep(Duration::from_secs(2));
            println!("dont forget that if you loose, fou will loose your soul too.......");
            start(1000,3);

        },
        _=> {
            println!("invalid choice .....\n ");
            thread::sleep(Duration::from_secs(3));
            println!("redirecting")
        }
    }
}
fn main() {
    println!("launching.....");
    thread::sleep(Duration::from_secs(5));
    menu();

}
