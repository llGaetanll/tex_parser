# `tex_parser`

**⚠️ Warning**: This is **not** a formal TeX parser. TeX [cannot actually be
parsed as a context free grammar](https://tex.stackexchange.com/questions/4201/is-there-a-bnf-grammar-of-the-tex-language), and requires a turing machine to handle any
edge cases

Despite this, most real world TeX is "well-behaved" enough that a context free
grammar may suffice. To that end, this library attempts to provide a "best
effort" solution to parsing TeX. This library is naturally opinionated.

## How it works

`tex_parser` uses `pest` under the hood to parse the file (the grammar can be
found in `tex.pest`) and turns it into an AST. 

<details>
<summary>Example file and AST output</summary>

Consider the following file
```tex
\begin{document}
  Hello world
\end{document}
```

The resulting AST would be
```
[
    Cmd(
        Command {
            name: "begin",
            args: [
                Required(
                    [
                        Text(
                            "document",
                        ),
                    ],
                ),
            ],
        },
    ),
    Text(
        "Hello World",
    ),
    Cmd(
        Command {
            name: "end",
            args: [
                Required(
                    [
                        Text(
                            "document",
                        ),
                    ],
                ),
            ],
        },
    ),
]
```
</details>


Once this is done, several (opinionated) optimizations can be performed such as

- [x] environment merging

    <details>
    <summary>Example</summary>


    The following AST
    ```
    [
        Cmd(
            Command {
                name: "begin",
                args: [
                    Required(
                        [
                            Text(
                                "document",
                            ),
                        ],
                    ),
                ],
            },
        ),
        Text(
            "Hello World",
        ),
        Cmd(
            Command {
                name: "end",
                args: [
                    Required(
                        [
                            Text(
                                "document",
                            ),
                        ],
                    ),
                ],
            },
        ),
    ]
    ```

    would turn into

    ```
    [
        Environment {
            name: "document",
            args: [
                Required(
                    [
                        Text(
                            "document",
                        ),
                    ],
                ),
            ],
            contents: [
                Text(
                    "Hello World",
                ),
            ],
        },
    ]
    ```
    </details>

- [ ] command substitution (not yet implemented)

    <details>
    <summary>Example</summary>

    For the following file
    ```tex
    \def\R{\mathbb R}

    \R
    ```

    All instances of `\R` in the AST would be replaced with `{\mathbb R}`.
    </details>

## Current (known) Limitations

Parsing TeX is hard. Consider the following example

```tex
% command 1
\section
{My Title}

% command 2
\medskip
{\bf unrelated}
```

Command 1 is a `section`, which takes one argument, while command 2, `medskip`,
takes none.

Currently, the grammar keeps the definitions of commands as general as possible,
but this introduces false positives. Currently, the parser will parse both
commands above as commands of one argument.

One solution to this problem might be to check all commands in the AST for the
correct number of arguments. This could be done by keeping a list of default (or
common) tex commands and their number of arguments, and building such rules
dynamically for new commands defined in the file.

Currently, `tex_parser` does not do this.

## Trying it

Despite limitations described above, `tex_parser` still works surprisingly well
on a large portion of tex files. You can try it yourself by running

```
cargo run --example text
```

## Found a Bug?

If you think you've found a bug, or an example of incorrectly parsed tex, open
an issue and I'll be happy to look into it! Contributions and feedback are
greatly welcome.
