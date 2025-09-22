use std::sync::Arc;
use gpui::{App, Context, IntoElement, Render, Window, px};
use ui::{
    Button, ButtonSize, ButtonStyle, h_flex, v_flex, Label, LabelSize, 
    prelude::*, IconName, Toggle
};
use crate::{
    novel_agent_profiles::{novel_profiles, NovelAgentProfilesBuilder},
    novel_mcp_manager::{NovelMcpManager, NovelMcpServerType, NovelMcpServerConfig},
};

/// 小说写作Agent配置界面
pub struct NovelAgentConfigView {
    selected_profile: String,
    mcp_manager: NovelMcpManager,
    show_advanced_settings: bool,
}

impl NovelAgentConfigView {
    pub fn new() -> Self {
        Self {
            selected_profile: novel_profiles::NOVELIST.to_string(),
            mcp_manager: NovelMcpManager::new(),
            show_advanced_settings: false,
        }
    }
    
    pub fn get_mcp_manager(&self) -> &NovelMcpManager {
        &self.mcp_manager
    }
    
    pub fn get_mcp_manager_mut(&mut self) -> &mut NovelMcpManager {
        &mut self.mcp_manager
    }
    
    pub fn set_profile(&mut self, profile_id: String) {
        self.selected_profile = profile_id.clone();
        
        // 自动配置MCP服务器
        if let Err(e) = self.mcp_manager.auto_configure_for_profile(&profile_id) {
            eprintln!("自动配置MCP服务器失败: {}", e);
        }
    }
    
    pub fn get_selected_profile(&self) -> &str {
        &self.selected_profile
    }
}

impl Render for NovelAgentConfigView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .padding(px(16.))
            .child(
                // 标题
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("Agent Configuration")
                            .size(LabelSize::Large)
                    )
                    .child(
                        Button::new("toggle_advanced", "Advanced")
                            .style(if self.show_advanced_settings { 
                                ButtonStyle::Filled 
                            } else { 
                                ButtonStyle::Subtle 
                            })
                            .size(ButtonSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.show_advanced_settings = !this.show_advanced_settings;
                                cx.notify();
                            }))
                    )
            )
            .child(self.render_profile_selector(cx))
            .child(self.render_mcp_server_status(cx))
            .child(self.render_mcp_server_list(cx))
            .when(self.show_advanced_settings, |this| {
                this.child(self.render_advanced_settings(cx))
            })
    }
}

impl NovelAgentConfigView {
    fn render_profile_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Agent Profile")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(self.profile_button(novel_profiles::NOVELIST, "Novelist", "全功能写作助手", cx))
                    .child(self.profile_button(novel_profiles::RESEARCHER, "Researcher", "资料收集专家", cx))
                    .child(self.profile_button(novel_profiles::EDITOR, "Editor", "文本编辑专家", cx))
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(self.profile_button(novel_profiles::OUTLINER, "Outliner", "大纲结构专家", cx))
                    .child(self.profile_button(novel_profiles::CHARACTER_DEVELOPER, "Character Dev", "角色发展专家", cx))
            )
    }
    
    fn profile_button(&self, profile_id: &str, name: &str, description: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.selected_profile == profile_id;
        let profile_id = profile_id.to_string();
        
        Button::new(format!("profile_{}", profile_id), name)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Default)
            .tooltip(|tooltip| tooltip.text(description))
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.set_profile(profile_id.clone());
                cx.notify();
            }))
    }
    
    fn render_mcp_server_status(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.mcp_manager.get_status_summary();
        
        h_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Total:"))
                    .child(Label::new(status.total_servers.to_string()))
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Enabled:"))
                    .child(Label::new(status.enabled_servers.to_string()))
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Running:"))
                    .child(Label::new(status.running_servers.to_string()))
            )
    }
    
    fn render_mcp_server_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("MCP Servers")
                    .size(LabelSize::Default)
            )
            .children(
                self.mcp_manager
                    .get_available_servers()
                    .values()
                    .map(|server_config| self.render_server_item(server_config, cx))
            )
    }
    
    fn render_server_item(&self, server_config: &NovelMcpServerConfig, cx: &mut Context<Self>) -> impl IntoElement {
        let server_id = server_config.id.clone();
        let is_enabled = server_config.enabled;
        let is_running = self.mcp_manager.is_server_running(&server_config.id);
        
        h_flex()
            .justify_between()
            .items_center()
            .padding(px(8.))
            .border_1()
            .border_color(ui::colors::border())
            .rounded(px(4.))
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new(&server_config.name)
                                    .size(LabelSize::Default)
                            )
                            .child(
                                Label::new(self.server_type_display(&server_config.server_type))
                                    .size(LabelSize::Small)
                            )
                            .when(is_running, |this| {
                                this.child(
                                    Label::new("● Running")
                                        .size(LabelSize::Small)
                                )
                            })
                    )
                    .child(
                        Label::new(&server_config.description)
                            .size(LabelSize::Small)
                    )
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Toggle::new(format!("toggle_{}", server_id))
                            .checked(is_enabled)
                            .on_click(cx.listener(move |this, _checked, _window, cx| {
                                if is_enabled {
                                    let _ = this.mcp_manager.disable_server(&server_id);
                                } else {
                                    let _ = this.mcp_manager.enable_server(&server_id);
                                }
                                cx.notify();
                            }))
                    )
                    .when(is_enabled && !is_running, |this| {
                        this.child(
                            Button::new(format!("start_{}", server_id), "Start")
                                .size(ButtonSize::Small)
                                .on_click(cx.listener(move |_this, _, _window, _cx| {
                                    // TODO: 启动服务器
                                    println!("启动服务器: {}", server_id);
                                }))
                        )
                    })
                    .when(is_running, |this| {
                        this.child(
                            Button::new(format!("stop_{}", server_id), "Stop")
                                .size(ButtonSize::Small)
                                .style(ButtonStyle::Subtle)
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    let _ = this.mcp_manager.stop_server(&server_id);
                                    cx.notify();
                                }))
                        )
                    })
            )
    }
    
    fn render_advanced_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Advanced Settings")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("start_all", "Start All Enabled")
                            .size(ButtonSize::Small)
                            .on_click(cx.listener(|_this, _, _window, _cx| {
                                // TODO: 启动所有已启用的服务器
                                println!("启动所有已启用的服务器");
                            }))
                    )
                    .child(
                        Button::new("stop_all", "Stop All")
                            .size(ButtonSize::Small)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.mcp_manager.stop_all_servers();
                                cx.notify();
                            }))
                    )
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Recommended for current profile:"))
                    .children(
                        self.mcp_manager
                            .get_recommended_servers_for_profile(&self.selected_profile)
                            .into_iter()
                            .map(|server| {
                                Label::new(format!("• {}", server.name))
                                    .size(LabelSize::Small)
                            })
                    )
            )
    }
    
    fn server_type_display(&self, server_type: &NovelMcpServerType) -> &'static str {
        match server_type {
            NovelMcpServerType::Research => "Research",
            NovelMcpServerType::Outline => "Outline",
            NovelMcpServerType::Editing => "Editing",
            NovelMcpServerType::Character => "Character",
            NovelMcpServerType::WorldBuilding => "World",
            NovelMcpServerType::Timeline => "Timeline",
        }
    }
}

impl Default for NovelAgentConfigView {
    fn default() -> Self {
        Self::new()
    }
}
