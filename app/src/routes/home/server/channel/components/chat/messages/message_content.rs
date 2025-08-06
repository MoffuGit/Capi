use leptos::prelude::*;
use common::convex::ChannelMessage;

use crate::components::ui::markdown::{Markdown, MarkdownParser};
use crate::routes::home::server::channel::components::chat::messages::message_attachments::MessageAttachments;

#[component]
pub fn MessageContent(msg: ChannelMessage) -> impl IntoView {
    let markdown = MarkdownParser::new(&msg.content).parse_tree();

    view! {
        <Markdown markdown=markdown.into() />
        <MessageAttachments attachments=msg.attachments />
    }
}
