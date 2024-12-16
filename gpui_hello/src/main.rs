use gpui::*;

/** GWAPO SI CARLO */

struct HelloWorld {
    text: SharedString,
}
{"type":"excalidraw/clipboard","elements":[{"id":"r_WdFqNdXUJme9FvDUTqC","type":"text","x":1741.2389066902394,"y":5101.2554338087775,"width":230.6159337759018,"height":45,"angle":0,"strokeColor":"#f08c00","backgroundColor":"transparent","fillStyle":"hachure","strokeWidth":2,"strokeStyle":"solid","roughness":1,"opacity":100,"groupIds":[],"frameId":null,"index":"b1R","roundness":null,"seed":28945789,"version":121,"versionNonce":1060216019,"isDeleted":false,"boundElements":[],"updated":1733673755694,"link":null,"locked":false,"text":"super flexible","fontSize":36,"fontFamily":5,"textAlign":"left","verticalAlign":"top","containerId":null,"originalText":"super flexible","autoResize":true,"lineHeight":1.25}],"files":{}}
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(Length::Definite(Pixels(500.0).into()))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .border_4()
                    .border_color(rgb(0x0000ff))
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| HelloWorld {
                    text: "Carlo!".into(),
                })
            },
        )
        .unwrap();
    });
}

// SO true
