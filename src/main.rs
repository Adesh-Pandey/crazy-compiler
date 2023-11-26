#[derive(Debug, PartialEq, Clone)]
enum Number {
    Int(i32),
    Float(f32),
}
#[derive(Debug, PartialEq, Clone)]
enum Types {
    Number(Number),
    String,
}
#[derive(Debug, PartialEq, Clone)]
enum Arithmatic {
    Add,
    Subtract,
    Multiply,
}
#[derive(Debug, PartialEq, Clone)]
enum Logical {
    OR,
    AND,
    NOT,
}
#[derive(Debug, PartialEq, Clone)]
enum Operator {
    ArithmaticOperator(Arithmatic),
    LogicalOperator(Logical),
    Assignment,
}
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Operator(Operator),
    Identifier,
    Comma,
    Constant,
}
#[derive(Debug, PartialEq, Clone)]
enum Data {
    Char(char),
    String(Box<String>),
}
fn save_current_token_buffer_and_clear(
    tokenized_list: &mut Vec<(TokenType, Data)>,
    current_token: &mut Vec<char>,
) {
    if !current_token.is_empty() {
        tokenized_list.push((
            TokenType::Identifier,
            Data::String(Box::new(current_token.clone().into_iter().collect())),
        ));

        current_token.clear()
    }
}

fn tokenize_instruction(instruction: &str) -> Vec<(TokenType, Data)> {
    let chars = instruction.chars();
    let mut current_token: Vec<char> = vec![];

    let mut tokenized_list: Vec<(TokenType, Data)> = vec![];

    for c in chars {
        match c {
            'a'..='z' | 'A'..='Z' => {
                current_token.push(c);
            }
            '0'..='9' => {
                if current_token.is_empty() {
                    tokenized_list.push((TokenType::Constant, Data::Char(c)));
                } else {
                    current_token.push(c);
                }
            }
            '+' => {
                tokenized_list.push((
                    TokenType::Operator(Operator::ArithmaticOperator(Arithmatic::Add)),
                    Data::Char('+'),
                ));
                save_current_token_buffer_and_clear(&mut tokenized_list, &mut current_token)
            }
            '=' => {
                tokenized_list.push((TokenType::Operator(Operator::Assignment), Data::Char('=')));
                save_current_token_buffer_and_clear(&mut tokenized_list, &mut current_token)
            }

            '-' => {
                tokenized_list.push((
                    TokenType::Operator(Operator::ArithmaticOperator(Arithmatic::Subtract)),
                    Data::Char('-'),
                ));
                save_current_token_buffer_and_clear(&mut tokenized_list, &mut current_token)
            }
            _ => save_current_token_buffer_and_clear(&mut tokenized_list, &mut current_token),
        }
    }
    tokenized_list
}

fn main() {
    let crazy_instructions = "variable = variable + 3";

    let tokenized = tokenize_instruction(crazy_instructions);
    println!("instructions \n {}\n", crazy_instructions);
    println!("{:?}", tokenized);
}
