use common::convex::ChannelMessage;
use leptos::prelude::*;
use markdown::Markdown;

// use crate::components::ui::markdown::{Markdown, MarkdownParser};
use crate::routes::home::server::channel::components::chat::messages::message_attachments::MessageAttachments;

#[component]
pub fn MessageContent(msg: ChannelMessage) -> impl IntoView {
    // let markdown = MarkdownParser::new(&msg.content).parse_tree();

    view! {
        <Markdown source=msg.content class="prose prose-stone prose-sm dark:prose-invert" />
        <MessageAttachments attachments=msg.attachments />
    }
}
