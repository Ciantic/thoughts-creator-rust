use pulldown_cmark::{escape, html, CodeBlockKind, Event, LinkType, Options, Parser, Tag};

struct EventIter<'a> {
    p: Parser<'a>,
}

impl<'a> EventIter<'a> {
    pub fn new(p: Parser<'a>) -> Self {
        EventIter { p }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.p.next()?;

        if let Event::Start(Tag::Image(LinkType::Inline, src, title)) = &next {
            // TODO: Determine image size with `imagesize` crate
            // TODO: There is no proper escaping library used here:
            let imgtag = format!(
                "<img src=\"{}\" title=\"{}\" width=\"\" height=\"\" />",
                src.replace("\"", "&quot;"),
                title.replace("\"", "&quot;")
            );
            return Some(Event::Html(imgtag.into()));
        }

        // match &next {
        //     Event::Start(Tag::Image(l, n, d)) => {
        //         println!("image {:?} {} {}", l, n, d);
        //         return Some(Event::Html("IMG TAG HERE".into()));
        //     },
        //     _ => ()
        // }

        // if let &Event::Start(Tag::Image(t, _, _)) = &next {

        //     return Some(next);
        // }

        // if let &Event::Start(Tag::CodeBlock(_)) = &next {
        // 	// Codeblock time!
        // 	let mut text_buf = String::new();
        // 	let mut next = self.p.next();
        // 	loop {
        // 		if let Some(Event::Text(ref s)) = next {
        // 			text_buf += s;
        // 		} else {
        // 			break;
        // 		}
        // 		next = self.p.next();
        // 	}
        // 	// let mut fmt = SyntectFormatter::new();
        // 	match &next {
        // 		Some(Event::End(Tag::CodeBlock(cb))) => {
        //             println!("Foo");
        // 			if let CodeBlockKind::Fenced(ref token) = cb {
        // 				// fmt = fmt.token(token);
        // 				println!("TTTTTTTTTTTTTTTTTTTTTTTTT {}", token);
        // 			}
        // 		},
        // 		_ => panic!("Unexpected element inside codeblock mode {:?}", next),
        // 	}
        // 	// let formatted = fmt.highlight_snippet(&text_buf);
        //     // return Some(Event::Html(formatted.into()));
        //     return Some(Event::Html(text_buf.into()));
        // }
        Some(next)
    }
}

// fn parse() {
//     let markdown_input = "
//     # foo

//     Lorem ipsum dolor sit amet, consectetuer adipiscing elit.
//     Duis tincidunt erat in purus ullamcorper ultricies. Duis 
//     lacinia aliquet dolor. 

//     ```bash
//     # My code block

//     echo \"Foo is here\"
//     ```

//     ![](./image.png \"with some title\")
    
//     Maecenas velit enim, eleifend a, tempor eu, mattis in, nisl.
//     Maecenas ut orci. Sed egestas auctor sem. Curabitur vitae 
//     pede vel nisl tristique commodo. Phasellus ut nisl. Cras massa.
//      Suspendisse potenti. Vestibulum vitae augue. Mauris mauris sapien,
//      aliquet vitae, tincidunt ac, volutpat eu, ante. Nunc sed quam.
//     "
//     .replace("    ", "");
//     // and we therefore must enable it explicitly.
//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     options.insert(Options::ENABLE_FOOTNOTES);
//     options.insert(Options::ENABLE_TABLES);
//     let parser = Parser::new_ext(&markdown_input, options);
//     let ev_it = EventIter::new(parser);
//     let mut html_output = String::new();
//     html::push_html(&mut html_output, ev_it);

//     println!("{}", html_output);
// }


pub async fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown, options);
    let ev_it = EventIter::new(parser);
    let mut html_output = String::new();
    html::push_html(&mut html_output, ev_it);
    println!("{}", html_output);
    html_output
}
