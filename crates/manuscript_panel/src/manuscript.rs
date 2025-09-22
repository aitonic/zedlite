use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manuscript {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub genre: String,
    pub target_word_count: Option<u32>,
    pub current_word_count: u32,
    pub status: ManuscriptStatus,
    pub chapters: Vec<String>, // chapter IDs in order
    pub characters: Vec<String>, // character IDs
    pub scenes: Vec<String>, // scene IDs
    pub research_notes: Vec<String>, // research note IDs
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: ManuscriptMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub manuscript_id: String,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub order_index: u32,
    pub word_count: u32,
    pub character_count: u32,
    pub scenes: Vec<String>, // scene IDs that occur in this chapter
    pub characters: Vec<String>, // character IDs that appear in this chapter
    pub status: ChapterStatus,
    pub notes: String, // author notes for this chapter
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub outline_points: Vec<String>, // key plot points for this chapter
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManuscriptMetadata {
    pub version: String,
    pub language: String,
    pub primary_pov: Option<String>, // primary point of view character
    pub setting: String,
    pub time_period: String,
    pub themes: Vec<String>,
    pub completion_percentage: f32,
    pub last_backup: Option<String>,
    pub writing_session_count: u32,
    pub total_writing_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ManuscriptStatus {
    Planning,     // 策划阶段
    Outlining,    // 大纲阶段
    Drafting,     // 初稿写作
    FirstDraft,   // 第一稿完成
    Revising,     // 修改阶段
    Editing,      // 编辑阶段
    Proofreading, // 校对阶段
    Completed,    // 完成
    Published,    // 已发布
    Archived,     // 已归档
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChapterStatus {
    Planned,      // 已规划
    Outlined,     // 已列大纲
    Drafting,     // 写作中
    FirstDraft,   // 初稿完成
    Reviewing,    // 审阅中
    Revising,     // 修改中
    Polished,     // 已完善
    Finalized,    // 已定稿
}

impl Manuscript {
    pub fn new(title: String, author: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            author,
            description: String::new(),
            genre: String::new(),
            target_word_count: None,
            current_word_count: 0,
            status: ManuscriptStatus::Planning,
            chapters: Vec::new(),
            characters: Vec::new(),
            scenes: Vec::new(),
            research_notes: Vec::new(),
            tags: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
            metadata: ManuscriptMetadata::default(),
        }
    }
    
    pub fn add_chapter(&mut self, chapter_id: String) {
        if !self.chapters.contains(&chapter_id) {
            self.chapters.push(chapter_id);
            self.update_timestamp();
        }
    }
    
    pub fn remove_chapter(&mut self, chapter_id: &str) {
        self.chapters.retain(|id| id != chapter_id);
        self.update_timestamp();
    }
    
    pub fn reorder_chapters(&mut self, new_order: Vec<String>) {
        if new_order.iter().all(|id| self.chapters.contains(id)) && 
           new_order.len() == self.chapters.len() {
            self.chapters = new_order;
            self.update_timestamp();
        }
    }
    
    pub fn add_character(&mut self, character_id: String) {
        if !self.characters.contains(&character_id) {
            self.characters.push(character_id);
            self.update_timestamp();
        }
    }
    
    pub fn add_scene(&mut self, scene_id: String) {
        if !self.scenes.contains(&scene_id) {
            self.scenes.push(scene_id);
            self.update_timestamp();
        }
    }
    
    pub fn add_research_note(&mut self, note_id: String) {
        if !self.research_notes.contains(&note_id) {
            self.research_notes.push(note_id);
            self.update_timestamp();
        }
    }
    
    pub fn update_word_count(&mut self, word_count: u32) {
        self.current_word_count = word_count;
        self.metadata.completion_percentage = if let Some(target) = self.target_word_count {
            if target > 0 {
                (word_count as f32 / target as f32 * 100.0).min(100.0)
            } else {
                0.0
            }
        } else {
            0.0
        };
        self.update_timestamp();
    }
    
    pub fn get_progress_percentage(&self) -> f32 {
        self.metadata.completion_percentage
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ManuscriptStatus::Completed | ManuscriptStatus::Published)
    }
    
    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.update_timestamp();
        }
    }
    
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.update_timestamp();
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("手稿标题不能为空".to_string());
        }
        
        if self.author.trim().is_empty() {
            return Err("作者姓名不能为空".to_string());
        }
        
        if let Some(target) = self.target_word_count {
            if target == 0 {
                return Err("目标字数必须大于0".to_string());
            }
        }
        
        Ok(())
    }
}

impl Chapter {
    pub fn new(title: String, manuscript_id: String, order_index: u32) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            manuscript_id,
            title,
            content: String::new(),
            summary: String::new(),
            order_index,
            word_count: 0,
            character_count: 0,
            scenes: Vec::new(),
            characters: Vec::new(),
            status: ChapterStatus::Planned,
            notes: String::new(),
            tags: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
            outline_points: Vec::new(),
        }
    }
    
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.calculate_statistics();
        self.update_timestamp();
    }
    
    pub fn calculate_statistics(&mut self) {
        // Calculate word count (Chinese characters + English words)
        let chinese_chars = self.content.chars().filter(|c| {
            let cp = *c as u32;
            (cp >= 0x4E00 && cp <= 0x9FFF) || // CJK Unified Ideographs
            (cp >= 0x3400 && cp <= 0x4DBF) || // CJK Extension A
            (cp >= 0x20000 && cp <= 0x2A6DF)  // CJK Extension B
        }).count() as u32;
        
        let english_words = self.content
            .split_whitespace()
            .filter(|word| word.chars().any(|c| c.is_ascii_alphabetic()))
            .count() as u32;
            
        self.word_count = chinese_chars + english_words;
        self.character_count = self.content.chars().count() as u32;
    }
    
    pub fn add_scene(&mut self, scene_id: String) {
        if !self.scenes.contains(&scene_id) {
            self.scenes.push(scene_id);
            self.update_timestamp();
        }
    }
    
    pub fn remove_scene(&mut self, scene_id: &str) {
        self.scenes.retain(|id| id != scene_id);
        self.update_timestamp();
    }
    
    pub fn add_character(&mut self, character_id: String) {
        if !self.characters.contains(&character_id) {
            self.characters.push(character_id);
            self.update_timestamp();
        }
    }
    
    pub fn remove_character(&mut self, character_id: &str) {
        self.characters.retain(|id| id != character_id);
        self.update_timestamp();
    }
    
    pub fn add_outline_point(&mut self, point: String) {
        self.outline_points.push(point);
        self.update_timestamp();
    }
    
    pub fn update_status(&mut self, status: ChapterStatus) {
        self.status = status;
        self.update_timestamp();
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ChapterStatus::Polished | ChapterStatus::Finalized)
    }
    
    pub fn get_estimated_reading_time(&self) -> u32 {
        // Estimate reading time in minutes (Chinese: 250 chars/min, English: 200 words/min)
        let chinese_chars = self.content.chars().filter(|c| {
            let cp = *c as u32;
            (cp >= 0x4E00 && cp <= 0x9FFF) || 
            (cp >= 0x3400 && cp <= 0x4DBF) || 
            (cp >= 0x20000 && cp <= 0x2A6DF)
        }).count() as u32;
        
        let english_words = self.content
            .split_whitespace()
            .filter(|word| word.chars().any(|c| c.is_ascii_alphabetic()))
            .count() as u32;
            
        let chinese_time = chinese_chars as f32 / 250.0;
        let english_time = english_words as f32 / 200.0;
        (chinese_time + english_time).ceil() as u32
    }
    
    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.update_timestamp();
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("章节标题不能为空".to_string());
        }
        
        if self.manuscript_id.trim().is_empty() {
            return Err("章节必须关联到手稿".to_string());
        }
        
        Ok(())
    }
}

impl Default for ManuscriptMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            language: "zh-CN".to_string(),
            primary_pov: None,
            setting: String::new(),
            time_period: String::new(),
            themes: Vec::new(),
            completion_percentage: 0.0,
            last_backup: None,
            writing_session_count: 0,
            total_writing_time_minutes: 0,
        }
    }
}

impl Default for ManuscriptStatus {
    fn default() -> Self {
        ManuscriptStatus::Planning
    }
}

impl Default for ChapterStatus {
    fn default() -> Self {
        ChapterStatus::Planned
    }
}
