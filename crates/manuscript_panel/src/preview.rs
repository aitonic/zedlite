use std::path::PathBuf;
use gpui::{IntoElement, Context, px};
use ui::{
    v_flex, h_flex, Label, LabelSize, IconName, IconButton, ButtonSize, Button, ButtonStyle,
    prelude::*
};
use crate::ManuscriptPanel;

pub struct PreviewView {
    current_file: Option<PathBuf>,
    preview_content: String,
    word_count: u32,
    character_count: u32,
    paragraph_count: u32,
    reading_time_minutes: u32,
    writing_mode: WritingMode,
    show_statistics: bool,
    font_size: FontSize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WritingMode {
    Focus,      // 专注模式，隐藏干扰元素
    Review,     // 审阅模式，显示完整内容
    Export,     // 导出模式，准备发布格式
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl PreviewView {
    pub fn new() -> Self {
        Self {
            current_file: None,
            preview_content: String::new(),
            word_count: 0,
            character_count: 0,
            paragraph_count: 0,
            reading_time_minutes: 0,
            writing_mode: WritingMode::Review,
            show_statistics: true,
            font_size: FontSize::Medium,
        }
    }
    
    pub fn load_file(&mut self, file_path: PathBuf) -> anyhow::Result<()> {
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            self.current_file = Some(file_path);
            self.update_content(content);
        }
        Ok(())
    }
    
    pub fn update_content(&mut self, content: String) {
        self.preview_content = content.clone();
        self.calculate_statistics(&content);
    }
    
    fn calculate_statistics(&mut self, content: &str) {
        // 计算字数（中文按字符计算，英文按单词计算）
        let chinese_chars = content.chars().filter(|c| {
            let cp = *c as u32;
            (cp >= 0x4E00 && cp <= 0x9FFF) || // CJK Unified Ideographs
            (cp >= 0x3400 && cp <= 0x4DBF) || // CJK Extension A
            (cp >= 0x20000 && cp <= 0x2A6DF)  // CJK Extension B
        }).count() as u32;
        
        let english_words = content
            .split_whitespace()
            .filter(|word| word.chars().any(|c| c.is_ascii_alphabetic()))
            .count() as u32;
            
        self.word_count = chinese_chars + english_words;
        self.character_count = content.chars().count() as u32;
        
        // 计算段落数
        self.paragraph_count = content
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .count() as u32;
            
        // 估算阅读时间（中文250字/分钟，英文200词/分钟）
        let chinese_reading_time = chinese_chars as f32 / 250.0;
        let english_reading_time = english_words as f32 / 200.0;
        self.reading_time_minutes = (chinese_reading_time + english_reading_time).ceil() as u32;
    }
    
    pub fn render(&mut self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(self.render_header(cx))
            .child(
                if self.show_statistics {
                    Some(self.render_statistics(cx))
                } else {
                    None
                }
            )
            .child(self.render_preview_content(cx))
    }
    
    fn render_header(&mut self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Label::new("preview-title", "预览模式")
                            .size(LabelSize::Small)
                    )
                    .child(
                        if let Some(file_path) = &self.current_file {
                            Label::new("preview-file", 
                                file_path.file_name()
                                    .and_then(|name| name.to_str())
                                    .unwrap_or("未知文件"))
                                .size(LabelSize::XSmall)
                        } else {
                            Label::new("preview-no-file", "未选择文件")
                                .size(LabelSize::XSmall)
                        }
                    )
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(self.render_mode_selector(cx))
                    .child(self.render_font_size_selector(cx))
                    .child(
                        IconButton::new("toggle-stats", IconName::Chart)
                            .size(ButtonSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                if let Some(preview_view) = this.get_preview_view_mut() {
                                    preview_view.show_statistics = !preview_view.show_statistics;
                                }
                                cx.notify();
                            }))
                    )
            )
    }
    
    fn render_mode_selector(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(
                Button::new("focus-mode", "专注")
                    .style(if self.writing_mode == WritingMode::Focus { 
                        ButtonStyle::Filled 
                    } else { 
                        ButtonStyle::Subtle 
                    })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(preview_view) = this.get_preview_view_mut() {
                            preview_view.writing_mode = WritingMode::Focus;
                        }
                        cx.notify();
                    }))
            )
            .child(
                Button::new("review-mode", "审阅")
                    .style(if self.writing_mode == WritingMode::Review { 
                        ButtonStyle::Filled 
                    } else { 
                        ButtonStyle::Subtle 
                    })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(preview_view) = this.get_preview_view_mut() {
                            preview_view.writing_mode = WritingMode::Review;
                        }
                        cx.notify();
                    }))
            )
            .child(
                Button::new("export-mode", "导出")
                    .style(if self.writing_mode == WritingMode::Export { 
                        ButtonStyle::Filled 
                    } else { 
                        ButtonStyle::Subtle 
                    })
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(preview_view) = this.get_preview_view_mut() {
                            preview_view.writing_mode = WritingMode::Export;
                        }
                        cx.notify();
                    }))
            )
    }
    
    fn render_font_size_selector(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .child(
                IconButton::new("font-decrease", IconName::Minus)
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(preview_view) = this.get_preview_view_mut() {
                            preview_view.font_size = match preview_view.font_size {
                                FontSize::ExtraLarge => FontSize::Large,
                                FontSize::Large => FontSize::Medium,
                                FontSize::Medium => FontSize::Small,
                                FontSize::Small => FontSize::Small,
                            };
                        }
                        cx.notify();
                    }))
            )
            .child(
                Label::new("font-size-label", match self.font_size {
                    FontSize::Small => "小",
                    FontSize::Medium => "中",
                    FontSize::Large => "大",
                    FontSize::ExtraLarge => "特大",
                })
                .size(LabelSize::XSmall)
            )
            .child(
                IconButton::new("font-increase", IconName::Plus)
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        if let Some(preview_view) = this.get_preview_view_mut() {
                            preview_view.font_size = match preview_view.font_size {
                                FontSize::Small => FontSize::Medium,
                                FontSize::Medium => FontSize::Large,
                                FontSize::Large => FontSize::ExtraLarge,
                                FontSize::ExtraLarge => FontSize::ExtraLarge,
                            };
                        }
                        cx.notify();
                    }))
            )
    }
    
    fn render_statistics(&self, _cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        h_flex()
            .gap_4()
            .p_2()
            .rounded_md()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("word-count-label", "字数")
                            .size(LabelSize::XSmall)
                    )
                    .child(
                        Label::new("word-count-value", self.word_count.to_string())
                            .size(LabelSize::Small)
                    )
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("char-count-label", "字符")
                            .size(LabelSize::XSmall)
                    )
                    .child(
                        Label::new("char-count-value", self.character_count.to_string())
                            .size(LabelSize::Small)
                    )
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("para-count-label", "段落")
                            .size(LabelSize::XSmall)
                    )
                    .child(
                        Label::new("para-count-value", self.paragraph_count.to_string())
                            .size(LabelSize::Small)
                    )
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("reading-time-label", "阅读时间")
                            .size(LabelSize::XSmall)
                    )
                    .child(
                        Label::new("reading-time-value", format!("{}分钟", self.reading_time_minutes))
                            .size(LabelSize::Small)
                    )
            )
    }
    
    fn render_preview_content(&self, _cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let font_size_class = match self.font_size {
            FontSize::Small => LabelSize::Small,
            FontSize::Medium => LabelSize::Default,
            FontSize::Large => LabelSize::Large,
            FontSize::ExtraLarge => LabelSize::Large, // 没有更大的可用
        };
        
        v_flex()
            .gap_2()
            .p_3()
            .rounded_md()
            .max_h(px(400.))
            .overflow_y_scroll()
            .when(self.writing_mode == WritingMode::Focus, |this| {
                // 专注模式的样式调整
                this.p_6()
            })
            .child(
                if self.preview_content.is_empty() {
                    Label::new("preview-empty", "选择文件以查看预览")
                        .size(LabelSize::Small)
                } else {
                    // 这里应该集成真正的markdown渲染
                    // 目前使用简单的文本显示
                    v_flex()
                        .gap_2()
                        .children(
                            self.preview_content
                                .split('\n')
                                .enumerate()
                                .filter(|(_, line)| !line.trim().is_empty())
                                .map(|(i, line)| {
                                    Label::new(format!("preview-line-{}", i), line.to_string())
                                        .size(font_size_class)
                                })
                        )
                }
            )
    }
}

impl Default for PreviewView {
    fn default() -> Self {
        Self::new()
    }
}
