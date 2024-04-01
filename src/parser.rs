use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest_derive::Parser;

use crate::ast::Ast;
use crate::ast::Command;
use crate::ast::CommandArgument;
use crate::ast::CommandOption;
use crate::ast::Math;
use crate::ast::MathContents;
use crate::ast::MathMode;

#[derive(Parser)]
#[grammar = "tex.pest"]
pub struct TexParser;

pub fn parse_cmd_opts(opts_pairs: Pairs<Rule>) -> Vec<CommandOption> {
    opts_pairs
        .map(|pair| {
            let cmd_opt = pair.into_inner().next().unwrap();

            match cmd_opt.as_rule() {
                Rule::cmd_opt_v => {
                    let s = cmd_opt.as_str();

                    CommandOption::Value(s)
                }

                Rule::cmd_opt_kv => {
                    let [pair_k, pair_v]: [Pair<Rule>; 2] =
                        cmd_opt.into_inner().collect::<Vec<_>>().try_into().unwrap();

                    let k = pair_k.as_str();
                    let v = pair_v.as_str();

                    CommandOption::KeyValue(k, v)
                }

                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn parse_cmd_arg(cmd_args: Pairs<Rule>) -> Option<Vec<CommandArgument>> {
    let mut args = Vec::new();

    for arg in cmd_args {
        match arg.as_rule() {
            Rule::cmd_arg_empty => return None,
            Rule::cmd_arg => {
                let arg = arg.into_inner().next().unwrap();

                args.push(match arg.as_rule() {
                    Rule::cmd => CommandArgument::Cmd(parse_cmd(arg.into_inner())),
                    Rule::text => CommandArgument::Text(arg.as_str()),

                    _ => unreachable!(),
                })
            }

            _ => unreachable!(),
        }
    }

    Some(args)
}

pub fn parse_cmd(cmd: Pairs<Rule>) -> Command {
    let mut name = "";
    let mut opts = vec![];
    let mut args: Vec<Option<Vec<CommandArgument>>> = vec![];

    for pair in cmd {
        match pair.as_rule() {
            Rule::cmd_name => {
                name = pair.as_str().trim();
            }

            Rule::cmd_opts => {
                opts = parse_cmd_opts(pair.into_inner());
            }

            Rule::cmd_args => {
                args.push(parse_cmd_arg(pair.into_inner()));
            }

            _ => unreachable!(),
        }
    }

    Command { name, opts, args }
}

pub fn parse_math(math: Pair<Rule>) -> Math {
    let mode = match math.as_rule() {
        Rule::math_inline => MathMode::Inline,
        Rule::math_multiline => MathMode::MultiLine,

        _ => unreachable!(),
    };

    let data = math
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::cmd => MathContents::Cmd(parse_cmd(pair.into_inner())),
            Rule::text => MathContents::Text(pair.as_str()),

            _ => unreachable!(),
        })
        .collect();

    Math { mode, data }
}

pub fn parse_e(pair: Pair<Rule>) -> Ast {
    match pair.as_rule() {
        Rule::cmd => Ast::Cmd(parse_cmd(pair.into_inner())),
        Rule::text => Ast::Text(pair.as_str()),
        Rule::math => Ast::Math(parse_math(pair.into_inner().next().unwrap())),

        Rule::scope_open => Ast::ScopeOpen,
        Rule::scope_close => Ast::ScopeClose,
        Rule::linebreak => Ast::LineBreak,

        _ => {
            println!("uh oh! got\n{pair:#?}");

            unreachable!();
        }
    }
}
