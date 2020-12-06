#[derive(Debug, Clone, Eq, PartialEq, Properties)]
struct RawHTMLProps {
    pub inner_html: String,
}

struct RawHTML {
    props: RawHTMLProps,
}

impl Component for RawHTML {
    type Message = Msg;
    type Properties = RawHTMLProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.props.inner_html[..]);

        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }
}