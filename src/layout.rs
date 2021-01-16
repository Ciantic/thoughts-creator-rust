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

fn iso8601(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.format("%FT%T%z").to_string()
}

#[derive(Debug)]
enum OgType {
    Website,
    Article {
        author: String,
        published: chrono::DateTime<chrono::Utc>,
        modified: chrono::DateTime<chrono::Utc>,
    },
}

#[derive(Debug)]
struct Og {
    // Required
    title: String,
    image: url::Url,
    url: url::Url,

    // Optional, but not for my implementation
    description: String,
    site_name: String,
    locale: String,

    ogtype: OgType,
}

#[component]
fn OgHead(og: Og) {
    rsx! {
        <>
            <meta name={"description"} content={og.description.clone()} />
            <link rel={"canonical"} href={og.url.to_string()} />
            <meta property={"og:url"} content={og.url.to_string()} />
            <meta property={"og:title"} content={og.title} />
            <meta property={"og:image"} content={og.image.to_string()} />
            // <meta property={"og:image:width"} content={image.1.to_string()} />
            // <meta property={"og:image:height"} content={image.2.to_string()} />

            <meta property={"og:locale"} content={og.locale} />
            <meta property={"og:description"} content={og.description} />
            <meta property={"og:site_name"} content={og.site_name} />

            {match og.ogtype {
                OgType::Website => rsx! {
                    <>
                        <meta property={"og:type"} content={"website"} />
                        // TODO: How can I get rid of these?
                        // Render-rs seems to assume each same typed rsx! block must have same amount of childs...
                        <meta />
                        <meta />
                        <meta />
                    </>
                },
                OgType::Article { author, published, modified } => rsx! {
                    <>
                        <meta property={"og:type"} content={"article"} />
                        <meta property={"article:author"} content={author} />
                        <meta property={"article:published_time"} content={iso8601(published)} />
                        <meta property={"article:modified_time"} content={iso8601(modified)} />
                        // <meta property={"article:publisher"} content={"https://www.facebook.com/YOURFBPAGE"} />
                    </>
                },
            }}
        </>
    }
}
///
///
/// og:article: https://ogp.me/#type_article
#[component]
fn OgGraph<'a, 'b, 'c, 'd, 'e>(
    title: &'a str,
    description: &'b str,
    image: (&'c str, u32, u32),
    canonical_url: &'d str,
    og_type: &'e str,
    article_published: chrono::DateTime<chrono::Utc>,
    article_modified: chrono::DateTime<chrono::Utc>,
    site_name: &'static str,
    locale: &'static str,
) {
    rsx! {
        <>
            <meta name={"description"} content={description} />
            <meta name={"robots"} content={"index, follow"} />
            <link rel={"canonical"} href={canonical_url} />
            <meta property={"og:url"} content={canonical_url} />
            <meta property={"og:locale"} content={locale} />
            <meta property={"og:title"} content={title} />
            <meta property={"og:description"} content={description} />
            <meta property={"og:site_name"} content={site_name} />

            <meta property={"og:image"} content={image.0} />
            <meta property={"og:image:width"} content={image.1.to_string()} />
            <meta property={"og:image:height"} content={image.2.to_string()} />

            // Type specifics:
            <meta property={"og:type"} content={og_type} />
            <meta property={"article:published_time"} content={iso8601(article_published)} />
            <meta property={"article:modified_time"} content={iso8601(article_modified)} />
            // <meta property={"article:publisher"} content={"https://www.facebook.com/YOURFBPAGE"} />
            // <meta property={"article:author"} content={"John Doe"} />

            // <meta name={"twitter:card"} content={"summary"} />
        </>
    }
}

#[component]
fn Html<'a, 'b, Children: Render>(title: &'a str, description: &'b str, children: Children) {
    rsx! { <>
       <HTML5Doctype />
       <html>
         <head>
            <title>{title}</title>
            <meta name={"robots"} content={"index, follow"} />
            <OgHead og={(Og {
                title: "test".into(),
                url: url::Url::parse("https://example.com/").unwrap(),
                image: url::Url::parse("https://example.com/image.jpg").unwrap(),
                description: "test".into(),
                site_name: "My Thoughts".into(),
                locale: "en_US".into(),
                ogtype: OgType::Website,
            })} />
            // <OgGraph
            //     title={title}
            //     description={description}
            //     image={("https://example.com/foo.jpg", 1600, 900)}
            //     site_name={"My Thoughts"}
            //     canonical_url={"https://example.com"}
            //     article_published={chrono::Utc::now()}
            //     article_modified={chrono::Utc::now()}
            //     locale={"en_GB"}
            //     og_type={"article"}
            //     />
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
            <Html title={"Main page"} description={"Foolio!"}>
                <Heading title={"Hello world!".into()} fullname={"Foo fighters".into()} />
            </Html>
        };
        println!("html?: {:?}", rendered_html);
    }
}
