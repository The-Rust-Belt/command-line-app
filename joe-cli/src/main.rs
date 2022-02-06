
use std::io;

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
        let mut command = String::new();
        io::stdin().read_line(&mut command);

        match command.as_str() {
            "random" => {
                println!("Launching Random Message Generator...");
                random_message();
                break;
            },
            "countdown" => {
                println!("Launching Count Down...");
                countdown();
                break;
            },
            "login" => {
                println!("Launching Login...");
                login();
                break;
            },
            "snake" => {
                println!("Launching Snake...");
                snake();
                break;
            },
            "linkedlist" => {
                println!("Launching Linked List...");
                linked_list();
                break;
            },
            "race" => {
                println!("Launching Race...");
                race();
                break;
            },
            "json" => {
                println!("Launching JSON...");
                json();
                break;
            },
            _ => {
                println!("Not Command. Try Again");
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
