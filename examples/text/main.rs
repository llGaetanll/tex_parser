use std::error::Error;
use std::fs::File;
use std::io::Read;

use tex_parser::parser::parse_e;
use tex_parser::util::merge_env;
use tex_parser::Rule;
use tex_parser::TexParser;

use pest::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/text/data/sample2.tex")?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let res = TexParser::parse(Rule::file, &data)?;
    let res = res.map(parse_e).collect::<Vec<_>>();
    let res = merge_env(&mut res.into_iter(), None);

    println!("{res:#?}");

    Ok(())
}
