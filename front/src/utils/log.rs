use dioxus::events::eval;

pub fn log(message: &str) {
    let eval = eval(
        r#"
        let msg = await dioxus.recv();
        console.log(msg);
        "#,
    );
    eval.send(message.into()).unwrap();
}
