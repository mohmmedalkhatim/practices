mod hero;
use hero::Hero; 

fn render(hero:Hero,V:Vec<Vec<&str>>){
    for v1 in V  {
        print!("\n");
        for v2 in v1  {
            print!("  {}", v2);
        };

    };
}



fn init(){
    let mut baurd: Vec<Vec<&str>> = Vec::new();
    let mut Hero = Hero::new("mohammed".to_string());
    baurd = vec![
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", "_", "_", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
    ];
    render(Hero,baurd);

}