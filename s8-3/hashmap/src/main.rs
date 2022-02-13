use std::collections::HashMap;

fn main() {
    let teams  = vec![
        String::from("Blue"), 
        String::from("Yellow")
        ];
    let initial_scores = vec![10, 50];
    
    let mut scores: HashMap<_, _> = teams.iter().
        zip(initial_scores.iter()).collect();

    let team_name = String::from("Green");
    let team_score = 20;

    // let mut scores = HashMap::new();
    // scores.insert(team_name, team_score);

    scores.insert(&team_name, &team_score);

    println!("{:#?}", scores);

    println!("{}", team_name);
    println!("{}", team_score);
}
