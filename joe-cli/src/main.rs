
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::{io, thread, time};

// --------------------------------------------------------

// ==========================
fn random_message() {
    let message_array = [
        "Live and Let Die",
        "Band on the Run",
        "Maybe I'm Amazed",
        "While My Guitar Gently Weeps",
        "Let it Be"
    ];
    let random_index = rand::thread_rng()
        .gen_range(0, message_array.len() - 1);
    println!("\n\n----------------------------------------------");
    println!("\n{}\n", message_array[random_index]);
    println!("----------------------------------------------\n\n")
}

// ==========================
fn countdown() {
    let mut x = 5;
    while x > 0 {
        println!("\n===");
        println!(" {} ", x);
        println!("===\n");
        thread::sleep(time::Duration::from_secs(1));
        x = x - 1;
    }
    println!("\n========");
    println!("Launch!!");
    println!("========\n");
}

// ==========================
fn login() {

    let email_ref: Regex = Regex::new(r"[A-Za-z0-9\.]*@[A-Za-z0-9]*\.[A-Za-z0-9]*").unwrap();
    let mut email = String::new();
    let mut email_format_correct: bool = false;
    while !email_format_correct {
        println!("\nEnter email address:");
        io::stdin().read_line(&mut email);
        if email_ref.is_match(&email) {
            email_format_correct = true;
        } else {
            println!("\nIncorrect email format! Try again.")
        };
    }
    let mut password = String::new();
    println!("\nEnter password:");
    io::stdin().read_line(&mut password);
    println!("\nLogin successful!")
}

// ==========================
fn snake_render(pattern: &str) {
    thread::sleep(time::Duration::from_millis(50));
    println!("{}", pattern);

}

fn snake() {
    snake_render("Let's print a snake!");
    thread::sleep(time::Duration::from_secs(1));
    snake_render("\n\n");
    snake_render("       ^");
    snake_render("       \\\\");
    snake_render("        \\ \\");
    snake_render("        | |");
    snake_render("        | |");
    snake_render("        / /");
    snake_render("       / /");
    snake_render("      / /");
    snake_render("     | |");
    snake_render("     | |");
    snake_render("     | |");
    snake_render("    / /");
    snake_render("   / /");
    snake_render("  | |");
    snake_render("  | |");
    snake_render("  / \\");
    snake_render(" /   \\");
    snake_render(" |    |");
    snake_render(" |    |");
    snake_render("| o  o |");
    snake_render(" \\    /");
    snake_render("    |");
    snake_render("    |");


}

// ==========================
// TODO: Null
struct Node {
    val: i32,
    // prev: Node,
}

fn traverse(current_node: Node) {

}

fn linked_list() {
    // let head: Node = Node {
    //     val: 1
    // };
    // for n in 2..10 {
    //
    // }
}

// ==========================
// TODO: Getters and Setters
fn race() {
    //
}

// ==========================
// TODO: JSON
fn json() {
    //
}


// --------------------------------------------------------

fn launch() {

    loop {

        println!("Enter command:");

        let mut command = String::new();
        io::stdin().read_line(&mut command);

        match command.trim() {
            "random" => {
                println!("Launching Random Message Generator...");
                thread::sleep(time::Duration::from_secs(2));
                random_message();
                println!("Type quit to quit");
            },
            "countdown" => {
                println!("Launching Count Down...");
                thread::sleep(time::Duration::from_secs(2));
                countdown();
                println!("Type quit to quit");
            },
            "login" => {
                println!("Launching Login...");
                thread::sleep(time::Duration::from_secs(2));
                login();
                println!("Type quit to quit");
            },
            "snake" => {
                println!("Launching Snake...");
                thread::sleep(time::Duration::from_secs(2));
                snake();
                println!("Type quit to quit");
            },
            "linkedlist" => {
                println!("Launching Linked List...");
                thread::sleep(time::Duration::from_secs(2));
                linked_list();
                println!("Type quit to quit");
            },
            "race" => {
                println!("Launching Race...");
                thread::sleep(time::Duration::from_secs(2));
                race();
                println!("Type quit to quit");
            },
            "json" => {
                println!("Launching JSON...");
                thread::sleep(time::Duration::from_secs(2));
                json();
                println!("Type quit to quit");
            },
            "quit" => break,
            _ => {
                println!("Not a valid command. Try again.");
            }
        }
    }
}

fn graceful_shutdown() {
    println!("Goodbye.")
}

// --------------------------------------------------------

fn main() {
    // launch();
    // graceful_shutdown();
    login();
}
