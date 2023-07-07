#![warn(unused_parens)]
#![warn(non_snake_case)]
use std::time::Duration;

use chrono::Utc;
fn clear_console() {
    let _ = std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status();
}   
fn main() {
    loop {
        clear_console();
        let mut input: String = Default::default();
        println!("When do you want your computer to turn off? ex: 19 20\nIf you have already initiated a shutdown press 0 to cancel it!");
        std::io::stdin().read_line(&mut input).unwrap();
        if &mut input.trim() == &"0" {
            let out = std::process::Command::new("cmd")
            .arg("/C")
            .arg("shutdown -a")
            .output()
            .expect("Failed to write command");
            let stdout = String::from_utf8_lossy(&out.stdout);
            println!("Command exceuted succesfully");
            std::io::stdin().read_line(&mut input).unwrap();
            input.clear();
            continue;
        }
        let mut vTime: Vec<String> = Vec::new();
        vTime.extend(input.trim().split(' ').map(|s| s.to_string()));

        // Get the current time
        let current_time = Utc::now().format("%H %M").to_string();
        vTime.extend(current_time.trim().split(' ').map(|s| s.to_string()));
        //convert everything into minutes

        let vtime0 : i32 = vTime[0].parse().expect("Only numbers are accepted!");
        let vtime1 : i32 = vTime[1].parse().expect("Only numbers are accepted!");
        let user_date: i32 = (vtime0 * 60 + vtime1) * 60;

        let vtime2 : i32 = vTime[2].parse().expect("Only numbers are accepted!");
        let vtime3 : i32 = vTime[3].parse().expect("Only numbers are accepted!");

        //i have to manually add 2 hours to the counter cuz of cest
        let pc_date: i32 =  ((vtime2 + 2) * 60 + vtime3) * 60;

        //user_date - pc_date assuming user_input is bigger
        let turn_date: i32 = user_date - pc_date;
        let combinedcommand = "shutdown -s -t ".to_owned() + &turn_date.to_string();
        let out = std::process::Command::new("cmd")
            .arg("/C")
            .arg(combinedcommand)
            .output()
            .expect("Failed to write command");
        //TODO : Do you want to make this re-occuring? (Always turn off your pc at this time)
        let hturn:f64 = turn_date as f64 / 3600.0;
        clear_console();
        
        let stdout = String::from_utf8_lossy(&out.stdout);
        println!("Console output : {}", stdout);
        println!("You computer is going to turn off in : {} seconds ({} hours)\n", turn_date, hturn);
        std::io::stdin().read_line(&mut input).unwrap();

        input.clear();
        vTime.clear();
    }
}
