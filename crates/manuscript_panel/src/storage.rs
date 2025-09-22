use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::models::{Scene, Character, ManuscriptFile};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManuscriptData {
    pub scenes: Vec<Scene>,
    pub characters: Vec<Character>,
    pub files: Vec<ManuscriptFile>,
    pub metadata: ManuscriptMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManuscriptMetadata {
    pub title: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
    pub total_word_count: u32,
    pub target_word_count: Option<u32>,
    pub genre: String,
    pub status: ManuscriptStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ManuscriptStatus {
    Draft,
    InProgress,
    FirstDraft,
    Revision,
    Editing,
    Completed,
    Published,
}

pub struct StorageManager {
    project_dir: PathBuf,
    data_file: PathBuf,
    backup_dir: PathBuf,
}

impl StorageManager {
    pub fn new(project_dir: PathBuf) -> Self {
        let data_file = project_dir.join(".manuscript").join("data.json");
        let backup_dir = project_dir.join(".manuscript").join("backups");
        
        Self {
            project_dir,
            data_file,
            backup_dir,
        }
    }
    
    pub fn init_project(&self) -> Result<()> {
        let manuscript_dir = self.project_dir.join(".manuscript");
        fs::create_dir_all(&manuscript_dir)?;
        fs::create_dir_all(&self.backup_dir)?;
        
        if !self.data_file.exists() {
            let default_data = ManuscriptData {
                scenes: Vec::new(),
                characters: Vec::new(),
                files: Vec::new(),
                metadata: ManuscriptMetadata {
                    title: "新手稿".to_string(),
                    author: "".to_string(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                    version: "1.0.0".to_string(),
                    total_word_count: 0,
                    target_word_count: None,
                    genre: "".to_string(),
                    status: ManuscriptStatus::Draft,
                },
            };
            self.save_data(&default_data)?;
        }
        
        Ok(())
    }
    
    pub fn load_data(&self) -> Result<ManuscriptData> {
        if !self.data_file.exists() {
            self.init_project()?;
        }
        
        let content = fs::read_to_string(&self.data_file)?;
        let data: ManuscriptData = serde_json::from_str(&content)?;
        Ok(data)
    }
    
    pub fn save_data(&self, data: &ManuscriptData) -> Result<()> {
        if let Some(parent) = self.data_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        if self.data_file.exists() {
            self.create_backup()?;
        }
        
        let mut updated_data = data.clone();
        updated_data.metadata.updated_at = chrono::Utc::now().to_rfc3339();
        updated_data.metadata.total_word_count = self.calculate_total_word_count(&updated_data);
        
        let json_content = serde_json::to_string_pretty(&updated_data)?;
        fs::write(&self.data_file, json_content)?;
        
        Ok(())
    }
    
    pub fn save_scenes(&self, scenes: &[Scene]) -> Result<()> {
        let mut data = self.load_data()?;
        data.scenes = scenes.to_vec();
        self.save_data(&data)
    }
    
    pub fn save_characters(&self, characters: &[Character]) -> Result<()> {
        let mut data = self.load_data()?;
        data.characters = characters.to_vec();
        self.save_data(&data)
    }
    
    fn create_backup(&self) -> Result<()> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_file = self.backup_dir.join(format!("data_{}.json", timestamp));
        
        fs::copy(&self.data_file, backup_file)?;
        self.cleanup_old_backups(10)?;
        
        Ok(())
    }
    
    fn cleanup_old_backups(&self, keep_count: usize) -> Result<()> {
        let mut backup_files: Vec<_> = fs::read_dir(&self.backup_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension()? == "json" {
                    let metadata = entry.metadata().ok()?;
                    let modified = metadata.modified().ok()?;
                    Some((path, modified))
                } else {
                    None
                }
            })
            .collect();
            
        backup_files.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (path, _) in backup_files.into_iter().skip(keep_count) {
            let _ = fs::remove_file(path);
        }
        
        Ok(())
    }
    
    fn calculate_total_word_count(&self, data: &ManuscriptData) -> u32 {
        data.files.iter().map(|file| file.word_count).sum()
    }
    
    pub fn get_project_stats(&self) -> Result<ProjectStats> {
        let data = self.load_data()?;
        
        Ok(ProjectStats {
            total_scenes: data.scenes.len(),
            total_characters: data.characters.len(),
            total_files: data.files.len(),
            total_words: data.metadata.total_word_count,
            target_words: data.metadata.target_word_count,
            completion_percentage: if let Some(target) = data.metadata.target_word_count {
                if target > 0 {
                    Some((data.metadata.total_word_count as f32 / target as f32 * 100.0) as u32)
                } else {
                    None
                }
            } else {
                None
            },
            last_updated: data.metadata.updated_at,
        })
    }
}

#[derive(Debug)]
pub struct ProjectStats {
    pub total_scenes: usize,
    pub total_characters: usize,
    pub total_files: usize,
    pub total_words: u32,
    pub target_words: Option<u32>,
    pub completion_percentage: Option<u32>,
    pub last_updated: String,
}

impl Default for ManuscriptStatus {
    fn default() -> Self {
        ManuscriptStatus::Draft
    }
}
