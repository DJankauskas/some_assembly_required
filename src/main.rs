#[macro_use]
extern crate lalrpop_util;

mod ast;
mod simulator;
lalrpop_mod!(pub parser);

use ast::Instruction;
use avalanche::{component, state, tracked, View};
use avalanche_web::components::{Button, Div, Text, TextArea, H1};

const USER_CODE_LEN: usize = 100;
const PROVIDED_CODE_LEN: usize = 1024 - USER_CODE_LEN;

#[component]
pub fn App() -> View {
    let (code, set_code) = state(self, || String::new());
    let (provided_code, set_provided_code) = state(self, || String::new());
    let (_simulator, update_simulator) = state(self, || simulator::Simulator::new(&[]));
    let (output, set_output) = state(self, || Vec::new());
    Div(
        self,
        [
            H1(self, [Text(self, "Simulator")]),
            Div(
                self,
                style = "display: flex; flex-direction: row; justify-content: space-between; flex-wrap: wrap;",
                [
                    CodeEditor(
                        self,
                        code = tracked!(code),
                        set_code = &|code| set_code.set(code),
                        label = "User Code",
                    ),
                    CodeEditor(
                        self,
                        code = tracked!(provided_code),
                        set_code = &|code| set_provided_code.set(code),
                        label = "Provided Code",
                    ),
                ],
            ),
            Button(
                self,
                on_click = |_| {
                    let mut output = Vec::new();

                    let user_code = match process_code(tracked!(code), USER_CODE_LEN, "user") {
                        Ok((code, len_msg)) => {
                            if let Some(len_msg) = len_msg {
                                output.push(len_msg);
                            }
                            code
                        }
                        Err(err) => {
                            output.push(err);
                            set_output.set(output);
                            return;
                        }
                    };

                    let provided_code = match process_code(
                        tracked!(provided_code),
                        PROVIDED_CODE_LEN,
                        "provided",
                    ) {
                        Ok((code, len_msg)) => {
                            if let Some(len_msg) = len_msg {
                                output.push(len_msg);
                            }
                            code
                        }
                        Err(err) => {
                            output.push(err);
                            set_output.set(output);
                            return;
                        }
                    };

                    let mut program = tracked!(user_code);
                    tracked!(&mut program).extend(tracked!(provided_code));

                    let mut simulator = simulator::Simulator::new(&tracked!(program));
                    tracked!(&mut simulator).run();
                    output.extend(tracked!(&simulator).output.clone());
                    set_output.set(output);
                    update_simulator.set(tracked!(simulator));
                },
                [Text(self, "Run")],
            ),
            (!tracked!(output).is_empty())
                .then(|| Output(self, tracked!(output)))
                .into(),
        ],
    )
}

#[component]
fn Output(output: &[String]) -> View {
    Div(
        self,
        [
            Text(self, "Output:"),
            Div(
                self,
                style = "border: 1px solid black; font-family: monospace;",
                (0..tracked!(output).len())
                    .map(|i| Div(self, key = i, [Text(self, key = i, &tracked!(output)[i])]))
                    .collect::<Vec<_>>(),
            ),
        ],
    )
}

#[component]
fn CodeEditor(code: &str, set_code: &dyn Fn(String), label: &str) -> View {
    Div(
        self,
        style = "display: flex; flex-direction: column;",
        [
            Div(self, [Text(self, tracked!(label))]),
            TextArea(
                self,
                style = "margin-right: 5px;",
                rows = 40u32,
                cols = 80u32,
                value = tracked!(code),
                on_input = |e| tracked!(set_code)(e.current_target().unwrap().value()),
            ),
        ],
    )
}

fn main() {
    avalanche_web::mount_to_body::<App>();
}

fn process_code(
    code: &str,
    len: usize,
    code_name: &str,
) -> Result<(Vec<ast::Instruction>, Option<String>), String> {
    let program = parser::ProgramParser::new().parse(code);
    let mut program = match program {
        Ok(program) => program,
        Err(err) => {
            return Err(format!(
                "ERROR: failed to parse instructions. Sytax error: {}",
                err.map_location(|loc| { ast::get_line_col(code, loc) })
            ));
        }
    };

    // Let user know if code is too long.
    let len_msg = if program.len() > len {
        Some(format!(
            "WARN: {} code is too long. {} instructions truncated.",
            code_name,
            program.len() - len
        ))
    } else {
        None
    };
    program.resize(len, Instruction::Nop);

    Ok((program, len_msg))
}
