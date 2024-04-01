use latex_parser::Rule;
use pest::iterators::Pairs;
use std::collections::HashMap;

/// Checks that, for every key in `results`, `expect` contains this key and matches.
/// Useful for smaller commands where all values should be tested for.
macro_rules! test_cmd_sub {
    ($test_name:ident, $cmd:expr, $expect:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn Error>> {
            let pairs = TexParser::parse(Rule::file, $cmd)?;

            let expect: HashMap<Rule, Vec<&'static str>> = $expect.into_iter().collect();

            let mut results = HashMap::new();
            flatten_rules(&mut results, pairs.clone());

            for (k, res) in &results {
                let exp = expect.get(&k).expect(&format!("failed to find key `{k:?}` in expect"));
                assert_eq!(res, exp, "\nPairs: {:#?}", &pairs);
            }
        
            Ok(())
        }
    };
}

/// Checks that, for every key in `expect`, `results` contains this key and matches.
/// Useful for larger commands where certain values are a pain to test for.
macro_rules! test_cmd_sup {
    ($test_name:ident, $cmd:expr, $expect:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn Error>> {
            let pairs = TexParser::parse(Rule::file, $cmd)?;

            let expect: HashMap<Rule, Vec<&'static str>> = $expect.into_iter().collect();

            let mut results = HashMap::new();
            flatten_rules(&mut results, pairs.clone());

            for (k, exp) in &expect {
                let res = results.get(&k).expect(&format!("failed to find key `{k:?}` in res"));
                assert_eq!(res, exp, "\nPairs: {:#?}", &pairs);
            }
        
            Ok(())
        }
    };
}

fn flatten_rules(map: &mut HashMap<Rule, Vec<&'static str>>, pairs: Pairs<'static, Rule>) {
    for pair in pairs {
        let rule = pair.as_rule();
        let s = pair.as_str();

        map.entry(rule)
            .and_modify(|vals| vals.push(s))
            .or_insert(vec![s]);

        let pairs = pair.into_inner();
        flatten_rules(map, pairs)
    }
}

mod commands {
    use std::collections::HashMap;
    use std::error::Error;

    use pest::Parser;

    use latex_parser::Rule;
    use latex_parser::TexParser;

    use crate::parser::flatten_rules;

    test_cmd_sub!(
        simple_a,
        r"\a",
        vec![(Rule::cmd, vec![r"\a"]), (Rule::cmd_name, vec!["a"])]
    );

    test_cmd_sub!(
        simple_b,
        r"\b[]",
        vec![(Rule::cmd, vec![r"\b[]"]), (Rule::cmd_name, vec!["b"]), (Rule::cmd_opts, vec!["[]"])]
    );

    test_cmd_sub!(
        simple_c,
        r"\c[value]",
        vec![
            (Rule::cmd, vec![r"\c[value]"]),
            (Rule::cmd_name, vec!["c"]),
            (Rule::cmd_opts, vec!["[value]"]),
            (Rule::cmd_opt_v, vec!["value"]),
        ]
    );

    test_cmd_sub!(
        simple_d,
        r"\d[foo=bar]",
        vec![
            (Rule::cmd, vec![r"\d[foo=bar]"]),
            (Rule::cmd_name, vec!["d"]),
            (Rule::cmd_opts, vec!["[foo=bar]"]),
            (Rule::cmd_opt_kv, vec!["foo=bar"]),
            (Rule::cmd_opt_k, vec!["foo"]),
            (Rule::cmd_opt_v, vec!["bar"]),
        ]
    );

    test_cmd_sub!(
        simple_e,
        r"\e[bar=baz, foo = bar]",
        vec![
            (Rule::cmd, vec![r"\e[bar=baz, foo = bar]"]),
            (Rule::cmd_name, vec!["e"]),
            (Rule::cmd_opts, vec!["[bar=baz, foo = bar]"]),
            (Rule::cmd_opt_kv, vec!["bar=baz", "foo = bar"]),
            (Rule::cmd_opt_k, vec!["bar", "foo"]),
            (Rule::cmd_opt_v, vec!["baz", "bar"]),
        ]
    );

    test_cmd_sup!(
        simple_f,
        r"\f[
          foo=1mm,
          bar=\roman,
          value,
          baz=baz3
        ]",
        vec![
            (Rule::cmd_name, vec!["f", "roman"]),
            (Rule::cmd_opt_kv, vec!["foo=1mm", r"bar=\roman", "baz=baz3"]),
            (Rule::cmd_opt_k, vec!["foo", "bar", "baz"]),
            (Rule::cmd_opt_v, vec!["1mm", r"\roman", "value", "baz3"]),
        ]
    );

    test_cmd_sup!(
        with_subcommands,
        r"\main[a, b, c]{\foo}{\bar}{\baz}",
        vec![
            (Rule::cmd_name, vec!["main", "foo", "bar", "baz"]),
            (Rule::cmd_opt_v, vec!["a", "b", "c"]),
        ]
    );
}

mod text {}
