use std::collections::HashMap;
use gpui::{IntoElement, Context, px};
use ui::{
    v_flex, h_flex, Label, LabelSize, IconName, IconButton, ButtonSize, Button, ButtonStyle,
    TextInput, prelude::*
};
use crate::models::{Scene, Character};
use crate::ManuscriptPanel;

pub struct ScenesView {
    scenes: Vec<Scene>,
    characters: Vec<Character>,
    selected_scene_id: Option<String>,
    editing_scene_id: Option<String>,
    new_scene_title: String,
    is_creating_scene: bool,
    show_timeline: bool,
}

impl ScenesView {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            characters: Vec::new(),
            selected_scene_id: None,
            editing_scene_id: None,
            new_scene_title: String::new(),
            is_creating_scene: false,
            show_timeline: false,
        }
    }
    
    pub fn add_scene(&mut self, title: String) {
        let scene = Scene::new(title);
        self.scenes.push(scene);
    }
    
    pub fn remove_scene(&mut self, scene_id: &str) {
        self.scenes.retain(|scene| scene.id != scene_id);
        if self.selected_scene_id.as_ref() == Some(&scene_id.to_string()) {
            self.selected_scene_id = None;
        }
    }
    
    pub fn get_scene_mut(&mut self, scene_id: &str) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|scene| scene.id == scene_id)
    }
    
    pub fn get_scene(&self, scene_id: &str) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == scene_id)
    }
    
    pub fn render(&mut self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(self.render_header(cx))
            .child(self.render_view_toggle(cx))
            .child(
                if self.show_timeline {
                    self.render_timeline_view(cx)
                } else {
                    self.render_scenes_list(cx)
                }
            )
            .child(
                if let Some(scene_id) = &self.selected_scene_id.clone() {
                    Some(self.render_scene_details(scene_id, cx))
                } else {
                    None
                }
            )
    }
    
    fn render_header(&mut self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .justify_between()
            .child(
                Label::new("scenes-title", format!("场景管理 ({})", self.scenes.len()))
                    .size(LabelSize::Small)
            )
            .child(
                if self.is_creating_scene {
                    h_flex()
                        .gap_1()
                        .child(
                            TextInput::new("new-scene-input")
                                .placeholder("场景标题")
                        )
                        .child(
                            Button::new("confirm-scene", "确认")
                                .style(ButtonStyle::Filled)
                                .size(ButtonSize::Small)
                        )
                        .child(
                            Button::new("cancel-scene", "取消")
                                .style(ButtonStyle::Subtle)
                                .size(ButtonSize::Small)
                        )
                } else {
                    IconButton::new("add-scene", IconName::Plus)
                        .size(ButtonSize::Small)
                }
            )
    }
    
    fn render_view_toggle(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(
                Button::new("list-view", "列表视图")
                    .style(if !self.show_timeline { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(scenes_view) = this.get_scenes_view_mut() {
                            scenes_view.show_timeline = false;
                        }
                        cx.notify();
                    }))
            )
            .child(
                Button::new("timeline-view", "时间线视图")
                    .style(if self.show_timeline { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(scenes_view) = this.get_scenes_view_mut() {
                            scenes_view.show_timeline = true;
                        }
                        cx.notify();
                    }))
            )
    }
    
    fn render_timeline_view(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let mut sorted_scenes = self.scenes.clone();
        sorted_scenes.sort_by_key(|scene| scene.timeline_position.unwrap_or(999));
        
        v_flex()
            .gap_2()
            .child(
                Label::new("timeline-title", "情节时间线")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_4()
                    .children(
                        sorted_scenes.iter().enumerate().map(|(index, scene)| {
                            self.render_timeline_item(scene, index, cx)
                        })
                    )
            )
            .when(sorted_scenes.is_empty(), |this| {
                this.child(
                    Label::new("no-scenes-timeline", "暂无场景，点击上方按钮添加场景")
                        .size(LabelSize::Small)
                )
            })
    }
    
    fn render_timeline_item(&self, scene: &Scene, index: usize, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let is_selected = self.selected_scene_id.as_ref() == Some(&scene.id);
        let scene_id = scene.id.clone();
        
        v_flex()
            .gap_2()
            .min_w(px(150.))
            .p_3()
            .rounded_md()
            .border_1()
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        Label::new(format!("timeline-position-{}", scene.id), format!("#{}", index + 1))
                            .size(LabelSize::XSmall)
                    )
                    .child(
                        IconButton::new(format!("timeline-delete-{}", scene.id), IconName::Trash)
                            .size(ButtonSize::Small)
                    )
            )
            .child(
                Label::new(format!("timeline-title-{}", scene.id), scene.title.clone())
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(format!("timeline-location-{}", scene.id), 
                    if scene.location.is_empty() { 
                        "地点未设置".to_string() 
                    } else { 
                        scene.location.clone() 
                    })
                    .size(LabelSize::XSmall)
            )
            .child(
                Label::new(format!("timeline-characters-{}", scene.id), 
                    format!("{}个角色", scene.characters.len()))
                    .size(LabelSize::XSmall)
            )
            .on_click(cx.listener(move |this, _, _window, cx| {
                if let Some(scenes_view) = this.get_scenes_view_mut() {
                    scenes_view.selected_scene_id = Some(scene_id.clone());
                }
                cx.notify();
            }))
    }
    
    fn render_scenes_list(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .children(
                self.scenes.iter().enumerate().map(|(index, scene)| {
                    self.render_scene_item(scene, index + 1, cx)
                })
            )
    }
    
    fn render_scene_item(&self, scene: &Scene, position: usize, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let is_selected = self.selected_scene_id.as_ref() == Some(&scene.id);
        let scene_id = scene.id.clone();
        
        h_flex()
            .gap_2()
            .items_center()
            .p_2()
            .rounded_md()
            .child(
                Label::new(format!("scene-position-{}", scene.id), position.to_string())
                    .size(LabelSize::XSmall)
            )
            .child(
                Label::new(format!("scene-title-{}", scene.id), scene.title.clone())
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(format!("scene-location-{}", scene.id), 
                    if scene.location.is_empty() { 
                        "未设置地点".to_string() 
                    } else { 
                        scene.location.clone() 
                    })
                    .size(LabelSize::XSmall)
            )
            .child(
                Label::new(format!("scene-characters-{}", scene.id), 
                    format!("{}个角色", scene.characters.len()))
                    .size(LabelSize::XSmall)
            )
            .child(
                IconButton::new(format!("delete-scene-{}", scene.id), IconName::Trash)
                    .size(ButtonSize::Small)
            )
            .on_click(cx.listener(move |this, _, _window, cx| {
                if let Some(scenes_view) = this.get_scenes_view_mut() {
                    scenes_view.selected_scene_id = Some(scene_id.clone());
                }
                cx.notify();
            }))
    }
    
    fn render_scene_details(&self, scene_id: &str, _cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        if let Some(scene) = self.get_scene(scene_id) {
            v_flex()
                .gap_2()
                .p_3()
                .rounded_md()
                .child(
                    Label::new("scene-detail-title", format!("场景详情: {}", scene.title))
                        .size(LabelSize::Default)
                )
                .child(
                    Label::new("scene-detail-description", 
                        if scene.description.is_empty() {
                            "暂无描述".to_string()
                        } else {
                            scene.description.clone()
                        })
                        .size(LabelSize::Small)
                )
                .child(
                    Label::new("scene-detail-location", 
                        format!("地点: {}", 
                            if scene.location.is_empty() { 
                                "未设置" 
                            } else { 
                                &scene.location 
                            }))
                        .size(LabelSize::Small)
                )
                .child(
                    Label::new("scene-detail-characters", 
                        format!("出场角色: {}", 
                            if scene.characters.is_empty() {
                                "无".to_string()
                            } else {
                                scene.characters.join(", ")
                            }))
                        .size(LabelSize::Small)
                )
        } else {
            Label::new("scene-not-found", "场景未找到")
        }
    }
}

impl Default for ScenesView {
    fn default() -> Self {
        Self::new()
    }
}
