use crate::{prelude::*, Icon, IconButton, Tab};
use gpui::prelude::*;

#[derive(Component)]
pub struct TabBar {
    id: ElementId,
    /// Backwards, Forwards
    can_navigate: (bool, bool),
    tabs: Vec<Tab>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>, tabs: Vec<Tab>) -> Self {
        Self {
            id: id.into(),
            can_navigate: (false, false),
            tabs,
        }
    }

    pub fn can_navigate(mut self, can_navigate: (bool, bool)) -> Self {
        self.can_navigate = can_navigate;
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let (can_navigate_back, can_navigate_forward) = self.can_navigate;

        div()
            .group("tab_bar")
            .id(self.id.clone())
            .w_full()
            .flex()
            .bg(cx.theme().colors().tab_bar_background)
            // Left Side
            .child(
                div()
                    .relative()
                    .px_1()
                    .flex()
                    .flex_none()
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .right_0()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(
                                IconButton::new("arrow_left", Icon::ArrowLeft)
                                    .state(InteractionState::Enabled.if_enabled(can_navigate_back)),
                            )
                            .child(
                                IconButton::new("arrow_right", Icon::ArrowRight).state(
                                    InteractionState::Enabled.if_enabled(can_navigate_forward),
                                ),
                            ),
                    ),
            )
            .child(
                div().w_0().flex_1().h_full().child(
                    div()
                        .id("tabs")
                        .flex()
                        .overflow_x_scroll()
                        .children(self.tabs.clone()),
                ),
            )
            // Right Side
            .child(
                div()
                    // We only use absolute here since we don't
                    // have opacity or `hidden()` yet
                    .absolute()
                    .neg_top_7()
                    .px_1()
                    .flex()
                    .flex_none()
                    .gap_2()
                    .group_hover("tab_bar", |this| this.top_0())
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(IconButton::new("plus", Icon::Plus))
                            .child(IconButton::new("split", Icon::Split)),
                    ),
            )
    }
}

use gpui::ElementId;
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Node, Render};

    pub struct TabBarStory;

    impl Render for TabBarStory {
        type Element = Node<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, TabBar>(cx))
                .child(Story::label(cx, "Default"))
                .child(TabBar::new(
                    "tab-bar",
                    vec![
                        Tab::new(1)
                            .title("Cargo.toml".to_string())
                            .current(false)
                            .git_status(GitStatus::Modified),
                        Tab::new(2)
                            .title("Channels Panel".to_string())
                            .current(false),
                        Tab::new(3)
                            .title("channels_panel.rs".to_string())
                            .current(true)
                            .git_status(GitStatus::Modified),
                        Tab::new(4)
                            .title("workspace.rs".to_string())
                            .current(false)
                            .git_status(GitStatus::Modified),
                        Tab::new(5)
                            .title("icon_button.rs".to_string())
                            .current(false),
                        Tab::new(6)
                            .title("storybook.rs".to_string())
                            .current(false)
                            .git_status(GitStatus::Created),
                        Tab::new(7).title("theme.rs".to_string()).current(false),
                        Tab::new(8)
                            .title("theme_registry.rs".to_string())
                            .current(false),
                        Tab::new(9)
                            .title("styleable_helpers.rs".to_string())
                            .current(false),
                    ],
                ))
        }
    }
}
