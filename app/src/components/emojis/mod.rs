use capi_primitives::tooltip::ToolTipSide;
use capi_virtual::{use_virtualizer, Virtualizer};
use emojis::{Emoji, Group};
use icons::{
    IconCircleCheck, IconClock, IconFlag, IconLamp, IconLeafe, IconPersonStanding, IconPlane,
    IconPopCorn, IconSearch, IconSmile, IconWavesLadder,
};
use leptos::html::Div;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;
use tailwind_fuse::tw_merge;

use crate::components::ui::button::*;
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::tooltip::*;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    emojis: Vec<&'static Emoji>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            emojis: Vec::new(),
        }
    }
}

struct EmojiTrie {
    root: TrieNode,
}

impl EmojiTrie {
    fn new() -> Self {
        EmojiTrie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, emoji: &'static Emoji) {
        let mut current = &mut self.root;
        for ch in emoji.name().to_lowercase().chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }
        current.emojis.push(emoji);
    }

    fn collect_all_emojis_in_subtree(node: &TrieNode, result: &mut Vec<&'static Emoji>) {
        result.extend_from_slice(&node.emojis);
        for child_node in node.children.values() {
            Self::collect_all_emojis_in_subtree(child_node, result);
        }
    }

    fn search_prefix(&self, prefix: &str) -> Vec<&'static Emoji> {
        let mut current = &self.root;
        for ch in prefix.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return Vec::new();
            }
        }
        let mut result = Vec::new();
        Self::collect_all_emojis_in_subtree(current, &mut result);
        result
    }
}

static EMOJI_TRIE: LazyLock<EmojiTrie> = LazyLock::new(|| {
    let mut trie = EmojiTrie::new();
    for emoji in emojis::iter() {
        trie.insert(emoji);
    }
    trie
});

#[derive(Debug, Clone, PartialEq)]
enum EmojiItem {
    Group(Group),
    EmojiRow(Vec<&'static Emoji>),
    HistoryHeader,
    HistoryRow(Vec<&'static Emoji>),
}

impl EmojiItem {
    fn from_group(group: Group) -> Self {
        EmojiItem::Group(group)
    }

    fn from_emoji_row(emojis: Vec<&'static Emoji>) -> Self {
        EmojiItem::EmojiRow(emojis)
    }
}

fn group_to_display_name(group: Group) -> &'static str {
    match group {
        Group::SmileysAndEmotion => "Smileys & Emotion",
        Group::PeopleAndBody => "People & Body",
        Group::AnimalsAndNature => "Animals & Nature",
        Group::FoodAndDrink => "Food & Drink",
        Group::TravelAndPlaces => "Travel & Places",
        Group::Activities => "Activities",
        Group::Objects => "Objects",
        Group::Symbols => "Symbols",
        Group::Flags => "Flags",
    }
}

const EMOJIS_PER_ROW: usize = 12;

#[component]
pub fn EmojiSelector(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional)] on_select_emoji: Option<Callback<&'static Emoji>>,
    #[prop(optional, into)] history: MaybeProp<Vec<String>>,
) -> impl IntoView {
    let (groups, set_groups) = signal(vec![]);
    let (emojis, set_emojis) = signal({
        let mut items: Vec<EmojiItem> = Vec::new();

        if let Some(history) = history.get() {
            if !history.is_empty() {
                items.push(EmojiItem::HistoryHeader);
                let mut current_emoji_chunk: Vec<&'static Emoji> = Vec::new();
                for emoji in history {
                    if let Some(emoji) = emojis::get(&emoji) {
                        current_emoji_chunk.push(emoji);
                        if current_emoji_chunk.len() == EMOJIS_PER_ROW {
                            items.push(EmojiItem::HistoryRow(current_emoji_chunk));
                            current_emoji_chunk = Vec::new();
                        }
                    }
                }
                if !current_emoji_chunk.is_empty() {
                    items.push(EmojiItem::HistoryRow(current_emoji_chunk));
                }
            }
        }

        for group in Group::iter() {
            set_groups.update(|groups| groups.push(group));
            items.push(EmojiItem::from_group(group));

            let mut current_emoji_chunk: Vec<&'static Emoji> = Vec::new();
            for emoji in group.emojis() {
                current_emoji_chunk.push(emoji);

                if current_emoji_chunk.len() == EMOJIS_PER_ROW {
                    items.push(EmojiItem::from_emoji_row(current_emoji_chunk));
                    current_emoji_chunk = Vec::new();
                }
            }
            if !current_emoji_chunk.is_empty() {
                items.push(EmojiItem::from_emoji_row(current_emoji_chunk));
            }
        }
        items
    });

    let (search, set_search) = signal(None::<String>);

    let scroll_ref: NodeRef<Div> = NodeRef::new();

    let filtered_emojis = Signal::derive(move || {
        let all_emojis_items = emojis.get();
        let search_term_option = search.get();

        if let Some(search_term) = search_term_option {
            let search_term_lower = search_term.to_lowercase();
            if search_term_lower.is_empty() {
                return all_emojis_items;
            }

            let matched_emojis_from_trie = EMOJI_TRIE.search_prefix(&search_term_lower);

            let mut filtered_items: Vec<EmojiItem> = Vec::new();
            let mut group_to_matched_emojis: HashMap<Group, Vec<&'static Emoji>> = HashMap::new();

            for emoji in matched_emojis_from_trie {
                group_to_matched_emojis
                    .entry(emoji.group())
                    .or_default()
                    .push(emoji);
            }

            for group in Group::iter() {
                if let Some(matched_emojis) = group_to_matched_emojis.remove(&group) {
                    if !matched_emojis.is_empty() {
                        filtered_items.push(EmojiItem::from_group(group));
                        let mut current_emoji_chunk: Vec<&'static Emoji> = Vec::new();
                        for emoji in matched_emojis {
                            current_emoji_chunk.push(emoji);
                            if current_emoji_chunk.len() == EMOJIS_PER_ROW {
                                filtered_items.push(EmojiItem::from_emoji_row(current_emoji_chunk));
                                current_emoji_chunk = Vec::new();
                            }
                        }
                        if !current_emoji_chunk.is_empty() {
                            filtered_items.push(EmojiItem::from_emoji_row(current_emoji_chunk));
                        }
                    }
                }
            }
            filtered_items
        } else {
            all_emojis_items
        }
    });

    let data_size = Signal::derive(move || filtered_emojis.get().len());
    let estimate_size = move |index: usize| {
        filtered_emojis.with(|items| match items.get(index) {
            Some(EmojiItem::Group(_)) | Some(EmojiItem::HistoryHeader) => 14.0,
            Some(EmojiItem::EmojiRow(_)) | Some(EmojiItem::HistoryRow(_)) => 32.0,
            None => 0.0,
        })
    };

    let key = move |index: usize| {
        filtered_emojis.with(|items| match items.get(index) {
            Some(EmojiItem::Group(group)) => group_to_display_name(*group).to_string(),
            Some(EmojiItem::EmojiRow(emojis)) => {
                emojis.iter().map(|e| e.as_str()).collect::<String>()
            }
            Some(EmojiItem::HistoryHeader) => "History".to_string(),
            Some(EmojiItem::HistoryRow(emojis)) => {
                emojis.iter().map(|e| e.as_str()).collect::<String>() + "_history"
            }
            None => index.to_string(),
        })
    };

    let virtualizer = use_virtualizer(data_size, scroll_ref, estimate_size, key, 5);

    let virtual_items = virtualizer.virtual_items;
    let total_height = virtualizer.total_height;

    let merged_class = Signal::derive(move || {
        tw_merge!(
            "overflow-auto w-full h-52 relative scrollbar-thin scrollbar-track-background",
            class.get()
        )
    });

    view! {
        <div class="flex flex-col items-center w-[445px] overflow-hidden">
            <div class="p-1 w-full relative flex items-center">
                <div class="size-8 absolute left-1 flex items-center justify-center gap-2">
                    <IconSearch class="size-4"/>
                </div>
                <Input class="pl-8" {..} value=search on:input=move |evt| {
                    set_search(Some(event_target_value(&evt)));
                } />
            </div>
            <div
                node_ref=scroll_ref
                class=merged_class
                style=("--total-height", move || format!("{}px", total_height.get()))
            >
                <div
                    class="relative w-full"
                    style=("height", "var(--total-height)")
                >
                    <For
                        each=move || virtual_items.get()
                        key=|virtual_item| format!("{} {}", virtual_item.key, virtual_item.start)
                        children=move |virtual_item| {
                            let item_data = move || {
                                filtered_emojis.with(|items| {
                                    items.get(virtual_item.index)
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            #[cfg(debug_assertions)]
                                            leptos::logging::error!(
                                                "Virtual item index out of bounds: index={}, items_len={}",
                                                virtual_item.index,
                                                items.len()
                                            );
                                            // Fallback to a valid item in debug, or Panic in release
                                            EmojiItem::Group(Group::SmileysAndEmotion)
                                        })
                                })
                            };
                            let top_style = format!("{}px", virtual_item.start);
                            view! {
                                <div
                                    class="absolute w-fit top-0 left-0"
                                    style=("transform", format!("translateY({top_style})"))
                                    style=("height", format!("{}px", virtual_item.size))
                                >
                                    {move || match item_data() {
                                        EmojiItem::Group(group) => view! {
                                            <Label class="h-auto w-full text-xs text-muted-foreground">
                                                {group_to_display_name(group)}
                                            </Label>
                                        }.into_any(),
                                        EmojiItem::HistoryHeader => view! {
                                            <Label class="h-auto w-full text-xs text-muted-foreground">
                                                {"History"}
                                            </Label>
                                        }.into_any(),
                                        EmojiItem::EmojiRow(emojis) | EmojiItem::HistoryRow(emojis) => view! {
                                            <div class="h-8 flex justify-center items-center gap-1">
                                                {
                                                    emojis.into_iter().map(|emoji| {
                                                        let emoji = StoredValue::new_local(emoji);
                                                        view! {
                                                            <ToolTip>
                                                                <ToolTipTrigger>
                                                                    <Button
                                                                        class="text-xl"
                                                                        size=ButtonSizes::Icon
                                                                        variant=ButtonVariants::Ghost
                                                                        on:click=move |_| {
                                                                            if let Some(callback) = on_select_emoji {
                                                                                callback.run(emoji.get_value());
                                                                            }
                                                                        }
                                                                    >
                                                                        {emoji.get_value().to_string()}
                                                                    </Button>
                                                                </ToolTipTrigger>
                                                                <ToolTipContent side=ToolTipSide::Bottom>
                                                                    {emoji.get_value().name()}
                                                                </ToolTipContent>
                                                            </ToolTip>
                                                        }
                                                    }).collect_view()
                                                }
                                            </div>
                                        }.into_any(),
                                    }}
                                </div>
                            }
                        }
                    />
                </div>
            </div>
            <GroupScroller virtualizer=virtualizer group_keys=groups />
        </div>
    }
}

#[component]
pub fn GroupScroller(
    virtualizer: Virtualizer,
    #[prop(into)] group_keys: Signal<Vec<Group>>,
) -> impl IntoView {
    view! {
        <div class="flex overflow-x-auto p-1 gap-1 border-t border-t-border w-full items-start">
            <ToolTip>
                <ToolTipTrigger>
                    <Button
                        class="text-muted-foreground"
                        size=ButtonSizes::Icon
                        variant=ButtonVariants::Ghost
                        on:click=move |_| virtualizer.scroll_to_key("History")
                    >
                        <IconClock/>
                    </Button>
                </ToolTipTrigger>
                <ToolTipContent side=ToolTipSide::Bottom>
                    {"History"}
                </ToolTipContent>
            </ToolTip>
            <For
                each=move || group_keys.get().into_iter()
                key=|group| *group
                children=move |group| {
                    view! {
                        <ToolTip>
                            <ToolTipTrigger>
                                <Button
                                    class="text-muted-foreground"
                                    size=ButtonSizes::Icon
                                    variant=ButtonVariants::Ghost
                                    on:click=move |_| virtualizer.scroll_to_key(group_to_display_name(group))
                                >
                                    {
                                        match group {
                                            Group::SmileysAndEmotion => view!{<IconSmile/>}.into_any(),
                                            Group::PeopleAndBody => view!{<IconPersonStanding/>}.into_any(),
                                            Group::AnimalsAndNature => view!{<IconLeafe/>}.into_any(),
                                            Group::FoodAndDrink => view!{<IconPopCorn/>}.into_any(),
                                            Group::TravelAndPlaces => view!{<IconPlane/>}.into_any(),
                                            Group::Activities => view!{<IconWavesLadder/>}.into_any(),
                                            Group::Objects => view!{<IconLamp/>}.into_any(),
                                            Group::Symbols => view!{<IconCircleCheck/>}.into_any(),
                                            Group::Flags => view!{<IconFlag/>}.into_any(),
                                        }
                                    }
                                </Button>
                            </ToolTipTrigger>
                            <ToolTipContent
                                side=ToolTipSide::Bottom
                            >
                                {group_to_display_name(group)}
                            </ToolTipContent>
                        </ToolTip>
                    }
                }
            />
        </div>
    }
}
