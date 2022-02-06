
use std::{io, thread, time};

// --------------------------------------------------------

fn random_message() {
    //
}

fn countdown() {
    //
}

fn login() {
    //
}

fn snake() {
    //
}

fn linked_list() {
    //
}

fn race() {
    //
}

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
                break;
            },
            "countdown" => {
                println!("Launching Count Down...");
                thread::sleep(time::Duration::from_secs(2));
                countdown();
                break;
            },
            "login" => {
                println!("Launching Login...");
                thread::sleep(time::Duration::from_secs(2));
                login();
                break;
            },
            "snake" => {
                println!("Launching Snake...");
                thread::sleep(time::Duration::from_secs(2));
                snake();
                break;
            },
            "linkedlist" => {
                println!("Launching Linked List...");
                thread::sleep(time::Duration::from_secs(2));
                linked_list();
                break;
            },
            "race" => {
                println!("Launching Race...");
                thread::sleep(time::Duration::from_secs(2));
                race();
                break;
            },
            "json" => {
                println!("Launching JSON...");
                thread::sleep(time::Duration::from_secs(2));
                json();
                break;
            },
            _ => {
                println!("Not a valid command. Try again.");
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
}

fn graceful_shutdown() {
    println!("Goodbye.")
}

// --------------------------------------------------------

fn main() {
    launch();
    graceful_shutdown();
}
