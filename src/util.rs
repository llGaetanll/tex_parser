use crate::ast::Ast;
use crate::ast::CommandArg;

/// Takes an iterator for an `Ast` and merges any environments, recursively.
pub fn merge_env<'a, I>(ast_iter: &mut I, env: Option<&'a str>) -> Vec<Ast<'a>>
where
    I: Iterator<Item = Ast<'a>>,
{
    // get the name of an environment
    let get_name = |args: &[CommandArg<'a>]| -> &str {
        args.iter()
            .find_map(|arg| match arg {
                CommandArg::Optional(_) => None,
                CommandArg::Required(arg) => match arg.first() {
                    Some(Ast::Text(name)) => Some(name),

                    _ => panic!("environment with no name!"), // TODO: don't panic
                },
            })
            .expect("environment with no name!") // ibid
    };

    let mut nodes = Vec::new();

    while let Some(node) = ast_iter.next() {
        match (&node, env) {
            (Ast::Cmd(cmd), Some(env)) if cmd.name == "end" && get_name(&cmd.args) == env => break,

            (Ast::Cmd(cmd), _) if cmd.name == "begin" => {
                let name = get_name(&cmd.args);

                nodes.push(Ast::Environment {
                    name,
                    args: cmd.args.clone(),
                    contents: merge_env(ast_iter, Some(name)),
                });
            }

            _ => {
                nodes.push(node);
            }
        }
    }

    nodes
}
