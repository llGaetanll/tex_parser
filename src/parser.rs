use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::Ast;
use crate::ast::Command;
use crate::ast::CommandArg;
use crate::ast::CommandOption;
use crate::ast::Math;
use crate::ast::MathContents;
use crate::ast::MathMode;

#[derive(Parser)]
#[grammar = "tex.pest"]
pub struct TexParser;

pub fn parse_cmd_opts(opts_pair: Pair<Rule>) -> Vec<CommandOption> {
    opts_pair
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::cmd_opt_v => {
                let s = pair.as_str();

                CommandOption::Value(s)
            }

            Rule::cmd_opt_kv => {
                let [pair_k, pair_v]: [Pair<Rule>; 2] =
                    pair.into_inner().collect::<Vec<_>>().try_into().unwrap();

                let k = pair_k.as_str();
                let v = pair_v.as_str();

                CommandOption::KeyValue(k, v)
            }

            _ => unreachable!(),
        })
        .collect()
}

pub fn parse_cmd(cmd: Pair<Rule>) -> Command {
    let mut pairs = cmd.into_inner();

    // `cmd_call` is always first
    let name = pairs.next().unwrap().into_inner().next().unwrap().as_str();

    // the rest are arguments
    let args: Vec<_> = pairs
        .map(|pair| match pair.as_rule() {
            Rule::cmd_opts => CommandArg::Optional(parse_cmd_opts(pair)),
            Rule::scope => CommandArg::Required(parse_scope(pair)),

            _ => unreachable!(),
        })
        .collect();

    Command { name, args }
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
            Rule::cmd => MathContents::Cmd(parse_cmd(pair)),
            Rule::text => MathContents::Text(pair.as_str()),

            _ => unreachable!(),
        })
        .collect();

    Math { mode, data }
}

pub fn parse_scope(scope: Pair<Rule>) -> Vec<Ast> {
    scope
        .into_inner()
        .nth(1) // 0 is scope open, 1 is content, 2 is scope close
        .unwrap()
        .into_inner()
        .map(parse_e)
        .collect()
}

pub fn parse_e(pair: Pair<Rule>) -> Ast {
    match pair.as_rule() {
        Rule::cmd => Ast::Cmd(parse_cmd(pair)),
        Rule::scope => Ast::Scope(parse_scope(pair)),
        Rule::text => Ast::Text(pair.as_str()),
        Rule::math => Ast::Math(parse_math(pair.into_inner().next().unwrap())),

        Rule::linebreak => Ast::LineBreak,

        _ => {
            println!("uh oh! got\n{pair:#?}");

            unreachable!();
        }
    }
}
