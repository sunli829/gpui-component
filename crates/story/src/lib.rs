mod button_story;
mod calendar_story;
mod dropdown_story;
mod icon_story;
mod image_story;
mod input_story;
mod list_story;
mod modal_story;
mod popup_story;
mod progress_story;
mod resizable_story;
mod scrollable_story;
mod switch_story;
mod table_story;
mod text_story;
mod tooltip_story;
mod webview_story;

pub use button_story::ButtonStory;
pub use calendar_story::CalendarStory;
pub use dropdown_story::DropdownStory;
pub use icon_story::IconStory;
pub use image_story::ImageStory;
pub use input_story::InputStory;
pub use list_story::ListStory;
pub use modal_story::ModalStory;
pub use popup_story::PopupStory;
pub use progress_story::ProgressStory;
pub use resizable_story::ResizableStory;
pub use scrollable_story::ScrollableStory;
pub use switch_story::SwitchStory;
pub use table_story::TableStory;
pub use text_story::TextStory;
pub use tooltip_story::TooltipStory;
pub use webview_story::WebViewStory;

use gpui::{
    actions, div, prelude::FluentBuilder as _, px, AnyView, AppContext, Div, EventEmitter,
    FocusableView, Hsla, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled as _, View, ViewContext, VisualContext, WindowContext,
};

use ui::{
    divider::Divider,
    dock::{Panel, PanelEvent, TitleStyle},
    h_flex,
    label::Label,
    notification::Notification,
    popup_menu::PopupMenu,
    theme::ActiveTheme,
    v_flex, ContextModal,
};

pub fn init(cx: &mut AppContext) {
    input_story::init(cx);
    dropdown_story::init(cx);
    popup_story::init(cx);
}

actions!(story, [PanelInfo]);

pub fn section(title: impl IntoElement, cx: &WindowContext) -> Div {
    use ui::theme::ActiveTheme;
    let theme = cx.theme();

    h_flex()
        .items_center()
        .gap_4()
        .p_4()
        .w_full()
        .rounded_lg()
        .border_1()
        .border_color(theme.border)
        .flex_wrap()
        .justify_around()
        .child(div().flex_none().w_full().child(title))
}

pub struct StoryContainer {
    focus_handle: gpui::FocusHandle,
    name: SharedString,
    title_bg: Option<Hsla>,
    description: SharedString,
    width: Option<gpui::Pixels>,
    height: Option<gpui::Pixels>,
    story: Option<AnyView>,
    story_klass: Option<SharedString>,
    closeable: bool,
}

#[derive(Debug)]
pub enum ContainerEvent {
    Close,
}

pub trait Story {
    fn klass() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn title() -> &'static str;
    fn description() -> &'static str {
        ""
    }
    fn closeable() -> bool {
        true
    }
    fn title_bg() -> Option<Hsla> {
        None
    }
    fn new_view(cx: &mut WindowContext) -> AnyView;
}

impl EventEmitter<ContainerEvent> for StoryContainer {}

impl StoryContainer {
    pub fn new(closeable: bool, cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            name: "".into(),
            title_bg: None,
            description: "".into(),
            width: None,
            height: None,
            story: None,
            story_klass: None,
            closeable,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn panel<S: Story>(cx: &mut WindowContext) -> View<Self> {
        let name = S::title();
        let description = S::description();
        let story = S::new_view(cx);
        let story_klass = S::klass();

        let view = cx.new_view(|cx| {
            let mut story = Self::new(S::closeable(), cx).story(story, story_klass);
            story.name = name.into();
            story.description = description.into();
            story.title_bg = S::title_bg();
            story
        });

        view
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn story(mut self, story: AnyView, story_klass: impl Into<SharedString>) -> Self {
        self.story = Some(story);
        self.story_klass = Some(story_klass.into());
        self
    }

    fn on_action_panel_info(&mut self, _: &PanelInfo, cx: &mut ViewContext<Self>) {
        struct Info;
        let note = Notification::new(format!("You have clicked panel info on: {}", self.name))
            .id::<Info>();
        cx.push_notification(note);
    }
}

impl Panel for StoryContainer {
    fn title(&self, _cx: &WindowContext) -> SharedString {
        self.name.clone()
    }

    fn title_style(&self, cx: &WindowContext) -> Option<TitleStyle> {
        if let Some(bg) = self.title_bg {
            Some(TitleStyle {
                background: bg,
                foreground: cx.theme().foreground,
            })
        } else {
            None
        }
    }

    fn closeable(&self, _cx: &WindowContext) -> bool {
        self.closeable
    }

    fn popup_menu(&self, menu: PopupMenu, _cx: &WindowContext) -> PopupMenu {
        menu.track_focus(&self.focus_handle)
            .menu("Info", Box::new(PanelInfo))
    }
}

impl EventEmitter<PanelEvent> for StoryContainer {}
impl FocusableView for StoryContainer {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for StoryContainer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("story-container")
            .size_full()
            .overflow_scroll()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_action_panel_info))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .child(Label::new(self.description.clone()).text_size(px(16.0)))
                    .child(Divider::horizontal().label("This is a divider")),
            )
            .when_some(self.story.clone(), |this, story| {
                this.child(
                    v_flex()
                        .id("story-children")
                        .overflow_scroll()
                        .size_full()
                        .p_4()
                        .child(story),
                )
            })
    }
}
