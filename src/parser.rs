use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tex.pest"]
pub struct TexParser;

/*
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

pub fn parse_cmd_args(args_pairs: Pairs<Rule>) -> Vec<Option<&'_ str>> {
    args_pairs
        .map(|pair| pair.into_inner().next().map(|pair| pair.as_str()))
        .collect()
}

pub fn parse_cmd(cmd_pairs: Pairs<Rule>) -> Ast {
    let mut name = "";
    let mut opts = vec![];
    let mut args = vec![];

    for pair in cmd_pairs {
        match pair.as_rule() {
            Rule::cmd_name => {
                name = pair.as_str().trim();
            }

            Rule::cmd_opts => {
                let opts_pairs = pair.into_inner();
                opts = parse_cmd_opts(opts_pairs);
            }

            Rule::cmd_args => {
                let args_pairs = pair.into_inner();
                args = parse_cmd_args(args_pairs);
            }

            _ => unreachable!(),
        }
    }

    Ast::Command { name, opts, args }
}

pub fn parse_e(pair: Pair<Rule>) -> Ast {
    match pair.as_rule() {
        Rule::cmd => parse_cmd(pair.into_inner()),

        Rule::text => Ast::Text(pair.as_str()),

        _ => {
            println!("uh oh! got\n{pair:#?}");

            unreachable!();
        }
    }
}
*/
