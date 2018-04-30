
extern crate 
text_souls;


    use std::env;
    use std::io;




use std::thread;
use std::time;
use text_souls::core::util as ds;


fn main() {
  let args: Vec<_> = env::args().collect();
  // dur: time to sleep for in between actions, 
default 1 second
  // --fast option == no time in between actions
  let mut dur = time::Duration::new(1, 0);
  if args.len() > 1 {
        if args[1] == "-f" || args[1] == "--fast" {
           dur = time::Duration::new(0, 0);
       } else if args[1] == "-h" || args[1] ==
 "--help" {
           print_help();
           return;
       }
   }

   println!("WELCOME TO TEXT SOULS");
   println!("Press enter to start");
   // TODO: find a better way to do this
   let mut _e = String::new();
   io::stdin().read_line(&mut _e).ok().expect("This 
 should never happen");



   ds::print_classes();
   println!("Pick a class");
   let mut input = String::new();
   io::stdin().read_line(&mut input).ok()
       .expect("Failure to understand input");







   // &*:  convert input to &'static str
   // because string literals in rust are &'static str
   // and there was a conflict with my Strings
   let mut player = match &*input.trim().to_lowercase() {
       "k" | "knight" => {
           ds::Character::new(
               700,
               "A strong Knight who does average damage",
               ds::Weapon::new("Broadsword", 85, 0.25),
               2)
       } "s" | "sorcerer" => {
           ds::Character::new(
               500,
               "A wizard who carries many items",
               ds::Weapon::new("Catalyst", 100, 0.10),
               5)
       } "t" | "thief" => {
           ds::Character::new(
               400,
               "A dastardly thief with high critical 
     hits",
               ds::Weapon::new("Bandit's Knife", 52,
     0.50),
               3)
       } _ => panic!("Failed trying to assign a 
     class")
   };


   let mut enemies: Vec<ds::Character> = vec![
       ds::Character::new(
           100,
           "An undead soldier",
           ds::Weapon::new("Halberd", 100, 0.1),
           0),
       ds::Character::new(
           85,
           "An undead soldier",
           ds::Weapon::new("Crossbow", 50, 0.1),
           0),
       ds::Character::new(
           100,
           "An undead soldier",
           ds::Weapon::new("Spear", 80, 0.1),
           0),
       ds::Character::new(
           100,
           "An undead soldier",
           ds::Weapon::new("Halberd", 100, 0.1),
           0),
       ds::Character::new(
           85,
           "An undead soldier",
           ds::Weapon::new("Crossbow", 50, 0.1),
           0),
       ds::Character::new(
           1000,
           "BOSS: Taurus Demon",
           ds::Weapon::new("Greataxe", 150, 0.0),
           0)
   ];
   for enemy in &mut enemies {
       println!("\nYou encountered: {} wielding a
 {}!", enemy.description,


    enemy.weapon.name);
       thread::sleep(dur);
       // lost the fight?
       if !player.fight(enemy, dur) {
           println!("\nYOU DIED");
           println!("Killer: {}", enemy.description);
           return;
       } else {
           println!("Enemy has been defeated!\n");
           println!("Your health: {}", player.health);
           println!("Remaining estus: {}",
    player.flasks);
            // r:  ignore escape characters
           println!(r"Use estus? (y\n)");


           let mut ans = String::new();
           io::stdin().read_line(&mut 
     ans).ok().expect("Failed");
           if ans.trim().to_lowercase() == "y" {
               player.heal();
               thread::sleep(dur);
               
           }
       }
   }
   println!("\nVICTORY ACHIEVED");
}


fn print_help() {
   println!("Usage: text-souls [OPTION]");
   println!("Start the Text Souls game.\n");
   println!("With no OPTION, start game as 
normal.\n");
   println!("   -f, --fast\tNo downtime between
attacks");
   println!("   -h, --help\tShow this message\n");
   println!("Examples:");
   println!("   text-souls -f\tStart game in sonic
 mode");
   println!("   text-souls --help\tPrint out this 
 message\n");
   println!("Github: 
 https://github.com/starchery/text-souls");


}
