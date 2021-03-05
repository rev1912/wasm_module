use wasm_bindgen::prelude::*;
use mogwai::prelude::*;

#[derive(Clone)]
enum In {
    Click
}

#[derive(Clone)]
enum Out {
    DrawClicks(i32)
}

#[wasm_bindgen]
pub struct App {
    num_clicks: i32
}

#[wasm_bindgen]
impl Component for App {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        builder! {
            <button on:click=tx.contra_map(|_| In::Click)>
                {(
                    "clicks = 0",
                    rx.branch_map(|msg| match msg {
                        Out::DrawClicks(n) => {
                            format!("clicks = {}", n)
                        }
                    })
                )}
            </button>
        }
    }

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        match msg {
            In::Click => {
                self.num_clicks += 1;
                tx_view.send(&Out::DrawClicks(self.num_clicks));
            }
        }
    }
}

#[wasm_bindgen]
pub fn main(struc: App) -> Result<(), JsValue> {
    let app = Gizmo::from(struc);

    if cfg!(target_arch = "wasm32") {
        View::from(app).run()
    } else {
        Ok(())
    }
}
