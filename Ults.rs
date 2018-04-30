extern crate rand;

use self::rand::Rng;
use std::thread;
use std::time;
// use std::io;

pub struct Character {
    pub health: i32,
    max_health: i32,
    pub description: &'static str,
    pub weapon: Weapon,
    pub flasks: u8, // health potions
}

pub struct Weapon {
    pub name: &'static str,
    pub damage: i32,
    pub crit_chance: f64
}

impl Character {
    pub fn new(hp: i32, bio: &'static str, wepon: Weapon, estus: u8) -> Character {
        Character {
            health: hp,
            max_health: hp,
            description: bio,
            weapon: wepon,
            flasks: estus
        }
    }


    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn heal(&mut self) {
        if self.flasks == 0 {
            println!("Out of estus!");
        } else {
            self.flasks -= 1;
            self.health += (self.max_health as f64 / 1.5f64) as i32;
            if self.health > self.max_health {
                self.health = self.max_health;
            }
            println!("Health recovered to {}.", self.health);
        }
    }

    // lol
    pub fn will_roll(&self) -> bool {
        rand::thread_rng().gen_range(0, 2) == 1
    }

    pub fn will_attack(&self) -> bool {
        rand::thread_rng().gen_range(0, 2) == 1
    }

    // dur:  time between each action, in milliseconds
    pub fn fight(&mut self, enemy: &mut Character, 
                 dur: time::Duration) -> bool {
        loop {
            thread::sleep(dur);
            if self.is_dead() {
                return false;
            } else if enemy.is_dead() {
                return true;
            }

            if self.will_attack() {
                // lesser chance for enemy to roll
                if enemy.will_roll() && enemy.will_roll() {
                    println!("YOU MISS the enemy!");
                } else {
                    // for printing damage
                    let dmg = self.weapon.get_damage();
                    enemy.health -= dmg;
                    println!("[HIT] You hit the enemy for {} damage!", dmg);
                    println!("He now has {} health.", enemy.health);
                }
            } else {
                if self.will_roll() {
                    println!("The ENEMY MISSED you!");
                } else {
                    let dmg = enemy.weapon.get_damage();
                    self.health -= dmg;
                    println!("[HIT] The enemy hit you for {} damage!", dmg);
                    println!("You now have {} health.", self.health);
                } // else
            } // else
        //thread::sleep(dur);
        } // loop
    } // fn
} // impl char

impl Weapon {
    pub fn new(desc: &'static str, dmg: i32, crit: f64) -> Weapon {
        Weapon {
            name: desc,
            damage: dmg,
            crit_chance: crit
        }
    }

    pub fn get_damage(&self) -> i32 {
        let will_crit = rand::thread_rng()
            .gen_range(0.0f64, 1.0f64) < self.crit_chance;
        if will_crit {
            print!("[CRIT] ");
            rand::thread_rng().gen_range((self.damage as f64) * 2.4,
                (self.damage as f64) * 5.0) as i32
        } else {
            rand::thread_rng().gen_range((self.damage as f64) / 1.5,
                (self.damage as f64) * 1.2) as i32
        }

    }
}

pub fn print_classes() {
    println!("(K)night");
    println!("\t700 hp");
    println!("\tA strong Knight who does average damage");
    println!("\tBroadsword");
    println!("\tEstus Flask (2)\n");

    println!("(S)orcerer");
    println!("\t500 hp");
    println!("\tA wizard who carries many items");
    println!("\tSpell Catalyst");
    println!("\tEstus Flask (5)\n");

    println!("(T)hief");
    println!("\t400 hp");
    println!("\tA dastardly thief with high critical hits");
    println!("\tBandit's knife");
    println!("\tEstus Flask (3)\n");
}

/*pub fn class_info(character: &Character) {
    println!("HP: {}", character.health);
    println!("BIO: {}", character.description);
    println!("WEP: {}: {}", character.weapon.name, character.weapon.damage);
    println!("ESTUS: {}", character.flasks);
}*/
