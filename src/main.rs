use std::fs::File;
use std::io::Read;
use std::io::Result;

const ALPH: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Context {
    // the previous character
    p: char,

    // previous significant character
    psc: char,

    // whether we're currenctly parsing a command
    cmd: bool,

    // whether we're current parsing optional args
    opt: bool,

    // the char index of the end of the last command
    cmd_end_i: usize,

    // whether we're in math mode
    math: bool,

    // how many scopes deep we are
    depth: usize,
}

struct Command<'a> {
    name: &'a str,
    ops: &'a str,
    args: &'a str
}

fn main() -> Result<()> {
    let mut file = File::open("./data/sample.tex")?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    let mut ctx = Context {
        p: ' ',
        psc: ' ',
        cmd: false,
        opt: false,
        cmd_end_i: 0,
        math: false,
        depth: 0
    };

    let mut line_num = 1;
    for (i, c) in data.chars().enumerate() {
        if c == '\n' { line_num += 1; }

        // command start
        if ctx.p == '\\' && ALPH.contains(c) {
            ctx.cmd = true;
        }

        // command end
        if ctx.cmd && !ALPH.contains(c) {
            ctx.cmd = false;
            ctx.cmd_end_i = i;
        }

        // start optional args
        // FIXME: this -1 check would fail on whitespace
        if c == '[' && ctx.cmd_end_i == i - 1 {
            ctx.opt = true;
        }

        // end optional args
        if c == ']' && ctx.opt { 
            ctx.opt = false;
        }

        // open cbrace
        if c == '{' && ctx.p != '\\' {
            ctx.depth += 1;
        }

        // close cbrace
        if c == '}' && ctx.p != '\\' {
            ctx.depth -= 1;
        }

        // toggle inline math
        if ctx.p != '\\' && c == '$' {
            ctx.math = !ctx.math;
        }

        // start multi-line math
        if ctx.p == '\\' && c == '[' {
            ctx.math = true;
        }

        // end multi-line math
        if ctx.p == '\\' && c == ']' {
            ctx.math = false;
        }

        ctx.p = c;

        if !c.is_whitespace() { ctx.psc = c; }
    }

    Ok(())
}
