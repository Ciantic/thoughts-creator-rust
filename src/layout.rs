#![allow(unused_braces)]
use render::{component, html::HTML5Doctype, rsx, Render};

// macro_rules! oddstruct {
//     ($n:ident, $t:type ) => {
//         struct OddStruct {
//             $($a)+
//         }
//     };
// }

// oddstruct! {
//     foo: String
// }

#[component]
fn OgGraph<Children: Render>(title: String, description: String, children: Children) {
    rsx! {
        <>
        <meta name={"description"} content={"..."} />
        <meta name={"robots"} content={"index, follow"} />
        <link rel={"canonical"} href={"https://EXAMPLE.COM/"} />
        <meta property={"og:url"} content={"https://EXAMPLE.COM/"} />
        <meta property={"og:locale"} content={"fi_FI"} />
        <meta property={"og:type"} content={"website"} />
        <meta property={"og:title"} content={"YOUR TITLE"} />
        <meta property={"og:description"} content={"YOUR DESCRIPTION"} />
        <meta property={"og:site_name"} content={"YOUR SITE NAME"} />
        <meta property={"article:publisher"} content={"https://www.facebook.com/YOURFBPAGE"} />
        <meta property={"article:modified_time"} content={"2020-12-23T20:08:51+00:00"} />
        <meta property={"og:image"} content={"https://EXAMPLE.COM/SOMEIMAGE.JPG"} />
        <meta property={"og:image:width"} content={"1920"} />
        <meta property={"og:image:height"} content={"1080"} />
        <meta name={"twitter:card"} content={"summary"} />
        </>
    }
}

#[component]
fn Html<Children: Render>(title: String, children: Children) {
    rsx! { <>
       <HTML5Doctype />
       <html>
         <head>
            <title>{title}</title>
        </head>
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
        let rendered_html = html! {
            <Html title={"Main page".into()}>
                <Heading title={"Hello world!".into()} fullname={"Foo fighters".into()} />
            </Html>
        };
        println!("html?: {:?}", rendered_html);
    }
}
