use std::fs::File;
use std::error::Error;
use std::io::Read;

use latex_parser::TexParser;
use latex_parser::Rule;
use latex_parser::parser::parse_e;

use pest::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/text/data/sample.tex")?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let res = TexParser::parse(Rule::file, &data)?;
    let res = res.map(parse_e).collect::<Vec<_>>();

    println!("{res:#?}");

    Ok(())
}
