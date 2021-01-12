#![allow(unused_braces)]
use render::{component, html::HTML5Doctype, rsx, Render};

#[component]
fn Html<Children: Render>(title: String, children: Children) {
    rsx! { <>
       <HTML5Doctype />
       <html>
         <head><title>{title}</title></head>
         <body>
           {children}
         </body>
       </html>
    </> }
}

#[component]
fn Heading(title: String, fullname: String) {
    rsx! { <h1 class={"title"}>{title}{" "}{fullname}</h1> }
}

#[cfg(test)]
mod test_layout {
    use super::{Heading, Html};
    use render::html;

    #[test]
    fn test() {
        let list = vec!["Mouse", "Rat", "Hamster"];
        let rendered_html = html! {
            <Html title={"Main page".into()}>
                <Heading title={"Hello world!".into()} fullname={"Foo fighters".into()} />
            </Html>
        };
        println!("html?: {:?}", rendered_html);
    }
}
