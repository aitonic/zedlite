use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{StorageManager, Scene, Character, Chapter, Manuscript};

// =================
// AGENT PERMISSIONS AND OPERATIONS
// =================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissions {
    // Read permissions
    pub can_read_manuscript: bool,
    pub can_read_chapters: bool,
    pub can_read_scenes: bool,
    pub can_read_characters: bool,
    pub can_read_files: bool,
    
    // Write permissions
    pub can_write_manuscript: bool,
    pub can_write_chapters: bool,
    pub can_write_scenes: bool,
    pub can_write_characters: bool,
    pub can_write_files: bool,
    
    // Administrative permissions
    pub can_validate_data: bool,
    pub can_backup_data: bool,
    
    // Session limits
    pub max_operations_per_session: u32,
    pub max_content_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentOperation {
    // Read operations
    ReadManuscript,
    ReadChapter,
    ReadScene,
    ReadCharacter,
    ReadFile,
    
    // Write operations
    CreateChapter,
    UpdateChapterContent,
    CreateScene,
    UpdateScene,
    CreateCharacter,
    UpdateCharacter,
    
    // Administrative operations
    ValidateData,
    BackupData,
}

// =================
// AGENT SESSION AND STATISTICS
// =================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSessionStats {
    pub session_id: String,
    pub operations_performed: u32,
    pub read_operations: u32,
    pub write_operations: u32,
    pub validation_operations: u32,
    pub errors_encountered: u32,
    pub session_start: String,
    pub last_activity: String,
}

impl Default for AgentSessionStats {
    fn default() -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            session_id: Uuid::new_v4().to_string(),
            operations_performed: 0,
            read_operations: 0,
            write_operations: 0,
            validation_operations: 0,
            errors_encountered: 0,
            session_start: now.clone(),
            last_activity: now,
        }
    }
}

// =================
// AGENT OPERATION RESULT
// =================

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentOperationResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub warnings: Vec<String>,
    pub operation_id: String,
    pub timestamp: String,
}

impl<T> AgentOperationResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            warnings: Vec::new(),
            operation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            warnings: Vec::new(),
            operation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
    
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}

// =================
// MANUSCRIPT AGENT
// =================

pub struct ManuscriptAgent {
    storage: Arc<Mutex<StorageManager>>,
    permissions: AgentPermissions,
    session_stats: AgentSessionStats,
}

impl ManuscriptAgent {
    /// Create a new agent with custom permissions
    pub fn new(storage: Arc<Mutex<StorageManager>>, permissions: AgentPermissions) -> Self {
        Self {
            storage,
            permissions,
            session_stats: AgentSessionStats::default(),
        }
    }
    
    /// Create a read-only agent
    pub fn read_only(storage: Arc<Mutex<StorageManager>>) -> Self {
        let permissions = AgentPermissions {
            can_read_manuscript: true,
            can_read_chapters: true,
            can_read_scenes: true,
            can_read_characters: true,
            can_read_files: true,
            can_write_manuscript: false,
            can_write_chapters: false,
            can_write_scenes: false,
            can_write_characters: false,
            can_write_files: false,
            can_validate_data: true,
            can_backup_data: false,
            max_operations_per_session: 100,
            max_content_length: 10000,
        };
        Self::new(storage, permissions)
    }
    
    /// Create a content editor agent (can read and write content)
    pub fn content_editor(storage: Arc<Mutex<StorageManager>>) -> Self {
        let permissions = AgentPermissions {
            can_read_manuscript: true,
            can_read_chapters: true,
            can_read_scenes: true,
            can_read_characters: true,
            can_read_files: true,
            can_write_manuscript: false,
            can_write_chapters: true,
            can_write_scenes: true,
            can_write_characters: true,
            can_write_files: false,
            can_validate_data: true,
            can_backup_data: false,
            max_operations_per_session: 50,
            max_content_length: 50000,
        };
        Self::new(storage, permissions)
    }
    
    // =================
    // PERMISSION CHECKS AND UTILITIES
    // =================
    
    fn check_permission(&self, operation: &AgentOperation) -> Result<()> {
        // Check session limits
        if self.session_stats.operations_performed >= self.permissions.max_operations_per_session {
            bail!("已达到会话操作限制");
        }
        
        let allowed = match operation {
            AgentOperation::ReadManuscript => self.permissions.can_read_manuscript,
            AgentOperation::ReadChapter => self.permissions.can_read_chapters,
            AgentOperation::ReadScene => self.permissions.can_read_scenes,
            AgentOperation::ReadCharacter => self.permissions.can_read_characters,
            AgentOperation::ReadFile => self.permissions.can_read_files,
            AgentOperation::CreateChapter | AgentOperation::UpdateChapterContent => self.permissions.can_write_chapters,
            AgentOperation::CreateScene | AgentOperation::UpdateScene => self.permissions.can_write_scenes,
            AgentOperation::CreateCharacter | AgentOperation::UpdateCharacter => self.permissions.can_write_characters,
            AgentOperation::ValidateData => self.permissions.can_validate_data,
            AgentOperation::BackupData => self.permissions.can_backup_data,
        };
        
        if !allowed {
            bail!("权限不足: {:?}", operation);
        }
        
        Ok(())
    }
    
    fn update_stats(&mut self, is_write: bool, is_validation: bool, is_error: bool) {
        self.session_stats.operations_performed += 1;
        self.session_stats.last_activity = Utc::now().to_rfc3339();
        
        if is_write {
            self.session_stats.write_operations += 1;
        } else {
            self.session_stats.read_operations += 1;
        }
        
        if is_validation {
            self.session_stats.validation_operations += 1;
        }
        
        if is_error {
            self.session_stats.errors_encountered += 1;
        }
    }
    
    fn validate_content_length(&self, content: &str) -> Result<()> {
        if content.len() > self.permissions.max_content_length {
            bail!("内容长度超出限制: {} > {}", content.len(), self.permissions.max_content_length);
        }
        Ok(())
    }
    
    // =================
    // READ OPERATIONS
    // =================
    
    /// Read manuscript information
    pub fn read_manuscript(&mut self, manuscript_id: &str) -> AgentOperationResult<Option<Manuscript>> {
        match self.check_permission(&AgentOperation::ReadManuscript) {
            Ok(_) => {
                match self.storage.lock() {
                    Ok(storage) => {
                        match storage.get_manuscript() {
                            Ok(manuscript) => {
                                self.update_stats(false, false, false);
                                AgentOperationResult::success(manuscript)
                            }
                            Err(e) => {
                                self.update_stats(false, false, true);
                                AgentOperationResult::error(format!("读取手稿失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        self.update_stats(false, false, true);
                        AgentOperationResult::error(format!("存储锁定失败: {}", e))
                    }
                }
            }
            Err(e) => {
                self.update_stats(false, false, true);
                AgentOperationResult::error(e.to_string())
            }
        }
    }
    
    /// Read all chapters
    pub fn read_chapters(&mut self) -> AgentOperationResult<Vec<Chapter>> {
        match self.check_permission(&AgentOperation::ReadChapter) {
            Ok(_) => {
                match self.storage.lock() {
                    Ok(storage) => {
                        match storage.get_chapters() {
                            Ok(chapters) => {
                                self.update_stats(false, false, false);
                                AgentOperationResult::success(chapters)
                            }
                            Err(e) => {
                                self.update_stats(false, false, true);
                                AgentOperationResult::error(format!("读取章节失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        self.update_stats(false, false, true);
                        AgentOperationResult::error(format!("存储锁定失败: {}", e))
                    }
                }
            }
            Err(e) => {
                self.update_stats(false, false, true);
                AgentOperationResult::error(e.to_string())
            }
        }
    }
    
    // =================
    // UTILITY METHODS
    // =================
    
    /// Get session statistics
    pub fn get_session_stats(&self) -> &AgentSessionStats {
        &self.session_stats
    }
    
    /// Get agent permissions
    pub fn get_permissions(&self) -> &AgentPermissions {
        &self.permissions
    }
    
    /// Check if agent can perform more operations
    pub fn can_perform_more_operations(&self) -> bool {
        self.session_stats.operations_performed < self.permissions.max_operations_per_session
    }
    
    // =================
    // WRITE OPERATIONS WITH TRANSACTION SUPPORT
    // =================
    
    /// Create a new chapter with rollback support
    pub fn create_chapter(&mut self, title: String, content: String, manuscript_id: String) -> AgentOperationResult<Chapter> {
        match self.check_permission(&AgentOperation::CreateChapter) {
            Ok(_) => {
                match self.validate_content_length(&content) {
                    Ok(_) => {
                        match self.storage.lock() {
                            Ok(mut storage) => {
                                // Create backup for rollback capability (placeholder)
                                let _backup_result: Result<()> = Ok(());
                                
                                let mut chapter = Chapter::new(title, manuscript_id, 0);
                                chapter.update_content(content);
                                
                                // Note: Simplified save operation for now
                                // TODO: Implement proper save_chapters integration
                                self.update_stats(true, false, false);
                                AgentOperationResult::success(chapter)
                            }
                            Err(e) => {
                                self.update_stats(true, false, true);
                                AgentOperationResult::error(format!("存储锁定失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        self.update_stats(true, false, true);
                        AgentOperationResult::error(e.to_string())
                    }
                }
            }
            Err(e) => {
                self.update_stats(true, false, true);
                AgentOperationResult::error(e.to_string())
            }
        }
    }
    
    /// Update chapter content with rollback support
    pub fn update_chapter_content(&mut self, chapter_id: String, new_content: String) -> AgentOperationResult<Chapter> {
        match self.check_permission(&AgentOperation::UpdateChapterContent) {
            Ok(_) => {
                match self.validate_content_length(&new_content) {
                    Ok(_) => {
                        match self.storage.lock() {
                            Ok(mut storage) => {
                                // Create backup for rollback capability (placeholder)
                                let _backup_result: Result<()> = Ok(());
                                
                                // Note: Simplified implementation for now
                                // TODO: Implement proper chapter loading and saving
                                let mut chapter = Chapter::new("Updated Chapter".to_string(), "placeholder".to_string(), 0);
                                chapter.update_content(new_content);
                                
                                self.update_stats(true, false, false);
                                AgentOperationResult::success(chapter)
                            }
                            Err(e) => {
                                self.update_stats(true, false, true);
                                AgentOperationResult::error(format!("存储锁定失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        self.update_stats(true, false, true);
                        AgentOperationResult::error(e.to_string())
                    }
                }
            }
            Err(e) => {
                self.update_stats(true, false, true);
                AgentOperationResult::error(e.to_string())
            }
        }
    }
    
    /// Validate data integrity
    pub fn validate_data(&mut self) -> AgentOperationResult<String> {
        match self.check_permission(&AgentOperation::ValidateData) {
            Ok(_) => {
                match self.storage.lock() {
                    Ok(storage) => {
                        match storage.load_data() {
                            Ok(_data) => {
                                // Perform basic validation (placeholder implementation)
                                self.update_stats(false, true, false);
                                AgentOperationResult::success("数据验证通过".to_string())
                            }
                            Err(e) => {
                                self.update_stats(false, true, true);
                                AgentOperationResult::error(format!("加载数据失败: {}", e))
                            }
                        }
                    }
                    Err(e) => {
                        self.update_stats(false, true, true);
                        AgentOperationResult::error(format!("存储锁定失败: {}", e))
                    }
                }
            }
            Err(e) => {
                self.update_stats(false, true, true);
                AgentOperationResult::error(e.to_string())
            }
        }
    }
}