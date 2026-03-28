use std::cmp::Ordering;

use gpui::{AnyElement, IntoElement, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

const START_TAB_SLOT_SIZE: Pixels = px(16.);
const END_TAB_SLOT_SIZE: Pixels = px(24.);

/// The position of a [`Tab`] within a list of tabs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabPosition {
    /// The tab is first in the list.
    First,

    /// The tab is in the middle of the list (i.e., it is not the first or last tab).
    ///
    /// The [`Ordering`] is where this tab is positioned with respect to the selected tab.
    Middle(Ordering),

    /// The tab is last in the list.
    Last,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TabCloseSide {
    Start,
    End,
}

#[derive(IntoElement, RegisterComponent)]
pub struct Tab {
    div: Stateful<Div>,
    selected: bool,
    position: TabPosition,
    close_side: TabCloseSide,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            div: div()
                .id(id.clone())
                .debug_selector(|| format!("TAB-{}", id)),
            selected: false,
            position: TabPosition::First,
            close_side: TabCloseSide::End,
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
        }
    }

    pub fn position(mut self, position: TabPosition) -> Self {
        self.position = position;
        self
    }

    pub fn close_side(mut self, close_side: TabCloseSide) -> Self {
        self.close_side = close_side;
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn content_height(cx: &App) -> Pixels {
        let _ = cx;
        px(28.)
    }

    pub fn container_height(cx: &App) -> Pixels {
        let _ = cx;
        px(40.)
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl Toggleable for Tab {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    #[allow(refining_impl_trait)]
    fn render(self, _: &mut Window, cx: &mut App) -> Stateful<Div> {
        let colors = cx.theme().colors();
        let active_tab_bg = colors
            .tab_active_background
            .blend(colors.border_selected.opacity(0.18));

        let (text_color, tab_hover_bg) = match self.selected {
            false => (colors.text_muted.opacity(0.75), colors.ghost_element_hover),
            true => (colors.text, active_tab_bg),
        };

        let has_end_slot = self.end_slot.is_some();

        let (start_slot, end_slot) = {
            let start_slot = self.start_slot.map(|start_slot| {
                h_flex()
                    .w(START_TAB_SLOT_SIZE)
                    .justify_center()
                    .child(start_slot)
                    .into_any_element()
            });

            let end_slot = self.end_slot.map(|end_slot| {
                h_flex()
                    .w(END_TAB_SLOT_SIZE)
                    .justify_start()
                    .child(end_slot)
                    .into_any_element()
            });

            match self.close_side {
                TabCloseSide::End => (start_slot, end_slot),
                TabCloseSide::Start => (end_slot, start_slot),
            }
        };

        self.div
            .relative()
            .h(Tab::container_height(cx))
            .map(|this| match self.position {
                TabPosition::First => this.pl_px().pr_px(),
                TabPosition::Last => this.pl_px().pr_px(),
                TabPosition::Middle(_) => this.px_px(),
            })
            .cursor_pointer()
            .child(
                h_flex()
                    .group("")
                    .relative()
                    .h(Tab::content_height(cx))
                    .mx_1()
                    .mt(px(6.))
                    .mb(px(6.))
                    .pl(px(8.))
                    .when(!has_end_slot, |this| this.pr(px(8.)))
                    .gap(DynamicSpacing::Base04.rems(cx))
                    .rounded_md()
                    .bg(colors.ghost_element_background)
                    .hover(|style| style.bg(tab_hover_bg))
                    .when(self.selected, |this| this.bg(active_tab_bg))
                    .text_color(text_color)
                    .when_some(start_slot, |this, start_slot| this.child(start_slot))
                    .children(self.children)
                    .when_some(end_slot, |this, end_slot| this.child(end_slot)),
            )
    }
}

impl Component for Tab {
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn description() -> Option<&'static str> {
        Some(
            "A tab component that can be used in a tabbed interface, supporting different positions and states.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Variations",
                    vec![
                        single_example(
                            "Default",
                            Tab::new("default").child("Default Tab").into_any_element(),
                        ),
                        single_example(
                            "Selected",
                            Tab::new("selected")
                                .toggle_state(true)
                                .child("Selected Tab")
                                .into_any_element(),
                        ),
                        single_example(
                            "First",
                            Tab::new("first")
                                .position(TabPosition::First)
                                .child("First Tab")
                                .into_any_element(),
                        ),
                        single_example(
                            "Middle",
                            Tab::new("middle")
                                .position(TabPosition::Middle(Ordering::Equal))
                                .child("Middle Tab")
                                .into_any_element(),
                        ),
                        single_example(
                            "Last",
                            Tab::new("last")
                                .position(TabPosition::Last)
                                .child("Last Tab")
                                .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
