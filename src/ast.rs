#[derive(Debug)]
pub enum CommandOption<'a> {
    Value(&'a str),
    KeyValue(&'a str, &'a str),
}

#[derive(Debug)]
pub enum Ast<'a> {
    Command {
        /// The name of the command
        name: &'a str,

        /// Any optional arguments go here
        opts: Vec<CommandOption<'a>>,

        /// Function arguments.
        /// `Option` because arguments may be empty
        args: Vec<Option<&'a str>>,
    },

    Text(&'a str),
}
