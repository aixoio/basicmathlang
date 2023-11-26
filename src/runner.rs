use std::{collections::HashMap, process};

use crate::parser::ParsedTokens;


pub fn run(tokens: &Vec<ParsedTokens>) {
    let mut vars: HashMap<String, f64> = HashMap::new();

    let mut last_val = 0f64;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            ParsedTokens::Add => {
                let last = tokens.get(index - 1);
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                if let None = last {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let last = last.expect("Error");
                let next = next.expect("Error");

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                last_val = last_v + next_v;

            },
            ParsedTokens::Sub => {
                let last = tokens.get(index - 1);
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                if let None = last {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let last = last.expect("Error");
                let next = next.expect("Error");

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                last_val = last_v - next_v;

            },
            ParsedTokens::Div => {
                let last = tokens.get(index - 1);
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                if let None = last {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let last = last.expect("Error");
                let next = next.expect("Error");

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                last_val = last_v / next_v;

            },
            ParsedTokens::Mpl => {
                let last = tokens.get(index - 1);
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                if let None = last {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let last = last.expect("Error");
                let next = next.expect("Error");

                let last_v;
                let next_v;

                match last {
                    ParsedTokens::Number(val) => last_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                last_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                match next {
                    ParsedTokens::Number(val) => next_v = val,
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                next_v = val;
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

                last_val = last_v * next_v;

            },
            ParsedTokens::Equal => {
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let next = next.expect("Error");

                match next {
                    ParsedTokens::Var(var) => {
                        vars.insert(var.clone(), last_val);
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }

            }
            ParsedTokens::Print => {
                let next = tokens.get(index + 1);

                if let None = next {
                    eprintln!("Syntax error");
                    process::exit(1);
                }

                let next = next.expect("Error");

                match next {
                    ParsedTokens::Var(var) => {
                        match vars.get(var) {
                            Some(val) => {
                                println!("{}: {}", var, val);
                            },
                            None => {
                                eprintln!("Syntax error");
                                process::exit(1);
                            }
                        }
                    },
                    _ => {
                        eprintln!("Syntax error");
                        process::exit(1);
                    },
                }
            },
            _ => {},
        }
    }
}
