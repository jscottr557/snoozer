use std::str::SplitWhitespace;
use std::str;

const COMMAND_CHAR: char = '!';
pub enum Commands {
	QUIT,
	REGISTER,
	UNREGISTER,
	INVAL,
}

pub fn string_is_command(message: &String) -> bool {

	let first_char: u8 = message.as_bytes()[0];

	if first_char.is_ascii() && first_char as char == COMMAND_CHAR {
			return true;
	}

	return false;
}

//Yes, this is overcomplicated, but I want to get good experience with match and working with Strings/strs and references
pub fn parse_command(message: &String) -> (Commands, SplitWhitespace) {
	let mut arguments = message.split_whitespace();

	let command_name = match arguments.next() {
		Some(command_name) => {
			match command_name.find('!') {
				Some(exclamation_index) => { str::from_utf8(&command_name.as_bytes()[(exclamation_index + 1)..]).unwrap() }, //This is illegible :)
				None => {"INVALID"},
			}
		},
		None => {"INVALID"},
	};
	let command;

	command = match command_name {
		"quit" => {Commands::QUIT},
		"register" => {Commands::REGISTER}
		"unregister" => {Commands::UNREGISTER},
		_ => {Commands::INVAL},
	};

	return (command, arguments);
}