mod hero;
use std::{clone, time::Duration};

use hero::Hero;
use tokio::time::sleep; 

fn render(hero:Hero,v:&Vec<Vec<&str>>){
    for v1 in v  {
        print!("\n");
        for v2 in v1  {
            print!("  {}", v2);
        };

    };
    println!("\n");
}



pub fn init(){
    let mut baurd: Vec<Vec<&str>> = Vec::new();
    let mut hero = Hero::new("mohammed".to_string());
    baurd = vec![
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", "_", "_", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
    ];

    while hero.hb.clone() != 0 {
        let _ = sleep(Duration::from_millis(0_34234));
        hero.hb -= 1;
        render(hero.clone(), &baurd);
    }
}