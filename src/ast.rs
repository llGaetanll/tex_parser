#[derive(Debug, Clone)]
pub enum CommandOption<'a> {
    Value(&'a str),
    KeyValue(&'a str, &'a str),
}

#[derive(Debug, Clone)]
pub enum CommandArg<'a> {
    Optional(Vec<CommandOption<'a>>),
    Required(Vec<Ast<'a>>),
}

#[derive(Debug, Clone)]
pub struct Command<'a> {
    /// The name of the command
    pub name: &'a str,

    /// Any arguments, optional or required.
    pub args: Vec<CommandArg<'a>>,
}

#[derive(Debug, Clone)]
pub enum MathContents<'a> {
    Text(&'a str),
    Cmd(Command<'a>),
}

#[derive(Debug, Clone)]
pub enum MathMode {
    Inline,
    MultiLine,
}

#[derive(Debug, Clone)]
pub struct Math<'a> {
    pub mode: MathMode,
    pub data: Vec<MathContents<'a>>,
}

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    Cmd(Command<'a>),
    Text(&'a str),
    Math(Math<'a>),
    Scope(Vec<Ast<'a>>),
    LineBreak,
    Environment {
        // the name of the environment is the
        // contents of the first required argument.
        name: &'a str,

        // environments might still have command args
        args: Vec<CommandArg<'a>>,

        contents: Vec<Ast<'a>>,
    },
}
