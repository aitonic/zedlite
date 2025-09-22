use std::path::{Path, PathBuf};
use std::collections::HashMap;
use gpui::{IntoElement, Context};
use ui::{v_flex, h_flex, Label, LabelSize, IconName, IconButton, ButtonSize, prelude::*};
use crate::models::{ManuscriptFile, ManuscriptFileType};
use crate::ManuscriptPanel;

pub struct NavigatorView {
    files: Vec<ManuscriptFile>,
    categorized_files: HashMap<ManuscriptFileType, Vec<ManuscriptFile>>,
}

impl NavigatorView {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            categorized_files: HashMap::new(),
        }
    }
    
    pub fn load_files(&mut self, directory_path: &Path) -> anyhow::Result<()> {
        self.files.clear();
        self.categorized_files.clear();
        
        if let Ok(entries) = std::fs::read_dir(directory_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && self.is_manuscript_file(&path) {
                    let file_type = self.classify_file(&path);
                    let manuscript_file = ManuscriptFile::new(
                        path.to_string_lossy().to_string(),
                        file_type.clone()
                    );
                    
                    self.files.push(manuscript_file.clone());
                    self.categorized_files
                        .entry(file_type)
                        .or_insert_with(Vec::new)
                        .push(manuscript_file);
                }
            }
        }
        
        Ok(())
    }
    
    fn is_manuscript_file(&self, path: &Path) -> bool {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") | Some("txt") | Some("rtf") | Some("docx") => true,
            _ => false,
        }
    }
    
    fn classify_file(&self, path: &Path) -> ManuscriptFileType {
        let filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_lowercase();
            
        let stem = path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // 智能分类规则
        if filename.contains("chapter") || 
           filename.contains("第") && filename.contains("章") ||
           stem.starts_with("ch") ||
           filename.matches(char::is_numeric).count() > 0 && 
           (filename.contains("chapter") || filename.len() < 10) {
            ManuscriptFileType::Chapter
        } else if filename.contains("outline") || 
                  filename.contains("大纲") ||
                  filename.contains("plot") ||
                  filename.contains("structure") {
            ManuscriptFileType::Outline
        } else if filename.contains("character") ||
                  filename.contains("角色") ||
                  filename.contains("人物") ||
                  filename.contains("profile") {
            ManuscriptFileType::CharacterProfile
        } else if filename.contains("scene") ||
                  filename.contains("场景") ||
                  filename.contains("情节") {
            ManuscriptFileType::SceneDescription
        } else if filename.contains("note") ||
                  filename.contains("笔记") ||
                  filename.contains("memo") ||
                  filename.contains("idea") {
            ManuscriptFileType::Notes
        } else if filename.contains("research") ||
                  filename.contains("资料") ||
                  filename.contains("参考") ||
                  filename.contains("reference") {
            ManuscriptFileType::Research
        } else {
            ManuscriptFileType::Other
        }
    }
    
    pub fn render(&self, cx: &mut Context<ManuscriptPanel>) -> impl IntoElement {
        let categories = vec![
            (ManuscriptFileType::Chapter, "章节", IconName::FileText),
            (ManuscriptFileType::Outline, "大纲", IconName::List),
            (ManuscriptFileType::CharacterProfile, "角色", IconName::Person),
            (ManuscriptFileType::SceneDescription, "场景", IconName::Camera),
            (ManuscriptFileType::Notes, "笔记", IconName::FileText),
            (ManuscriptFileType::Research, "资料", IconName::FileSearch),
            (ManuscriptFileType::Other, "其他", IconName::File),
        ];
        
        v_flex()
            .gap_2()
            .children(categories.into_iter().map(|(file_type, label, icon)| {
                self.render_category(&file_type, label, icon, cx)
            }))
    }
    
    fn render_category(
        &self, 
        file_type: &ManuscriptFileType, 
        label: &str,
        icon: IconName,
        _cx: &mut Context<ManuscriptPanel>
    ) -> impl IntoElement {
        let files = self.categorized_files
            .get(file_type)
            .cloned()
            .unwrap_or_default();
            
        let file_count = files.len();
        
        v_flex()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(IconButton::new(format!("category-{:?}", file_type), icon)
                        .size(ButtonSize::Small))
                    .child(Label::new(format!("category-label-{:?}", file_type), format!("{} ({})", label, file_count))
                        .size(LabelSize::Small))
            )
            .children(
                if file_count > 0 {
                    Some(v_flex()
                        .gap_1()
                        .pl_4()
                        .children(files.into_iter().map(|file| {
                            self.render_file_item(file)
                        })))
                } else {
                    None
                }
            )
    }
    
    fn render_file_item(&self, file: ManuscriptFile) -> impl IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .child(IconButton::new(format!("file-{}", file.path), IconName::File)
                .size(ButtonSize::Small))
            .child(Label::new(format!("file-label-{}", file.path), file.title.clone())
                .size(LabelSize::Small))
            .child(Label::new(format!("file-count-{}", file.path), format!("{}字", file.word_count))
                .size(LabelSize::XSmall))
    }
}

impl Default for NavigatorView {
    fn default() -> Self {
        Self::new()
    }
}
