extern crate discord;

use discord::model::Event;
use discord::Discord;
use std::env;

use commands::Commands;
mod commands;
fn main() {
    //login
    let token: &str = &env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN envvar not present");
    let discord = Discord::from_bot_token(token).expect("bot login failed!");

    let(mut connection, _) = discord.connect().expect("bot connection failed!");
    println!("bot ready, logged on as {}", discord.get_current_user().unwrap().username); //unwrap should be safe, protected by valid/invalid connection

    loop { match connection.recv_event() {
        Ok(Event::MessageCreate(message)) => {
            let message_content = &message.content;
            if commands::string_is_command(message_content) {
                let (command, _arguments) = commands::parse_command(message_content);
                match command {
                    Commands::QUIT => {break},
                    Commands::REGISTER => {println!("register")},
                    Commands::UNREGISTER => {println!("unregister")},
                    Commands::INVAL => {println!("invalid")},
                }
            }
        }
        Ok(_) => {}
        Err(err) => {println!("An error occured! {err:?}"); break} //this catches any and all connection errors
    }}

}
