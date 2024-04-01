#[derive(Debug)]
pub enum CommandArgument<'a> {
    Text(&'a str),
    Cmd(Command<'a>)
}

#[derive(Debug)]
pub enum CommandOption<'a> {
    Value(&'a str),
    KeyValue(&'a str, &'a str),
}

#[derive(Debug)]
pub struct Command<'a> {
    /// The name of the command
    pub name: &'a str,

    /// Any optional arguments go here
    pub opts: Vec<CommandOption<'a>>,

    /// Function arguments.
    /// `Option` because arguments may be empty
    pub args: Vec<Option<CommandArgument<'a>>>,
}

#[derive(Debug)]
pub enum Ast<'a> {
    Cmd(Command<'a>),
    Text(&'a str),
}
