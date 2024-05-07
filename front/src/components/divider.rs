use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DividerProps {
    class: String,
}

pub fn Divider(props: DividerProps) -> Element {
    let base_class = "border-1 border-slate-500 w-dvw";
    let a = props.class;
    let class = format!("{base_class} {a}");
    rsx! { hr { class } }
}
