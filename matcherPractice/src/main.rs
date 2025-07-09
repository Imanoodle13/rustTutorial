/* Skill Matching */
fn event_similarity_ratio(event_skill_profile: &Vec<String>, volunteer_skill_profile: &Vec<String>) -> (f32,f32){
    let mut sorted_esp: Vec<String> = event_skill_profile.clone();
    let mut sorted_vsp: Vec<String> = volunteer_skill_profile.clone();
    sorted_esp.sort();
    sorted_vsp.sort();
    
    let mut event_similarity_index: f32 = 0.0;
    for skill in &sorted_vsp {
        for event_skill in &sorted_esp {
            if skill == event_skill {
                event_similarity_index += 1.0;
            }
        }
    }

    let esi_ratio: f32 = event_similarity_index / (event_skill_profile.len() as f32);
    return (event_similarity_index,esi_ratio);
}
fn main(){
    // Event Skill Profiles
    let event_skill_profile1: Vec<String> = vec!["Rust".to_string(), "Python".to_string(), "JavaScript".to_string()];
    let event_skill_profile2: Vec<String> = vec!["Java".to_string(), "C#".to_string()    , "Ruby".to_string()  , "Swift".to_string()];
    let event_skill_profile3: Vec<String> = vec!["PHP".to_string() , "Perl".to_string()  ,  "Scala".to_string(), "Python".to_string(), "Elixir".to_string()];

    // Input volunteer skills
    let mut volunteer_skill_profile: Vec<String> = Vec::new();
    println!("Please enter your skills (5 skills):");
    for i in 0..5 {
        println!("ENTER SKILL {}: ", i + 1);
        let mut skill: String = "".to_string();
        
        std::io::stdin().read_line(&mut skill).expect("Failed to read line");
        volunteer_skill_profile.push(skill.trim().to_string());
    }

    // Display results
    let (mut count,mut ratio) = event_similarity_ratio(&event_skill_profile1,&volunteer_skill_profile);
    println!("\nEVENT SIMILARITIES:");
    println!("EVENT 1: Count = {}, Ratio = {:.2}", count,ratio);

    (count,ratio)                       = event_similarity_ratio(&event_skill_profile2,&volunteer_skill_profile);
    println!("EVENT 2: Count = {}, Ratio = {:.2}", count,ratio);

    (count,ratio)                       = event_similarity_ratio(&event_skill_profile3,&volunteer_skill_profile);
    println!("EVENT 3: Count = {}, Ratio = {:.2}", count,ratio);
}