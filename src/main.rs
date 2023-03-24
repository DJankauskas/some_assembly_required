#[macro_use]
extern crate lalrpop_util;

mod ast;
mod simulator;
lalrpop_mod!(pub parser);

use avalanche::{component, state, store, tracked, View};
use avalanche_web::components::{Button, Div, Text, TextArea, H1};

#[component]
pub fn App() -> View {
    let (code, set_code) = state(self, || String::new());
    let (simulator, update_simulator) = state(self, || simulator::Simulator::new(&[]));
    let (output, set_output) = store(self, |_| Vec::new());
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
                .then(|| {
                    Div(self, style="border: 1px solid black;", [
                    Text(self, tracked!(simulator).output.join("\n"))
                    ])
                })
                .into(),
        ],
    )
}

fn main() {
    avalanche_web::mount_to_body::<App>();
}
