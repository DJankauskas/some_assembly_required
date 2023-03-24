#[macro_use]
extern crate lalrpop_util;

mod ast;
mod simulator;
lalrpop_mod!(pub parser);

use avalanche::{component, state, tracked, View};
use avalanche_web::components::{Button, Div, Text, TextArea, H1};

#[component]
pub fn App() -> View {
    let (code, set_code) = state(self, || String::new());
    let (simulator, update_simulator) = state(self, || simulator::Simulator::new(&[]));
    let (output, set_output) = state(self, || Vec::new());
    Div(
        self,
        [
            H1(self, [Text(self, "Simulator")]),
            TextArea(
                self,
                rows = 40u32,
                cols = 80u32,
                value = tracked!(code),
                on_input = |e| set_code.set(e.current_target().unwrap().value()),
            ),
            Button(
                self,
                on_click = |_| {
                    let program = parser::ProgramParser::new().parse(&tracked!(code));
                    let program = match tracked!(program) {
                        Ok(program) => program,
                        Err(err) => {
                            set_output.set(vec![format!("Parse error: {}", err)]);
                            return;
                        }
                    };
                    let mut simulator = simulator::Simulator::new(&tracked!(program));
                    tracked!(&mut simulator).run();
                    set_output.set(tracked!(&simulator).output.clone());
                    update_simulator.set(tracked!(simulator));
                },
                [Text(self, "Run")],
            ),
            (!tracked!(simulator).output.is_empty())
                .then(|| Output(self, tracked!(output)))
                .into(),
        ],
    )
}

#[component]
pub fn Output(output: &[String]) -> View {
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

fn main() {
    avalanche_web::mount_to_body::<App>();
}
