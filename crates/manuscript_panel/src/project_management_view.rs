use std::path::PathBuf;
use gpui::{App, Context, IntoElement, Render, Window, px};
use ui::{
    Button, ButtonSize, ButtonStyle, h_flex, v_flex, Label, LabelSize, 
    prelude::*, IconName, TextInput, Toggle
};
use crate::{
    project_manager::{NovelProjectManager, NovelProject, ProjectStatistics, NovelProjectTemplate},
    file_search::{AdvancedFileSearch, SearchType, SearchFilters, SearchResult, FileType, ContentStatus}
};

/// 导出格式
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Markdown,
    HTML,
    PDF,
    DOCX,
    EPUB,
    TXT,
}

/// 导出内容
#[derive(Debug, Clone)]
pub struct ExportContent {
    pub title: String,
    pub author: String,
    pub chapters: Vec<String>,
    pub total_words: u32,
}

/// 导出结果
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub success: bool,
    pub output_path: Option<PathBuf>,
    pub file_size: Option<u64>,
    pub error: Option<String>,
}

/// 项目管理视图
pub struct ProjectManagementView {
    project_manager: NovelProjectManager,
    file_search: AdvancedFileSearch,
    
    // UI状态
    current_tab: ProjectTab,
    
    // 项目创建
    new_project_name: String,
    new_project_path: String,
    new_project_author: String,
    selected_template: String,
    
    // 搜索状态
    search_query: String,
    search_type: SearchType,
    search_filters: SearchFilters,
    current_search_result: Option<SearchResult>,
    
    // 导出状态
    export_format: ExportFormat,
    export_output_path: String,
    export_include_metadata: bool,
    export_include_toc: bool,
    export_include_cover: bool,
    export_chapter_breaks: bool,
    
    // 界面状态
    show_advanced_search: bool,
    show_project_settings: bool,
    show_export_preview: bool,
}

/// 项目管理标签页
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTab {
    Overview,      // 项目概览
    FileSearch,    // 文件搜索
    Templates,     // 模板管理
    Export,        // 导出发布
    Settings,      // 项目设置
}

impl ProjectManagementView {
    pub fn new() -> Self {
        Self {
            project_manager: NovelProjectManager::new(),
            file_search: AdvancedFileSearch::new(),
            current_tab: ProjectTab::Overview,
            new_project_name: String::new(),
            new_project_path: String::new(),
            new_project_author: String::new(),
            selected_template: "standard_novel".to_string(),
            search_query: String::new(),
            search_type: SearchType::Combined,
            search_filters: SearchFilters::default(),
            current_search_result: None,
            export_format: ExportFormat::PDF,
            export_output_path: String::new(),
            export_include_metadata: true,
            export_include_toc: true,
            export_include_cover: true,
            export_chapter_breaks: true,
            show_advanced_search: false,
            show_project_settings: false,
            show_export_preview: false,
        }
    }
    
    pub fn get_project_manager(&self) -> &NovelProjectManager {
        &self.project_manager
    }
    
    pub fn get_project_manager_mut(&mut self) -> &mut NovelProjectManager {
        &mut self.project_manager
    }
    
    pub fn get_file_search(&self) -> &AdvancedFileSearch {
        &self.file_search
    }
    
    pub fn get_file_search_mut(&mut self) -> &mut AdvancedFileSearch {
        &mut self.file_search
    }
}

impl Render for ProjectManagementView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .padding(px(16.))
            .child(self.render_header(cx))
            .child(self.render_tab_selector(cx))
            .child(self.render_tab_content(cx))
    }
}

impl ProjectManagementView {
    fn render_header(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .justify_between()
            .items_center()
            .child(
                Label::new("Project Management")
                    .size(LabelSize::Large)
            )
            .child(
                self.render_project_status()
            )
    }
    
    fn render_project_status(&self) -> impl IntoElement {
        if let Some(project) = self.project_manager.get_current_project() {
            h_flex()
                .gap_2()
                .child(
                    Label::new(&format!("Project: {}", project.name))
                        .size(LabelSize::Default)
                )
                .child(
                    Label::new(&format!("Status: {:?}", project.metadata.status))
                        .size(LabelSize::Small)
                )
        } else {
            Label::new("No Project Loaded")
                .size(LabelSize::Default)
        }
    }
    
    fn render_tab_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(self.tab_button("Overview", ProjectTab::Overview, cx))
            .child(self.tab_button("Search", ProjectTab::FileSearch, cx))
            .child(self.tab_button("Templates", ProjectTab::Templates, cx))
            .child(self.tab_button("Export", ProjectTab::Export, cx))
            .child(self.tab_button("Settings", ProjectTab::Settings, cx))
    }
    
    fn tab_button(&self, label: &str, tab: ProjectTab, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.current_tab == tab;
        let tab_for_click = tab.clone();
        
        Button::new(format!("tab_{:?}", tab), label)
            .style(if is_active { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Default)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.current_tab = tab_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_tab_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.current_tab {
            ProjectTab::Overview => self.render_overview_tab(cx),
            ProjectTab::FileSearch => self.render_search_tab(cx),
            ProjectTab::Templates => self.render_templates_tab(cx),
            ProjectTab::Export => self.render_export_tab(cx),
            ProjectTab::Settings => self.render_settings_tab(cx),
        }
    }
    
    fn render_overview_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(self.render_current_project_info())
            .child(self.render_recent_projects(cx))
            .child(self.render_project_creation_form(cx))
    }
    
    fn render_current_project_info(&self) -> impl IntoElement {
        if let Some(project) = self.project_manager.get_current_project() {
            v_flex()
                .gap_2()
                .child(
                    Label::new("Current Project")
                        .size(LabelSize::Default)
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new(&format!("Name: {}", project.name)))
                        .child(Label::new(&format!("Author: {}", project.metadata.author)))
                        .child(Label::new(&format!("Word Count: {}", project.metadata.total_word_count)))
                        .child(Label::new(&format!("Progress: {:.1}%", project.metadata.progress_percentage)))
                        .child(Label::new(&format!("Created: {}", project.created_at)))
                )
                .child(self.render_project_statistics())
        } else {
            v_flex()
                .gap_2()
                .child(
                    Label::new("No Project Currently Loaded")
                        .size(LabelSize::Default)
                )
                .child(
                    Label::new("Create a new project or load an existing one to get started.")
                        .size(LabelSize::Small)
                )
        }
    }
    
    fn render_project_statistics(&self) -> impl IntoElement {
        if let Some(stats) = self.project_manager.get_project_statistics() {
            h_flex()
                .gap_4()
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("Files"))
                        .child(Label::new(stats.total_files.to_string()))
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("Chapters"))
                        .child(Label::new(stats.chapter_count.to_string()))
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("Characters"))
                        .child(Label::new(stats.character_count.to_string()))
                )
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("Scenes"))
                        .child(Label::new(stats.scene_count.to_string()))
                )
        } else {
            h_flex().child(Label::new("No statistics available"))
        }
    }
    
    fn render_recent_projects(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let recent_projects = self.project_manager.get_recent_projects();
        
        v_flex()
            .gap_2()
            .child(
                Label::new("Recent Projects")
                    .size(LabelSize::Default)
            )
            .children(
                recent_projects.iter().take(5).map(|project_info| {
                    let project_path = project_info.path.clone();
                    
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
                                .child(Label::new(&project_info.name))
                                .child(
                                    Label::new(&format!("{} words", project_info.word_count))
                                        .size(LabelSize::Small)
                                )
                        )
                        .child(
                            Button::new(format!("load_project_{}", project_info.id), "Load")
                                .size(ButtonSize::Small)
                                .on_click(cx.listener(move |this, _, _window, _cx| {
                                    if let Err(e) = this.project_manager.load_project(project_path.clone()) {
                                        eprintln!("Failed to load project: {}", e);
                                    }
                                }))
                        )
                })
            )
    }
    
    fn render_project_creation_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Create New Project")
                    .size(LabelSize::Default)
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new("Project Name:")
                                    .size(LabelSize::Small)
                            )
                            .child(
                                TextInput::new("project_name")
                                    .placeholder("Enter project name")
                                    .on_input(cx.listener(|this, input, _window, cx| {
                                        this.new_project_name = input;
                                        cx.notify();
                                    }))
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new("Author:")
                                    .size(LabelSize::Small)
                            )
                            .child(
                                TextInput::new("project_author")
                                    .placeholder("Your name")
                                    .on_input(cx.listener(|this, input, _window, cx| {
                                        this.new_project_author = input;
                                        cx.notify();
                                    }))
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new("Location:")
                                    .size(LabelSize::Small)
                            )
                            .child(
                                TextInput::new("project_path")
                                    .placeholder("/path/to/project")
                                    .on_input(cx.listener(|this, input, _window, cx| {
                                        this.new_project_path = input;
                                        cx.notify();
                                    }))
                            )
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new("Template:")
                                    .size(LabelSize::Small)
                            )
                            .child(self.render_template_selector(cx))
                    )
            )
            .child(
                Button::new("create_project", "Create Project")
                    .style(ButtonStyle::Filled)
                    .disabled(self.new_project_name.is_empty() || self.new_project_path.is_empty())
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        let project_path = PathBuf::from(&this.new_project_path);
                        match this.project_manager.create_project(
                            this.new_project_name.clone(),
                            project_path,
                            Some(this.selected_template.clone()),
                            this.new_project_author.clone(),
                        ) {
                            Ok(_project_id) => {
                                // Clear form
                                this.new_project_name.clear();
                                this.new_project_path.clear();
                                this.new_project_author.clear();
                                
                                // 构建文件索引
                                if let Some(project) = this.project_manager.get_current_project() {
                                    let _ = this.file_search.build_index(&project.root_path);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to create project: {}", e);
                            }
                        }
                    }))
            )
    }
    
    fn render_template_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .children(
                self.project_manager
                    .get_available_templates()
                    .into_iter()
                    .map(|template| {
                        let is_selected = self.selected_template == template.id;
                        let template_id = template.id.clone();
                        
                        Button::new(format!("template_{}", template.id), &template.name)
                            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                            .size(ButtonSize::Small)
                            .tooltip(|tooltip| tooltip.text(&template.description))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.selected_template = template_id.clone();
                                cx.notify();
                            }))
                    })
            )
    }
    
    fn render_search_tab(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(self.render_search_interface(cx))
            .child(self.render_search_results())
    }
    
    fn render_search_interface(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        TextInput::new("search_query")
                            .placeholder("Search files...")
                            .on_input(cx.listener(|this, input, _window, cx| {
                                this.search_query = input;
                                cx.notify();
                            }))
                    )
                    .child(
                        Button::new("search_execute", "Search")
                            .style(ButtonStyle::Filled)
                            .disabled(self.search_query.is_empty())
                            .on_click(cx.listener(|this, _, _window, cx| {
                                if !this.search_query.is_empty() {
                                    match this.file_search.search(
                                        this.search_query.clone(),
                                        this.search_type.clone(),
                                        Some(this.search_filters.clone()),
                                    ) {
                                        Ok(result) => {
                                            this.current_search_result = Some(result);
                                        }
                                        Err(e) => {
                                            eprintln!("Search failed: {}", e);
                                        }
                                    }
                                    cx.notify();
                                }
                            }))
                    )
                    .child(
                        Button::new("toggle_advanced", "Advanced")
                            .style(if self.show_advanced_search { 
                                ButtonStyle::Filled 
                            } else { 
                                ButtonStyle::Subtle 
                            })
                            .size(ButtonSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.show_advanced_search = !this.show_advanced_search;
                                cx.notify();
                            }))
                    )
            )
            .child(self.render_search_type_selector(cx))
            .when(self.show_advanced_search, |this| {
                this.child(self.render_advanced_search_filters(cx))
            })
    }
    
    fn render_search_type_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Label::new("Search Type:"))
            .child(self.search_type_button("File Name", SearchType::FileName, cx))
            .child(self.search_type_button("Content", SearchType::FileContent, cx))
            .child(self.search_type_button("Metadata", SearchType::Metadata, cx))
            .child(self.search_type_button("Combined", SearchType::Combined, cx))
            .child(self.search_type_button("Fuzzy", SearchType::Fuzzy, cx))
            .child(self.search_type_button("Regex", SearchType::Regex, cx))
    }
    
    fn search_type_button(&self, label: &str, search_type: SearchType, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = std::mem::discriminant(&self.search_type) == std::mem::discriminant(&search_type);
        let search_type_for_click = search_type.clone();
        
        Button::new(format!("search_type_{:?}", search_type), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.search_type = search_type_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_advanced_search_filters(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Advanced Filters")
                    .size(LabelSize::Default)
            )
            .child(self.render_file_type_filters(cx))
            .child(self.render_content_status_filters(cx))
    }
    
    fn render_file_type_filters(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Label::new("File Types:"))
            .child(self.file_type_button("All", FileType::All, cx))
            .child(self.file_type_button("Chapter", FileType::Chapter, cx))
            .child(self.file_type_button("Character", FileType::Character, cx))
            .child(self.file_type_button("Scene", FileType::Scene, cx))
            .child(self.file_type_button("Research", FileType::Research, cx))
            .child(self.file_type_button("Note", FileType::Note, cx))
    }
    
    fn file_type_button(&self, label: &str, file_type: FileType, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.search_filters.file_types.contains(&file_type);
        let file_type_for_click = file_type.clone();
        
        Button::new(format!("file_type_{:?}", file_type), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                let file_type = file_type_for_click.clone();
                if this.search_filters.file_types.contains(&file_type) {
                    this.search_filters.file_types.retain(|t| t != &file_type);
                } else {
                    this.search_filters.file_types.push(file_type);
                }
                cx.notify();
            }))
    }
    
    fn render_content_status_filters(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Label::new("Status:"))
            .child(self.status_button("Draft", ContentStatus::Draft, cx))
            .child(self.status_button("In Progress", ContentStatus::InProgress, cx))
            .child(self.status_button("Review", ContentStatus::Review, cx))
            .child(self.status_button("Complete", ContentStatus::Complete, cx))
    }
    
    fn status_button(&self, label: &str, status: ContentStatus, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.search_filters.status.contains(&status);
        let status_for_click = status.clone();
        
        Button::new(format!("status_{:?}", status), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                let status = status_for_click.clone();
                if this.search_filters.status.contains(&status) {
                    this.search_filters.status.retain(|s| s != &status);
                } else {
                    this.search_filters.status.push(status);
                }
                cx.notify();
            }))
    }
    
    fn render_search_results(&self) -> impl IntoElement {
        if let Some(result) = &self.current_search_result {
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .justify_between()
                        .child(
                            Label::new(&format!("Found {} results in {}ms", 
                                result.total_matches, result.search_time_ms))
                        )
                        .child(
                            Label::new(&format!("Query: \"{}\"", result.query.query))
                                .size(LabelSize::Small)
                        )
                )
                .children(
                    result.results.iter().take(20).map(|search_match| {
                        v_flex()
                            .gap_2()
                            .padding(px(8.))
                            .border_1()
                            .border_color(ui::colors::border())
                            .rounded(px(4.))
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(
                                        Label::new(&search_match.file.name)
                                            .size(LabelSize::Default)
                                    )
                                    .child(
                                        Label::new(&format!("{:.2}", search_match.relevance_score))
                                            .size(LabelSize::Small)
                                    )
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new(&format!("{:?}", search_match.file.file_type))
                                            .size(LabelSize::Small)
                                    )
                                    .child(
                                        Label::new(&format!("{} words", search_match.file.word_count))
                                            .size(LabelSize::Small)
                                    )
                                    .child(
                                        Label::new(&format!("{:?}", search_match.match_type))
                                            .size(LabelSize::Small)
                                    )
                            )
                            .children(
                                search_match.context_snippets.iter().map(|snippet| {
                                    Label::new(&format!("L{}: {}", snippet.line_number, snippet.content))
                                        .size(LabelSize::Small)
                                })
                            )
                    })
                )
        } else {
            v_flex()
                .child(
                    Label::new("No search results yet.")
                        .size(LabelSize::Default)
                )
                .child(
                    Label::new("Enter a search query and click Search to find files.")
                        .size(LabelSize::Small)
                )
        }
    }
    
    fn render_templates_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Project Templates")
                    .size(LabelSize::Default)
            )
            .children(
                self.project_manager
                    .get_available_templates()
                    .into_iter()
                    .map(|template| {
                        v_flex()
                            .gap_2()
                            .padding(px(12.))
                            .border_1()
                            .border_color(ui::colors::border())
                            .rounded(px(4.))
                            .child(
                                Label::new(&template.name)
                                    .size(LabelSize::Default)
                            )
                            .child(
                                Label::new(&template.description)
                                    .size(LabelSize::Small)
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Label::new(&format!("Type: {:?}", template.template_type))
                                            .size(LabelSize::Small)
                                    )
                                    .child(
                                        Label::new(&format!("Files: {}", template.initial_files.len()))
                                            .size(LabelSize::Small)
                                    )
                            )
                    })
            )
    }
    
    fn render_export_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                Label::new("Export & Publication")
                    .size(LabelSize::Large)
            )
            .child(self.render_export_settings(cx))
            .child(self.render_export_preview(cx))
            .child(self.render_export_actions(cx))
            .child(self.render_export_history())
    }
    
    fn render_export_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Export Settings")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Format:"))
                    .child(self.export_format_button("PDF", ExportFormat::PDF, cx))
                    .child(self.export_format_button("HTML", ExportFormat::HTML, cx))
                    .child(self.export_format_button("Markdown", ExportFormat::Markdown, cx))
                    .child(self.export_format_button("DOCX", ExportFormat::DOCX, cx))
                    .child(self.export_format_button("EPUB", ExportFormat::EPUB, cx))
                    .child(self.export_format_button("TXT", ExportFormat::TXT, cx))
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Output Path:"))
                    .child(
                        TextInput::new("export_path")
                            .placeholder("Enter output file path...")
                            .on_input(cx.listener(|this, input, _window, cx| {
                                this.export_output_path = input;
                                cx.notify();
                            }))
                    )
            )
            .child(self.render_export_options(cx))
    }
    
    fn export_format_button(&self, label: &str, format: ExportFormat, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.export_format == format;
        let format_for_click = format.clone();
        
        Button::new(format!("export_format_{:?}", format), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.export_format = format_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_export_options(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Options")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Include Metadata"))
                    .child(
                        Toggle::new("export_metadata")
                            .checked(self.export_include_metadata)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.export_include_metadata = checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Include Table of Contents"))
                    .child(
                        Toggle::new("export_toc")
                            .checked(self.export_include_toc)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.export_include_toc = checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Include Cover Page"))
                    .child(
                        Toggle::new("export_cover")
                            .checked(self.export_include_cover)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.export_include_cover = checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Chapter Page Breaks"))
                    .child(
                        Toggle::new("export_breaks")
                            .checked(self.export_chapter_breaks)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.export_chapter_breaks = checked;
                                cx.notify();
                            }))
                    )
            )
    }
    
    fn render_export_preview(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("Export Preview")
                            .size(LabelSize::Default)
                    )
                    .child(
                        Button::new("toggle_preview", "Show Preview")
                            .style(if self.show_export_preview { 
                                ButtonStyle::Filled 
                            } else { 
                                ButtonStyle::Subtle 
                            })
                            .size(ButtonSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.show_export_preview = !this.show_export_preview;
                                cx.notify();
                            }))
                    )
            )
            .when(self.show_export_preview, |this| {
                this.child(self.render_preview_content())
            })
    }
    
    fn render_preview_content(&self) -> impl IntoElement {
        v_flex()
            .gap_2()
            .padding(px(12.))
            .border_1()
            .border_color(ui::colors::border())
            .rounded(px(4.))
            .child(
                Label::new("Preview Content")
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(&format!("Format: {:?}", self.export_format))
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(&format!("Include metadata: {}", self.export_include_metadata))
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(&format!("Include TOC: {}", self.export_include_toc))
                    .size(LabelSize::Small)
            )
            .child(
                Label::new("Sample output preview would appear here...")
                    .size(LabelSize::Small)
            )
    }
    
    fn render_export_actions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                Button::new("export_current", "Export Current Project")
                    .style(ButtonStyle::Filled)
                    .disabled(self.export_output_path.is_empty() || self.project_manager.get_current_project().is_none())
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        if let Some(project) = this.project_manager.get_current_project() {
                            match this.perform_export(project) {
                                Ok(_) => {
                                    // 导出成功，可以显示通知
                                }
                                Err(e) => {
                                    eprintln!("Export failed: {}", e);
                                }
                            }
                        }
                    }))
            )
            .child(
                Button::new("export_chapters", "Export Selected Chapters")
                    .style(ButtonStyle::Subtle)
                    .disabled(self.export_output_path.is_empty())
                    .on_click(cx.listener(|_this, _, _window, _cx| {
                        // TODO: 实现章节选择导出
                    }))
            )
            .child(
                Button::new("quick_preview", "Quick Preview")
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.show_export_preview = true;
                        cx.notify();
                    }))
            )
    }
    
    fn render_export_history(&self) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Recent Exports")
                    .size(LabelSize::Default)
            )
            .child(
                Label::new("Export history will be displayed here...")
                    .size(LabelSize::Small)
            )
    }
    
    fn perform_export(&self, project: &NovelProject) -> Result<ExportResult, String> {
        // 创建导出内容
        let export_content = ExportContent {
            title: project.name.clone(),
            author: project.metadata.author.clone(),
            chapters: vec!["Chapter 1".to_string(), "Chapter 2".to_string()], // 简化实现
            total_words: project.metadata.total_word_count,
        };
        
        // 根据格式生成内容
        let content = self.generate_export_content(&export_content)?;
        
        // 写入文件
        std::fs::write(&self.export_output_path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        // 获取文件大小
        let file_size = std::fs::metadata(&self.export_output_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        Ok(ExportResult {
            success: true,
            output_path: Some(PathBuf::from(&self.export_output_path)),
            file_size: Some(file_size),
            error: None,
        })
    }
    
    fn generate_export_content(&self, content: &ExportContent) -> Result<String, String> {
        match self.export_format {
            ExportFormat::Markdown => self.generate_markdown(content),
            ExportFormat::HTML => self.generate_html(content),
            ExportFormat::TXT => self.generate_txt(content),
            ExportFormat::PDF => Err("PDF export requires additional libraries".to_string()),
            ExportFormat::DOCX => Err("DOCX export requires additional libraries".to_string()),
            ExportFormat::EPUB => Err("EPUB export requires additional libraries".to_string()),
        }
    }
    
    fn generate_markdown(&self, content: &ExportContent) -> Result<String, String> {
        let mut output = String::new();
        
        if self.export_include_metadata {
            output.push_str(&format!("# {}\n\n", content.title));
            output.push_str(&format!("**Author:** {}\n\n", content.author));
            output.push_str(&format!("**Total Words:** {}\n\n", content.total_words));
            output.push_str("---\n\n");
        }
        
        if self.export_include_toc && content.chapters.len() > 1 {
            output.push_str("## Table of Contents\n\n");
            for (i, chapter) in content.chapters.iter().enumerate() {
                output.push_str(&format!("{}. [{}](#chapter-{})\n", i + 1, chapter, i + 1));
            }
            output.push_str("\n---\n\n");
        }
        
        for (i, chapter) in content.chapters.iter().enumerate() {
            output.push_str(&format!("## Chapter {} {}\n\n", i + 1, chapter));
            output.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit...\n\n");
            
            if self.export_chapter_breaks && i < content.chapters.len() - 1 {
                output.push_str("---\n\n");
            }
        }
        
        Ok(output)
    }
    
    fn generate_html(&self, content: &ExportContent) -> Result<String, String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>{}</title>\n", content.title));
        html.push_str("<style>\nbody { font-family: 'Times New Roman', serif; line-height: 1.6; }\n");
        html.push_str("h1, h2 { color: #333; }\n");
        html.push_str(".cover { text-align: center; page-break-after: always; }\n");
        html.push_str(".chapter { page-break-before: always; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        
        if self.export_include_cover {
            html.push_str("<div class=\"cover\">\n");
            html.push_str(&format!("<h1>{}</h1>\n", content.title));
            html.push_str(&format!("<h2>by {}</h2>\n", content.author));
            html.push_str("</div>\n");
        }
        
        if self.export_include_toc && content.chapters.len() > 1 {
            html.push_str("<div class=\"toc\">\n<h2>Table of Contents</h2>\n<ul>\n");
            for (i, chapter) in content.chapters.iter().enumerate() {
                html.push_str(&format!("<li><a href=\"#chapter-{}\">{}</a></li>\n", i + 1, chapter));
            }
            html.push_str("</ul>\n</div>\n");
        }
        
        for (i, chapter) in content.chapters.iter().enumerate() {
            html.push_str("<div class=\"chapter\">\n");
            html.push_str(&format!("<h2 id=\"chapter-{}\">Chapter {} {}</h2>\n", i + 1, i + 1, chapter));
            html.push_str("<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit...</p>\n");
            html.push_str("</div>\n");
        }
        
        html.push_str("</body>\n</html>");
        Ok(html)
    }
    
    fn generate_txt(&self, content: &ExportContent) -> Result<String, String> {
        let mut output = String::new();
        
        output.push_str(&format!("{}\n", content.title));
        output.push_str(&format!("by {}\n\n", content.author));
        output.push_str(&"=".repeat(50));
        output.push_str("\n\n");
        
        for (i, chapter) in content.chapters.iter().enumerate() {
            output.push_str(&format!("Chapter {} {}\n\n", i + 1, chapter));
            output.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit...\n\n");
            output.push_str(&"-".repeat(30));
            output.push_str("\n\n");
        }
        
        Ok(output)
    }

    fn render_settings_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                Label::new("Project Settings")
                    .size(LabelSize::Default)
            )
            .child(
                Button::new("toggle_settings", "Show Project Settings")
                    .style(if self.show_project_settings { 
                        ButtonStyle::Filled 
                    } else { 
                        ButtonStyle::Subtle 
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.show_project_settings = !this.show_project_settings;
                        cx.notify();
                    }))
            )
            .when(self.show_project_settings && self.project_manager.get_current_project().is_some(), |this| {
                this.child(self.render_project_settings(cx))
            })
    }
    
    fn render_project_settings(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(project) = self.project_manager.get_current_project() {
            v_flex()
                .gap_3()
                .child(
                    Label::new("Word Count Targets")
                        .size(LabelSize::Default)
                )
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Daily"))
                                .child(Label::new(&format!("{}", 
                                    project.settings.word_count_targets.daily_target.unwrap_or(0))))
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Weekly"))
                                .child(Label::new(&format!("{}", 
                                    project.settings.word_count_targets.weekly_target.unwrap_or(0))))
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Project"))
                                .child(Label::new(&format!("{}", 
                                    project.settings.word_count_targets.project_target.unwrap_or(0))))
                        )
                )
                .child(
                    Label::new("Export Settings")
                        .size(LabelSize::Default)
                )
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Format"))
                                .child(Label::new(&format!("{:?}", project.settings.export_settings.default_format)))
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Include Metadata"))
                                .child(Label::new(&format!("{}", project.settings.export_settings.include_metadata)))
                        )
                )
        } else {
            Label::new("No project loaded")
        }
    }
}

impl Default for ProjectManagementView {
    fn default() -> Self {
        Self::new()
    }
}
