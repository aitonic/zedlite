use gpui::{
    actions, App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, Pixels, Render,
    WeakEntity, Window, px,
};
use ui::{
    Button, ButtonSize, ButtonStyle, h_flex, TextInput,
    Label, LabelSize, IconName, prelude::*, v_flex
};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

mod models;
mod navigator;
mod scenes;
mod characters;
mod preview;
mod storage;
mod manuscript;
mod agent_api;
mod novel_agent_profiles;
mod novel_mcp_manager;
mod novel_agent_config;
mod project_manager;
mod file_search;
mod project_management_view;
mod writing_analytics;
mod writing_assistant_view;

pub use models::*;
pub use storage::*;
pub use manuscript::*;
pub use agent_api::*;
pub use novel_agent_profiles::*;
pub use novel_mcp_manager::*;
pub use novel_agent_config::*;
pub use project_manager::*;
pub use file_search::*;
pub use project_management_view::*;
pub use writing_analytics::*;
pub use writing_assistant_view::*;
use navigator::NavigatorView;
use scenes::ScenesView;
use characters::CharactersView;
use preview::PreviewView;

actions!(manuscript_panel, [ToggleFocus]);

const PANEL_NAME: &str = "ManuscriptPanel";
const DEFAULT_WIDTH: Pixels = px(280.);

#[derive(Debug, Clone, PartialEq)]
pub enum ManuscriptMode {
    Navigator,  // 文件导航
    Scenes,     // 场景管理  
    Characters, // 角色管理
    Preview,    // 预览模式
    AgentConfig, // Agent配置
    ProjectManager, // 项目管理
    WritingAssistant, // 写作助手
}

impl Default for ManuscriptMode {
    fn default() -> Self {
        ManuscriptMode::Navigator
    }
}

/// Placeholder panel that will evolve into a manuscript navigator.
pub struct ManuscriptPanel {
    workspace: WeakEntity<Workspace>,
    focus: FocusHandle,
    position: DockPosition,
    width: Pixels,
    zoomed: bool,
    active: bool,
    mode: ManuscriptMode,
    navigator: NavigatorView,
    scenes: ScenesView,
    characters: CharactersView,
    preview: PreviewView,
    agent_config: NovelAgentConfigView,
    project_management: ProjectManagementView,
    writing_assistant: WritingAssistantView,
}

impl EventEmitter<PanelEvent> for ManuscriptPanel {}

impl ManuscriptPanel {
    pub fn register(app: &mut App) {
        app.observe_new(|workspace: &mut Workspace, _, _| {
            workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<ManuscriptPanel>(window, cx);
            });
        })
        .detach();
    }

    pub fn load(workspace: WeakEntity<Workspace>, cx: gpui::AsyncWindowContext) -> gpui::Task<anyhow::Result<gpui::Entity<Self>>> {
        cx.spawn(async move |cx| {
            workspace.update_in(cx, |workspace, window, cx| Ok(Self::new(workspace, window, cx)))
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> gpui::Entity<Self> {
        let focus = cx.focus_handle();
        let workspace_handle = workspace.weak_handle();
        cx.new(|_cx| ManuscriptPanel {
            workspace: workspace_handle,
            focus,
            position: DockPosition::Left,
            width: DEFAULT_WIDTH,
            zoomed: false,
            active: false,
            mode: ManuscriptMode::Navigator,
            navigator: NavigatorView::new(),
            scenes: ScenesView::new(),
            characters: CharactersView::new(),
            preview: PreviewView::new(),
            agent_config: NovelAgentConfigView::new(),
            project_management: ProjectManagementView::new(),
            writing_assistant: WritingAssistantView::new(),
        })
    }
}

impl Render for ManuscriptPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_mode = self.mode.clone();
        
        v_flex()
            .track_focus(&self.focus_handle(cx.app()))
            .gap_2()
            .padding(px(12.))
            .child(
                Label::new("Manuscript")
                    .size(LabelSize::Large),
            )
            .child(
                // Mode selector tabs
                h_flex()
                    .gap_1()
                    .child(self.mode_button("Navigator", ManuscriptMode::Navigator, current_mode.clone(), cx))
                    .child(self.mode_button("Scenes", ManuscriptMode::Scenes, current_mode.clone(), cx))
                    .child(self.mode_button("Characters", ManuscriptMode::Characters, current_mode.clone(), cx))
                    .child(self.mode_button("Preview", ManuscriptMode::Preview, current_mode.clone(), cx))
                    .child(self.mode_button("Agent", ManuscriptMode::AgentConfig, current_mode.clone(), cx))
                    .child(self.mode_button("Project", ManuscriptMode::ProjectManager, current_mode.clone(), cx))
                    .child(self.mode_button("Assistant", ManuscriptMode::WritingAssistant, current_mode.clone(), cx))
            )
            .child(self.render_mode_content(current_mode, cx))
    }
}

impl ManuscriptPanel {
    fn mode_button(
        &self,
        label: &str,
        mode: ManuscriptMode,
        current_mode: ManuscriptMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = mode == current_mode;
        let mode_for_click = mode.clone();
        
        Button::new(format!("mode-{:?}", mode), label)
            .style(if is_active { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.mode = mode_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_mode_content(&mut self, mode: ManuscriptMode, cx: &mut Context<Self>) -> impl IntoElement {
        match mode {
            ManuscriptMode::Navigator => {
                self.navigator.render(cx)
            }
            ManuscriptMode::Scenes => {
                self.scenes.render(cx)
            }
            ManuscriptMode::Characters => {
                self.characters.render(cx)
            }
            ManuscriptMode::Preview => {
                self.preview.render(cx)
            }
            ManuscriptMode::AgentConfig => {
                self.agent_config.render(_window, cx)
            }
            ManuscriptMode::ProjectManager => {
                self.project_management.render(_window, cx)
            }
            ManuscriptMode::WritingAssistant => {
                self.writing_assistant.render(_window, cx)
            }
        }
    }
    
    pub fn get_scenes_view_mut(&mut self) -> Option<&mut ScenesView> {
        Some(&mut self.scenes)
    }
    
    pub fn get_characters_view_mut(&mut self) -> Option<&mut CharactersView> {
        Some(&mut self.characters)
    }
    
    pub fn get_preview_view_mut(&mut self) -> Option<&mut PreviewView> {
        Some(&mut self.preview)
    }
    
    pub fn get_agent_config_mut(&mut self) -> Option<&mut NovelAgentConfigView> {
        Some(&mut self.agent_config)
    }
    
    pub fn get_project_management_mut(&mut self) -> Option<&mut ProjectManagementView> {
        Some(&mut self.project_management)
    }
    
    pub fn get_writing_assistant_mut(&mut self) -> Option<&mut WritingAssistantView> {
        Some(&mut self.writing_assistant)
    }
}

impl Panel for ManuscriptPanel {
    fn persistent_name() -> &'static str {
        PANEL_NAME
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _window: &mut Window, _cx: &mut Context<Self>) {
        self.position = position;
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size.unwrap_or(DEFAULT_WIDTH);
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Book)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Manuscript Navigator")
    }

    fn icon_label(&self, _window: &Window, _cx: &App) -> Option<String> {
        Some("Manuscript".into())
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        ToggleFocus.boxed_clone()
    }

    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        self.zoomed = zoomed;
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, _cx: &mut Context<Self>) {
        self.active = active;
    }

    fn activation_priority(&self) -> u32 {
        100
    }

    fn enabled(&self, cx: &App) -> bool {
        // Eventually respect settings; always enabled for prototype.
        let _ = cx;
        true
    }
}

impl Focusable for ManuscriptPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}
