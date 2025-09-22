use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 小说写作项目管理器
#[derive(Debug, Clone)]
pub struct NovelProjectManager {
    current_project: Option<NovelProject>,
    recent_projects: Vec<NovelProjectInfo>,
    project_templates: HashMap<String, NovelProjectTemplate>,
}

/// 小说项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub root_path: PathBuf,
    pub structure: ProjectStructure,
    pub settings: ProjectSettings,
    pub metadata: ProjectMetadata,
    pub created_at: String,
    pub last_modified: String,
}

/// 项目结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub manuscript_folder: String,
    pub chapters_folder: String,
    pub characters_folder: String,
    pub research_folder: String,
    pub notes_folder: String,
    pub drafts_folder: String,
    pub exports_folder: String,
    pub assets_folder: String,
    pub custom_folders: Vec<CustomFolder>,
}

/// 自定义文件夹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFolder {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub folder_type: CustomFolderType,
}

/// 自定义文件夹类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomFolderType {
    Research,
    Reference,
    Planning,
    Archive,
    Custom(String),
}

/// 项目设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub auto_backup: bool,
    pub backup_interval_minutes: u32,
    pub version_control: bool,
    pub word_count_targets: WordCountTargets,
    pub writing_schedule: WritingSchedule,
    pub export_settings: ExportSettings,
}

/// 字数目标设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCountTargets {
    pub daily_target: Option<u32>,
    pub weekly_target: Option<u32>,
    pub monthly_target: Option<u32>,
    pub project_target: Option<u32>,
}

/// 写作计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingSchedule {
    pub enabled: bool,
    pub daily_hours: Option<f32>,
    pub preferred_time_slots: Vec<TimeSlot>,
    pub writing_days: Vec<Weekday>,
}

/// 时间段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start_hour: u8,
    pub end_hour: u8,
    pub description: Option<String>,
}

/// 星期几
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// 导出设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub default_format: ExportFormat,
    pub include_metadata: bool,
    pub include_statistics: bool,
    pub custom_formatting: HashMap<String, String>,
}

/// 导出格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Markdown,
    HTML,
    PDF,
    DOCX,
    TXT,
    EPUB,
}

/// 项目元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub author: String,
    pub genre: Option<String>,
    pub target_audience: Option<String>,
    pub status: ProjectStatus,
    pub progress_percentage: f32,
    pub total_word_count: u32,
    pub chapter_count: u32,
    pub character_count: u32,
    pub scene_count: u32,
    pub tags: Vec<String>,
}

/// 项目状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    FirstDraft,
    Revision,
    Editing,
    Proofreading,
    Complete,
    OnHold,
    Abandoned,
}

/// 项目信息摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelProjectInfo {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub last_accessed: String,
    pub word_count: u32,
    pub status: ProjectStatus,
}

/// 项目模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelProjectTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub structure: ProjectStructure,
    pub initial_files: Vec<TemplateFile>,
    pub settings: ProjectSettings,
    pub template_type: TemplateType,
}

/// 模板文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
    pub file_type: TemplateFileType,
}

/// 模板文件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateFileType {
    Chapter,
    Character,
    Scene,
    Research,
    Note,
    Outline,
}

/// 模板类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    Novel,
    ShortStory,
    Series,
    Screenplay,
    Custom(String),
}

impl NovelProjectManager {
    /// 创建新的项目管理器
    pub fn new() -> Self {
        let mut manager = Self {
            current_project: None,
            recent_projects: Vec::new(),
            project_templates: HashMap::new(),
        };
        
        manager.register_default_templates();
        manager
    }
    
    /// 注册默认项目模板
    fn register_default_templates(&mut self) {
        // 标准小说模板
        self.project_templates.insert(
            "standard_novel".to_string(),
            NovelProjectTemplate {
                id: "standard_novel".to_string(),
                name: "Standard Novel".to_string(),
                description: "适用于长篇小说的标准项目结构".to_string(),
                structure: ProjectStructure {
                    manuscript_folder: "manuscript".to_string(),
                    chapters_folder: "chapters".to_string(),
                    characters_folder: "characters".to_string(),
                    research_folder: "research".to_string(),
                    notes_folder: "notes".to_string(),
                    drafts_folder: "drafts".to_string(),
                    exports_folder: "exports".to_string(),
                    assets_folder: "assets".to_string(),
                    custom_folders: vec![
                        CustomFolder {
                            name: "Timeline".to_string(),
                            path: "timeline".to_string(),
                            description: Some("故事时间线".to_string()),
                            folder_type: CustomFolderType::Planning,
                        },
                        CustomFolder {
                            name: "World Building".to_string(),
                            path: "worldbuilding".to_string(),
                            description: Some("世界设定资料".to_string()),
                            folder_type: CustomFolderType::Reference,
                        },
                    ],
                },
                initial_files: vec![
                    TemplateFile {
                        path: "manuscript/synopsis.md".to_string(),
                        content: "# Synopsis\n\n## 故事概述\n\n## 主要角色\n\n## 主题\n\n".to_string(),
                        file_type: TemplateFileType::Outline,
                    },
                    TemplateFile {
                        path: "characters/protagonist.md".to_string(),
                        content: "# 主角\n\n## 基本信息\n- 姓名：\n- 年龄：\n- 职业：\n\n## 性格特征\n\n## 背景故事\n\n## 角色弧线\n\n".to_string(),
                        file_type: TemplateFileType::Character,
                    },
                ],
                settings: ProjectSettings {
                    auto_backup: true,
                    backup_interval_minutes: 30,
                    version_control: true,
                    word_count_targets: WordCountTargets {
                        daily_target: Some(1000),
                        weekly_target: Some(7000),
                        monthly_target: Some(30000),
                        project_target: Some(80000),
                    },
                    writing_schedule: WritingSchedule {
                        enabled: false,
                        daily_hours: Some(2.0),
                        preferred_time_slots: vec![],
                        writing_days: vec![],
                    },
                    export_settings: ExportSettings {
                        default_format: ExportFormat::Markdown,
                        include_metadata: true,
                        include_statistics: true,
                        custom_formatting: HashMap::new(),
                    },
                },
                template_type: TemplateType::Novel,
            },
        );
        
        // 短篇小说模板
        self.project_templates.insert(
            "short_story".to_string(),
            NovelProjectTemplate {
                id: "short_story".to_string(),
                name: "Short Story".to_string(),
                description: "适用于短篇小说的简化项目结构".to_string(),
                structure: ProjectStructure {
                    manuscript_folder: "story".to_string(),
                    chapters_folder: "sections".to_string(),
                    characters_folder: "characters".to_string(),
                    research_folder: "research".to_string(),
                    notes_folder: "notes".to_string(),
                    drafts_folder: "drafts".to_string(),
                    exports_folder: "exports".to_string(),
                    assets_folder: "assets".to_string(),
                    custom_folders: vec![],
                },
                initial_files: vec![
                    TemplateFile {
                        path: "story/main.md".to_string(),
                        content: "# 故事标题\n\n".to_string(),
                        file_type: TemplateFileType::Chapter,
                    },
                ],
                settings: ProjectSettings {
                    auto_backup: true,
                    backup_interval_minutes: 15,
                    version_control: false,
                    word_count_targets: WordCountTargets {
                        daily_target: Some(500),
                        weekly_target: None,
                        monthly_target: None,
                        project_target: Some(5000),
                    },
                    writing_schedule: WritingSchedule {
                        enabled: false,
                        daily_hours: Some(1.0),
                        preferred_time_slots: vec![],
                        writing_days: vec![],
                    },
                    export_settings: ExportSettings {
                        default_format: ExportFormat::Markdown,
                        include_metadata: false,
                        include_statistics: false,
                        custom_formatting: HashMap::new(),
                    },
                },
                template_type: TemplateType::ShortStory,
            },
        );
    }
    
    /// 创建新项目
    pub fn create_project(
        &mut self,
        name: String,
        root_path: PathBuf,
        template_id: Option<String>,
        author: String,
    ) -> Result<String> {
        let project_id = Uuid::new_v4().to_string();
        let template = template_id
            .and_then(|id| self.project_templates.get(&id))
            .cloned()
            .unwrap_or_else(|| self.project_templates.get("standard_novel").unwrap().clone());
        
        let project = NovelProject {
            id: project_id.clone(),
            name: name.clone(),
            description: None,
            root_path: root_path.clone(),
            structure: template.structure,
            settings: template.settings,
            metadata: ProjectMetadata {
                author,
                genre: None,
                target_audience: None,
                status: ProjectStatus::Planning,
                progress_percentage: 0.0,
                total_word_count: 0,
                chapter_count: 0,
                character_count: 0,
                scene_count: 0,
                tags: Vec::new(),
            },
            created_at: Utc::now().to_rfc3339(),
            last_modified: Utc::now().to_rfc3339(),
        };
        
        // 创建项目文件夹结构
        self.create_project_structure(&project, &template)?;
        
        // 设置为当前项目
        self.current_project = Some(project.clone());
        
        // 添加到最近项目列表
        self.add_to_recent_projects(&project);
        
        Ok(project_id)
    }
    
    /// 创建项目文件夹结构
    fn create_project_structure(&self, project: &NovelProject, template: &NovelProjectTemplate) -> Result<()> {
        use std::fs;
        
        // 创建根目录
        fs::create_dir_all(&project.root_path)?;
        
        // 创建标准文件夹
        let structure = &project.structure;
        let folders = [
            &structure.manuscript_folder,
            &structure.chapters_folder,
            &structure.characters_folder,
            &structure.research_folder,
            &structure.notes_folder,
            &structure.drafts_folder,
            &structure.exports_folder,
            &structure.assets_folder,
        ];
        
        for folder in folders {
            let folder_path = project.root_path.join(folder);
            fs::create_dir_all(folder_path)?;
        }
        
        // 创建自定义文件夹
        for custom_folder in &structure.custom_folders {
            let folder_path = project.root_path.join(&custom_folder.path);
            fs::create_dir_all(folder_path)?;
        }
        
        // 创建初始文件
        for template_file in &template.initial_files {
            let file_path = project.root_path.join(&template_file.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(file_path, &template_file.content)?;
        }
        
        Ok(())
    }
    
    /// 加载现有项目
    pub fn load_project(&mut self, project_path: PathBuf) -> Result<()> {
        let project_file = project_path.join(".novel_project.json");
        
        if !project_file.exists() {
            bail!("项目文件不存在: {}", project_file.display());
        }
        
        let content = std::fs::read_to_string(project_file)?;
        let mut project: NovelProject = serde_json::from_str(&content)?;
        
        // 更新最后访问时间
        project.last_modified = Utc::now().to_rfc3339();
        
        self.current_project = Some(project.clone());
        self.add_to_recent_projects(&project);
        
        Ok(())
    }
    
    /// 保存当前项目
    pub fn save_current_project(&self) -> Result<()> {
        if let Some(project) = &self.current_project {
            let project_file = project.root_path.join(".novel_project.json");
            let content = serde_json::to_string_pretty(project)?;
            std::fs::write(project_file, content)?;
        }
        Ok(())
    }
    
    /// 添加到最近项目列表
    fn add_to_recent_projects(&mut self, project: &NovelProject) {
        let project_info = NovelProjectInfo {
            id: project.id.clone(),
            name: project.name.clone(),
            path: project.root_path.clone(),
            last_accessed: Utc::now().to_rfc3339(),
            word_count: project.metadata.total_word_count,
            status: project.metadata.status.clone(),
        };
        
        // 移除已存在的相同项目
        self.recent_projects.retain(|p| p.id != project.id);
        
        // 添加到列表开头
        self.recent_projects.insert(0, project_info);
        
        // 保持最多10个最近项目
        if self.recent_projects.len() > 10 {
            self.recent_projects.truncate(10);
        }
    }
    
    /// 获取当前项目
    pub fn get_current_project(&self) -> Option<&NovelProject> {
        self.current_project.as_ref()
    }
    
    /// 获取最近项目列表
    pub fn get_recent_projects(&self) -> &[NovelProjectInfo] {
        &self.recent_projects
    }
    
    /// 获取可用模板
    pub fn get_available_templates(&self) -> Vec<&NovelProjectTemplate> {
        self.project_templates.values().collect()
    }
    
    /// 更新项目元数据
    pub fn update_project_metadata<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut ProjectMetadata),
    {
        if let Some(project) = &mut self.current_project {
            updater(&mut project.metadata);
            project.last_modified = Utc::now().to_rfc3339();
            self.save_current_project()?;
        }
        Ok(())
    }
    
    /// 获取项目统计信息
    pub fn get_project_statistics(&self) -> Option<ProjectStatistics> {
        self.current_project.as_ref().map(|project| {
            ProjectStatistics {
                total_files: 0, // TODO: 实际统计文件数量
                total_word_count: project.metadata.total_word_count,
                chapter_count: project.metadata.chapter_count,
                character_count: project.metadata.character_count,
                scene_count: project.metadata.scene_count,
                progress_percentage: project.metadata.progress_percentage,
                daily_progress: 0, // TODO: 计算每日进度
                weekly_progress: 0, // TODO: 计算每周进度
            }
        })
    }
}

/// 项目统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub total_files: u32,
    pub total_word_count: u32,
    pub chapter_count: u32,
    pub character_count: u32,
    pub scene_count: u32,
    pub progress_percentage: f32,
    pub daily_progress: u32,
    pub weekly_progress: u32,
}

impl Default for NovelProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ProjectStructure {
    fn default() -> Self {
        Self {
            manuscript_folder: "manuscript".to_string(),
            chapters_folder: "chapters".to_string(),
            characters_folder: "characters".to_string(),
            research_folder: "research".to_string(),
            notes_folder: "notes".to_string(),
            drafts_folder: "drafts".to_string(),
            exports_folder: "exports".to_string(),
            assets_folder: "assets".to_string(),
            custom_folders: Vec::new(),
        }
    }
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            auto_backup: true,
            backup_interval_minutes: 30,
            version_control: false,
            word_count_targets: WordCountTargets::default(),
            writing_schedule: WritingSchedule::default(),
            export_settings: ExportSettings::default(),
        }
    }
}

impl Default for WordCountTargets {
    fn default() -> Self {
        Self {
            daily_target: Some(1000),
            weekly_target: Some(7000),
            monthly_target: Some(30000),
            project_target: Some(80000),
        }
    }
}

impl Default for WritingSchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            daily_hours: Some(2.0),
            preferred_time_slots: Vec::new(),
            writing_days: Vec::new(),
        }
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::Markdown,
            include_metadata: true,
            include_statistics: true,
            custom_formatting: HashMap::new(),
        }
    }
}
