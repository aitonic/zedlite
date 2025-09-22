use std::collections::HashMap;
use gpui::{IntoElement, Context, px};
use ui::{
    v_flex, h_flex, Label, LabelSize, IconName, IconButton, ButtonSize, Button, ButtonStyle,
    TextInput, prelude::*
};
use crate::models::{Character, Relationship, RelationshipType, Scene};
use crate::ManuscriptPanel;

pub struct CharactersView {
    characters: Vec<Character>,
    scenes: Vec<Scene>,
    selected_character_id: Option<String>,
    new_character_name: String,
    is_creating_character: bool,
    show_relationships: bool,
}

impl CharactersView {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
            scenes: Vec::new(),
            selected_character_id: None,
            new_character_name: String::new(),
            is_creating_character: false,
            show_relationships: false,
        }
    }
    
    pub fn add_character(&mut self, name: String) {
        let character = Character::new(name);
        self.characters.push(character);
    }
    
    pub fn remove_character(&mut self, character_id: &str) {
        self.characters.retain(|character| character.id != character_id);
        
        for character in &mut self.characters {
            character.remove_relationship(character_id);
        }
        
        if self.selected_character_id.as_ref() == Some(&character_id.to_string()) {
            self.selected_character_id = None;
        }
    }
    
    pub fn get_character(&self, character_id: &str) -> Option<&Character> {
        self.characters.iter().find(|character| character.id == character_id)
    }
    
    pub fn render(&mut self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(self.render_header(cx))
            .child(self.render_view_toggle(cx))
            .child(
                if self.show_relationships {
                    self.render_relationships_view(cx)
                } else {
                    self.render_characters_list(cx)
                }
            )
            .child(
                if let Some(character_id) = &self.selected_character_id.clone() {
                    Some(self.render_character_details(character_id, cx))
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
                Label::new("characters-title", format!("角色管理 ({})", self.characters.len()))
                    .size(LabelSize::Small)
            )
            .child(
                if self.is_creating_character {
                    h_flex()
                        .gap_1()
                        .child(
                            TextInput::new("new-character-input")
                                .placeholder("角色姓名")
                        )
                        .child(
                            Button::new("confirm-character", "确认")
                                .style(ButtonStyle::Filled)
                                .size(ButtonSize::Small)
                        )
                        .child(
                            Button::new("cancel-character", "取消")
                                .style(ButtonStyle::Subtle)
                                .size(ButtonSize::Small)
                        )
                } else {
                    IconButton::new("add-character", IconName::Plus)
                        .size(ButtonSize::Small)
                }
            )
    }
    
    fn render_view_toggle(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(
                Button::new("list-view", "角色列表")
                    .style(if !self.show_relationships { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(characters_view) = this.get_characters_view_mut() {
                            characters_view.show_relationships = false;
                        }
                        cx.notify();
                    }))
            )
            .child(
                Button::new("relationships-view", "关系网络")
                    .style(if self.show_relationships { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(characters_view) = this.get_characters_view_mut() {
                            characters_view.show_relationships = true;
                        }
                        cx.notify();
                    }))
            )
    }
    
    fn render_characters_list(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .children(
                self.characters.iter().map(|character| {
                    self.render_character_item(character, cx)
                })
            )
            .when(self.characters.is_empty(), |this| {
                this.child(
                    Label::new("no-characters", "暂无角色，点击上方按钮添加角色")
                        .size(LabelSize::Small)
                )
            })
    }
    
    fn render_character_item(&self, character: &Character, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let character_id = character.id.clone();
        
        h_flex()
            .gap_2()
            .items_center()
            .p_2()
            .rounded_md()
            .child(
                IconButton::new(format!("character-avatar-{}", character.id), IconName::Person)
                    .size(ButtonSize::Small)
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(format!("character-name-{}", character.id), character.name.clone())
                            .size(LabelSize::Small)
                    )
                    .child(
                        Label::new(format!("character-occupation-{}", character.id), 
                            if character.occupation.is_empty() { 
                                "职业未设置".to_string() 
                            } else { 
                                character.occupation.clone() 
                            })
                            .size(LabelSize::XSmall)
                    )
            )
            .child(
                Label::new(format!("character-age-{}", character.id), 
                    character.age.map(|age| format!("{}岁", age)).unwrap_or_else(|| "年龄未知".to_string()))
                    .size(LabelSize::XSmall)
            )
            .child(
                IconButton::new(format!("delete-character-{}", character.id), IconName::Trash)
                    .size(ButtonSize::Small)
            )
            .on_click(cx.listener(move |this, _, _window, cx| {
                if let Some(characters_view) = this.get_characters_view_mut() {
                    characters_view.selected_character_id = Some(character_id.clone());
                }
                cx.notify();
            }))
    }
    
    fn render_relationships_view(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("relationships-title", "角色关系网络")
                    .size(LabelSize::Default)
            )
            .child(
                v_flex()
                    .gap_3()
                    .children(
                        self.characters.iter().map(|character| {
                            self.render_character_relationships(character, cx)
                        })
                    )
            )
            .when(self.characters.is_empty(), |this| {
                this.child(
                    Label::new("no-relationships", "暂无角色，无法显示关系网络")
                        .size(LabelSize::Small)
                )
            })
    }
    
    fn render_character_relationships(&self, character: &Character, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let character_id = character.id.clone();
        
        v_flex()
            .gap_2()
            .p_3()
            .rounded_md()
            .border_1()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        IconButton::new(format!("rel-avatar-{}", character.id), IconName::Person)
                            .size(ButtonSize::Small)
                    )
                    .child(
                        Label::new(format!("rel-name-{}", character.id), character.name.clone())
                            .size(LabelSize::Small)
                    )
            )
            .child(
                if character.relationships.is_empty() {
                    Label::new(format!("no-rel-{}", character.id), "暂无关系")
                        .size(LabelSize::XSmall)
                } else {
                    v_flex()
                        .gap_1()
                        .children(
                            character.relationships.iter().map(|(other_id, relationship)| {
                                self.render_single_relationship(character, other_id, relationship)
                            })
                        )
                }
            )
            .on_click(cx.listener(move |this, _, _window, cx| {
                if let Some(characters_view) = this.get_characters_view_mut() {
                    characters_view.selected_character_id = Some(character_id.clone());
                }
                cx.notify();
            }))
    }
    
    fn render_single_relationship(&self, _character: &Character, other_id: &str, relationship: &Relationship) -> impl IntoElement {
        let other_character_name = self.get_character(other_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "未知角色".to_string());
            
        let relationship_text = match &relationship.relationship_type {
            RelationshipType::Friend => "朋友",
            RelationshipType::Enemy => "敌人", 
            RelationshipType::Family => "家人",
            RelationshipType::Romance => "恋人",
            RelationshipType::Colleague => "同事",
            RelationshipType::Stranger => "陌生人",
            RelationshipType::Other(custom) => custom,
        };
        
        h_flex()
            .gap_2()
            .items_center()
            .child(
                Label::new(format!("arrow-{}-{}", _character.id, other_id), "→")
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(format!("rel-other-{}-{}", _character.id, other_id), other_character_name)
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(format!("rel-type-{}-{}", _character.id, other_id), 
                    format!("({})", relationship_text))
                    .size(LabelSize::XSmall)
            )
    }
    
    fn render_character_details(&self, character_id: &str, _cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        if let Some(character) = self.get_character(character_id) {
            v_flex()
                .gap_2()
                .p_3()
                .rounded_md()
                .child(
                    Label::new("character-detail-title", format!("角色档案: {}", character.name))
                        .size(LabelSize::Default)
                )
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    Label::new("detail-age-label", "年龄:")
                                        .size(LabelSize::Small)
                                )
                                .child(
                                    Label::new("detail-age-value", 
                                        character.age.map(|age| format!("{}岁", age)).unwrap_or_else(|| "未设置".to_string()))
                                        .size(LabelSize::Small)
                                )
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    Label::new("detail-occupation-label", "职业:")
                                        .size(LabelSize::Small)
                                )
                                .child(
                                    Label::new("detail-occupation-value", 
                                        if character.occupation.is_empty() { "未设置" } else { &character.occupation })
                                        .size(LabelSize::Small)
                                )
                        )
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            Label::new("detail-description-label", "角色描述:")
                                .size(LabelSize::Small)
                        )
                        .child(
                            Label::new("detail-description-value", 
                                if character.description.is_empty() {
                                    "暂无描述".to_string()
                                } else {
                                    character.description.clone()
                                })
                                .size(LabelSize::Small)
                        )
                )
        } else {
            Label::new("character-not-found", "角色未找到")
        }
    }
}

impl Default for CharactersView {
    fn default() -> Self {
        Self::new()
    }
}
