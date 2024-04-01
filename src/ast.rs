#[derive(Debug)]
pub enum CommandArgument<'a> {
    Text(&'a str),
    Cmd(Command<'a>),
    Math(Vec<Math<'a>>)
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
    pub args: Vec<Option<Vec<CommandArgument<'a>>>>,
}

#[derive(Debug)]
pub enum MathContents<'a> {
    Text(&'a str),
    Cmd(Command<'a>)
}

#[derive(Debug)]
pub enum MathMode { Inline, MultiLine }

#[derive(Debug)]
pub struct Math<'a> {
    pub mode: MathMode,
    pub data: Vec<MathContents<'a>>
}

#[derive(Debug)]
pub enum Ast<'a> {
    Cmd(Command<'a>),
    Text(&'a str),
    Math(Math<'a>),
    ScopeOpen,
    ScopeClose,
    LineBreak
}
