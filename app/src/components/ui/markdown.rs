use std::iter::Peekable;
use std::str::FromStr;

use pulldown_cmark::{
    BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd,
};
use regex::Regex;
use uuid::Uuid;

use leptos::prelude::*;

#[component]
pub fn Markdown(markdown: Signal<MarkdownTree>) -> impl IntoView {
    view! {
        {
            move || {
                view!{
                    <MarkdownParagraph node=markdown.get().root  />
                }
            }
        }
    }
}

#[component]
pub fn MarkdownParagraph(node: MarkdownNode) -> impl IntoView {
    let childrens = node
        .childrens
        .iter()
        .map(|node| {
            view! {<MarkdownParagraph node=node.clone()  />}
        })
        .collect_view();

    match node.element {
        MarkdownElement::Paragraph => view! {<p class="leading-7">{childrens}</p>}.into_any(),
        MarkdownElement::Text(text) => view! {{text}}.into_any(),
        MarkdownElement::LineBreak => view! {<br/>}.into_any(),
        MarkdownElement::Bold => view! {<strong>{childrens}</strong>}.into_any(),
        MarkdownElement::Italic => view! {<em>{childrens}</em>}.into_any(),
        MarkdownElement::Heading(level) => match level {
            HeadingLevel::H1 => view! {<h1 class="scroll-m-20 my-2 text-4xl font-extrabold tracking-tight text-balance">{childrens}</h1>}.into_any(),
            HeadingLevel::H2 => view! {<h2 class="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">{childrens}</h2>}.into_any(),
            HeadingLevel::H3 => view! {<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight">{childrens}</h3>}.into_any(),
            HeadingLevel::H4 => view! {<h4 class="scroll-m-20 text-xl font-semibold tracking-tight">{childrens}</h4>}.into_any(),
            HeadingLevel::H5 => view! {<h5>{childrens}</h5>}.into_any(),
            HeadingLevel::H6 => view! {<h6>{childrens}</h6>}.into_any(),
        },
        MarkdownElement::Blockquotes(kind) => {
            if kind.is_some() {
                // block_kind.set(kind);
            }
            view! {<blockquote class="my-2 border-l-2 pl-6 italic">{childrens}</blockquote>}.into_any()
        }
        MarkdownElement::ListItem => view! {<li>{childrens}</li>}.into_any(),
        MarkdownElement::List { order } => {
            if order {
                view! {<ol class="my-2 ml-6 list-decimal">{childrens}</ol>}.into_any()
            } else {
                view! {<ul class="my-2 ml-6 list-disc">{childrens}</ul>}.into_any()
            }
        }
        MarkdownElement::Code(code) => view! {<code class="bg-muted relative rounded px-[0.3rem] py-[0.2rem] text-sm font-semibold font-mono">{code}</code>}.into_any(),
        MarkdownElement::CodeBlock(_lang) => view! {
            <pre class="my-2 w-fit bg-muted relative rounded px-[0.3rem] py-[0.2rem] text-sm font-semibold font-mono">
                <code>{childrens}</code>
            </pre>
        }
        .into_any(),
        MarkdownElement::Link { url } => view! {
            <a href=url class="text-primary font-medium underline underline-offset-4">{url.clone()}</a>
        }
        .into_any(),
        MarkdownElement::Role(id) => view! {
            // {
            //     move || {
            //         if let Some(mention) = role_mentions.get().iter().find(|mention| mention.id == id).cloned() {
            //             Either::Left(view!{
            //                 <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">@{mention.name}</span>
            //             })
            //         } else {
            //             Either::Right(view!{
            //                 <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">"Unknown"</span>
            //             })
            //         }
            //     }
            // }
        }
        .into_any(),
        MarkdownElement::Mention(id) => view! {
            // {
            //     move || {
            //         if let Some(mention) = mentions.get().iter().find(|mention| mention.id == id).cloned() {
            //             let name = mention.name.clone();
            //             Either::Left(
            //                 view!{
            //                     <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=mention class="cursor-pointer inline select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5" >
            //                         {format!("@{}", name)}
            //                     </MemberBanner >
            //                 }
            //             )
            //         } else {
            //             Either::Right(view!{
            //                 <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">@"Unknown"</span>
            //             })
            //         }
            //     }
            // }
        }
        .into_any(),
        MarkdownElement::Everyone => view! {
            <span>@"Everyone"</span>
        }
        .into_any(),
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownElement {
    Paragraph,
    Text(String),
    Code(String),
    Heading(HeadingLevel),
    LineBreak,
    Role(Uuid),
    Mention(Uuid),
    Bold,
    Everyone,
    Italic,
    Blockquotes(Option<BlockQuoteKind>),
    List { order: bool },
    ListItem,
    CodeBlock(Option<String>),
    Link { url: Option<String> },
}

impl TryFrom<Tag<'_>> for MarkdownElement {
    type Error = String;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        Ok(match value {
            Tag::Link {
                link_type: LinkType::Autolink,
                dest_url,
                title: _,
                id: _,
            } => MarkdownElement::Link {
                url: Some(dest_url.to_string()),
            },
            Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            } => MarkdownElement::Text(dest_url.to_string()),
            Tag::CodeBlock(kind) => {
                let lang = if let CodeBlockKind::Fenced(info) = kind {
                    let lang = info.split(' ').next().unwrap();
                    if lang.is_empty() {
                        None
                    } else {
                        Some(lang.to_string())
                    }
                } else {
                    None
                };

                MarkdownElement::CodeBlock(lang)
            }
            Tag::Emphasis => MarkdownElement::Italic,
            Tag::Strong => MarkdownElement::Bold,
            Tag::Heading { level, .. } => MarkdownElement::Heading(level),
            Tag::BlockQuote(kind) => MarkdownElement::Blockquotes(kind),
            Tag::List(order) => MarkdownElement::List {
                order: order.is_some(),
            },
            Tag::Item => MarkdownElement::ListItem,
            _ => return Err(String::from("This is not possibe right now")),
        })
    }
}

impl TryFrom<TagEnd> for MarkdownElement {
    type Error = String;

    fn try_from(value: TagEnd) -> Result<Self, Self::Error> {
        Ok(match value {
            TagEnd::Link => MarkdownElement::Link { url: None },
            TagEnd::CodeBlock => MarkdownElement::CodeBlock(None),
            TagEnd::Item => MarkdownElement::ListItem,
            TagEnd::Paragraph => MarkdownElement::Paragraph,
            TagEnd::Emphasis => MarkdownElement::Italic,
            TagEnd::Strong => MarkdownElement::Bold,
            TagEnd::BlockQuote(kind) => MarkdownElement::Blockquotes(kind),
            TagEnd::Heading(level) => MarkdownElement::Heading(level),
            TagEnd::List(order) => MarkdownElement::List { order },
            _ => return Err(String::from("This is not possible right now")),
        })
    }
}

pub struct MarkdownParser<'a> {
    parser: Peekable<Parser<'a>>,
    offset: usize,
}

impl<'a> MarkdownParser<'a> {
    pub fn new(input: &'a str) -> Self {
        MarkdownParser::new_with_offset(input, 0)
    }

    pub fn new_with_offset(input: &'a str, offset: usize) -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_GFM);
        let parser = Parser::new_ext(input, options).peekable();
        MarkdownParser { parser, offset }
    }

    pub fn parse_tree(&mut self) -> MarkdownTree {
        let root = self.parse();
        MarkdownTree::new_with_root(root)
    }

    fn parse(&mut self) -> MarkdownNode {
        let mut root = MarkdownNode {
            element: MarkdownElement::Paragraph,
            start_offset: self.offset,
            end_offset: self.offset,
            // node_ref: NodeRef::new(),
            childrens: vec![],
        };

        while let Some(event) = self.parser.next() {
            if let Some(children) = self.parse_event(event) {
                root.childrens.push(children);
            }
        }
        root.end_offset = self.offset;
        root
    }

    fn parse_text(&mut self, text: String) -> MarkdownNode {
        let mut nodes = vec![];
        let mut current_offset = self.offset;
        let start = self.offset;
        let mention_regex =
            Regex::new(r"<@(?:(?P<type>role):)?(?P<id>[0-9a-f]{32})>|<@everyone>").unwrap();
        let mut last_match_end = 0;

        for capture in mention_regex.captures_iter(&text) {
            let full_match = capture.get(0).unwrap();
            let start = full_match.start();
            let end = full_match.end();

            if start > last_match_end {
                let text_part = &text[last_match_end..start];
                nodes.push(MarkdownNode {
                    element: MarkdownElement::Text(text_part.to_string()),
                    start_offset: current_offset,
                    end_offset: current_offset + text_part.len(),
                    childrens: vec![],
                });
                current_offset += text_part.len();
            }

            let element = if full_match.as_str() == "<@everyone>" {
                MarkdownElement::Everyone
            } else if let Some(id) = capture.name("id") {
                if let Ok(id) = Uuid::from_str(id.as_str()) {
                    match capture.name("type") {
                        Some(_) => MarkdownElement::Role(id),
                        None => MarkdownElement::Mention(id),
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };

            nodes.push(MarkdownNode {
                element,
                start_offset: current_offset,
                end_offset: current_offset + end - start,
                childrens: vec![],
            });
            current_offset += end - start;
            last_match_end = end;
        }

        if last_match_end < text.len() {
            let text_part = &text[last_match_end..text.len()];
            nodes.push(MarkdownNode {
                element: MarkdownElement::Text(text_part.to_string()),
                start_offset: current_offset,
                end_offset: current_offset + text_part.len(),
                childrens: vec![],
            });
        }
        self.offset += text.len();
        if nodes.len() == 1 {
            nodes.remove(0)
        } else {
            MarkdownNode {
                element: MarkdownElement::Paragraph,
                start_offset: start,
                end_offset: current_offset,
                childrens: nodes,
            }
        }
    }

    fn parse_event(&mut self, event: Event) -> Option<MarkdownNode> {
        Some(match event {
            Event::Code(cow_str) => {
                let start = self.offset;
                self.offset += cow_str.len();
                MarkdownNode {
                    element: MarkdownElement::Code(cow_str.to_string()),
                    start_offset: start,
                    end_offset: self.offset,
                    childrens: vec![],
                    // node_ref: NodeRef::new(),
                }
            }
            Event::Start(tag) => self.parse_tag(tag)?,
            Event::SoftBreak | Event::HardBreak => MarkdownNode {
                element: MarkdownElement::LineBreak,
                start_offset: self.offset,
                end_offset: self.offset,
                childrens: vec![],
            },
            Event::Text(cow_str) => {
                let mut text = String::from(cow_str);
                while let Some(Event::Text(cow_str)) = self.parser.peek() {
                    text.push_str(cow_str);
                    self.parser.next();
                }
                self.parse_text(text)
            }
            _ => return None,
        })
    }

    fn parse_tag(&mut self, tag: Tag) -> Option<MarkdownNode> {
        let element = MarkdownElement::try_from(tag.clone()).ok()?;
        let start = self.offset;

        let mut node = MarkdownNode {
            element,
            start_offset: start,
            end_offset: start,
            childrens: vec![],
            // node_ref: NodeRef::new(),
        };

        while let Some(event) = self.parser.next() {
            match event {
                Event::End(end_tag) => {
                    if let Ok(end_el) = MarkdownElement::try_from(end_tag) {
                        if matches!(node.element, MarkdownElement::CodeBlock(..))
                            && matches!(end_el, MarkdownElement::CodeBlock(..))
                        {
                            node.end_offset = self.offset;
                            break;
                        }
                        if matches!(node.element, MarkdownElement::Link { .. })
                            && matches!(end_el, MarkdownElement::Link { .. })
                        {
                            node.end_offset = self.offset;
                            break;
                        }
                        if end_el == node.element {
                            node.end_offset = self.offset;
                            break;
                        }
                    }
                }
                _ => {
                    if let Some(child) = self.parse_event(event) {
                        if let (MarkdownElement::Text(new_text), Some(last_child)) =
                            (&child.element, node.childrens.last_mut())
                        {
                            if let MarkdownElement::Text(last_text) = &last_child.element {
                                last_child.element =
                                    MarkdownElement::Text(format!("{last_text}{new_text}"));
                                last_child.end_offset = child.end_offset;
                                continue;
                            }
                        }
                        node.childrens.push(child);
                    }
                }
            }
        }

        Some(node)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkdownNode {
    pub element: MarkdownElement,
    pub start_offset: usize,
    pub end_offset: usize,
    pub childrens: Vec<MarkdownNode>,
    // node_ref: NodeRef<Span>,
}

impl Default for MarkdownNode {
    fn default() -> Self {
        Self {
            element: MarkdownElement::Paragraph,
            start_offset: Default::default(),
            end_offset: Default::default(),
            childrens: Default::default(),
            // node_ref: Default::default(),
        }
    }
}

impl MarkdownNode {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn iter(&self) -> MarkdownNodeIterator {
        MarkdownNodeIterator { stack: vec![self] }
    }
}

pub struct MarkdownNodeIterator<'a> {
    stack: Vec<&'a MarkdownNode>,
}

impl<'a> Iterator for MarkdownNodeIterator<'a> {
    type Item = &'a MarkdownNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.childrens.iter().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkdownTree {
    pub root: MarkdownNode,
    pub offset: usize,
}

impl MarkdownTree {
    pub fn new() -> Self {
        let root = MarkdownNode::default();
        MarkdownTree { root, offset: 0 }
    }

    pub fn new_with_root(root: MarkdownNode) -> Self {
        let offset = root.end_offset;
        MarkdownTree { root, offset }
    }

    pub fn iter(&self) -> MarkdownTreeIterator {
        MarkdownTreeIterator {
            stack: vec![&self.root],
        }
    }
}

pub struct MarkdownTreeIterator<'a> {
    stack: Vec<&'a MarkdownNode>,
}

impl<'a> Iterator for MarkdownTreeIterator<'a> {
    type Item = &'a MarkdownNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.childrens.iter().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}
