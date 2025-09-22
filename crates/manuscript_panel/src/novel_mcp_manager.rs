use std::sync::Arc;
use std::path::Path;
use anyhow::{Result, bail};
use collections::HashMap;
use serde::{Deserialize, Serialize};
use gpui::{App, Task, Entity};
use project::Project;
use agent_servers::AgentServer;
use crate::novel_agent_profiles::novel_mcp_servers;

/// 小说写作MCP服务器管理器
#[derive(Debug, Clone)]
pub struct NovelMcpManager {
    available_servers: HashMap<String, NovelMcpServerConfig>,
    active_servers: HashMap<String, Arc<dyn AgentServer>>,
}

/// 小说写作MCP服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelMcpServerConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    pub enabled: bool,
    pub auto_start: bool,
    pub server_type: NovelMcpServerType,
}

/// 小说写作MCP服务器类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NovelMcpServerType {
    /// 研究和资料收集服务器
    Research,
    /// 大纲和结构服务器
    Outline,
    /// 编辑和校对服务器
    Editing,
    /// 角色发展服务器
    Character,
    /// 世界构建服务器
    WorldBuilding,
    /// 时间线管理服务器
    Timeline,
}

impl NovelMcpManager {
    /// 创建新的MCP管理器
    pub fn new() -> Self {
        let mut manager = Self {
            available_servers: HashMap::new(),
            active_servers: HashMap::new(),
        };
        
        manager.register_default_servers();
        manager
    }
    
    /// 注册默认的小说写作MCP服务器
    fn register_default_servers(&mut self) {
        // 研究服务器
        self.available_servers.insert(
            novel_mcp_servers::RESEARCH_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::RESEARCH_SERVER.to_string(),
                name: "Novel Research Server".to_string(),
                description: "提供资料收集、事实核查和文献管理功能".to_string(),
                command: "novel-research-server".to_string(),
                args: vec!["--mode".to_string(), "research".to_string()],
                env: None,
                enabled: true,
                auto_start: true,
                server_type: NovelMcpServerType::Research,
            },
        );
        
        // 大纲服务器
        self.available_servers.insert(
            novel_mcp_servers::OUTLINE_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::OUTLINE_SERVER.to_string(),
                name: "Novel Outline Server".to_string(),
                description: "提供故事结构、情节规划和大纲管理功能".to_string(),
                command: "novel-outline-server".to_string(),
                args: vec!["--mode".to_string(), "outline".to_string()],
                env: None,
                enabled: true,
                auto_start: true,
                server_type: NovelMcpServerType::Outline,
            },
        );
        
        // 编辑服务器
        self.available_servers.insert(
            novel_mcp_servers::EDITING_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::EDITING_SERVER.to_string(),
                name: "Novel Editing Server".to_string(),
                description: "提供文本编辑、风格检查和一致性校对功能".to_string(),
                command: "novel-editing-server".to_string(),
                args: vec!["--mode".to_string(), "editing".to_string()],
                env: None,
                enabled: true,
                auto_start: false,
                server_type: NovelMcpServerType::Editing,
            },
        );
        
        // 角色发展服务器
        self.available_servers.insert(
            novel_mcp_servers::CHARACTER_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::CHARACTER_SERVER.to_string(),
                name: "Character Development Server".to_string(),
                description: "提供角色创建、关系管理和对话优化功能".to_string(),
                command: "character-dev-server".to_string(),
                args: vec!["--mode".to_string(), "character".to_string()],
                env: None,
                enabled: true,
                auto_start: false,
                server_type: NovelMcpServerType::Character,
            },
        );
        
        // 世界构建服务器
        self.available_servers.insert(
            novel_mcp_servers::WORLD_BUILDING_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::WORLD_BUILDING_SERVER.to_string(),
                name: "World Building Server".to_string(),
                description: "提供世界设定、地点管理和设定一致性检查功能".to_string(),
                command: "world-building-server".to_string(),
                args: vec!["--mode".to_string(), "worldbuilding".to_string()],
                env: None,
                enabled: false,
                auto_start: false,
                server_type: NovelMcpServerType::WorldBuilding,
            },
        );
        
        // 时间线管理服务器
        self.available_servers.insert(
            novel_mcp_servers::TIMELINE_SERVER.to_string(),
            NovelMcpServerConfig {
                id: novel_mcp_servers::TIMELINE_SERVER.to_string(),
                name: "Timeline Management Server".to_string(),
                description: "提供事件时间线、情节时序和叙事节奏管理功能".to_string(),
                command: "timeline-server".to_string(),
                args: vec!["--mode".to_string(), "timeline".to_string()],
                env: None,
                enabled: false,
                auto_start: false,
                server_type: NovelMcpServerType::Timeline,
            },
        );
    }
    
    /// 获取所有可用的服务器配置
    pub fn get_available_servers(&self) -> &HashMap<String, NovelMcpServerConfig> {
        &self.available_servers
    }
    
    /// 获取特定类型的服务器
    pub fn get_servers_by_type(&self, server_type: NovelMcpServerType) -> Vec<&NovelMcpServerConfig> {
        self.available_servers
            .values()
            .filter(|config| std::mem::discriminant(&config.server_type) == std::mem::discriminant(&server_type))
            .collect()
    }
    
    /// 启用服务器
    pub fn enable_server(&mut self, server_id: &str) -> Result<()> {
        if let Some(config) = self.available_servers.get_mut(server_id) {
            config.enabled = true;
            Ok(())
        } else {
            bail!("服务器不存在: {}", server_id);
        }
    }
    
    /// 禁用服务器
    pub fn disable_server(&mut self, server_id: &str) -> Result<()> {
        if let Some(config) = self.available_servers.get_mut(server_id) {
            config.enabled = false;
            // 如果服务器正在运行，停止它
            self.active_servers.remove(server_id);
            Ok(())
        } else {
            bail!("服务器不存在: {}", server_id);
        }
    }
    
    /// 检查服务器是否已启用
    pub fn is_server_enabled(&self, server_id: &str) -> bool {
        self.available_servers
            .get(server_id)
            .map(|config| config.enabled)
            .unwrap_or(false)
    }
    
    /// 检查服务器是否正在运行
    pub fn is_server_running(&self, server_id: &str) -> bool {
        self.active_servers.contains_key(server_id)
    }
    
    /// 获取推荐的服务器配置 (基于agent配置文件)
    pub fn get_recommended_servers_for_profile(&self, profile_id: &str) -> Vec<&NovelMcpServerConfig> {
        use crate::novel_agent_profiles::novel_profiles;
        
        match profile_id {
            novel_profiles::NOVELIST => {
                // 小说家需要编辑、大纲和研究服务器
                vec![
                    novel_mcp_servers::EDITING_SERVER,
                    novel_mcp_servers::OUTLINE_SERVER,
                    novel_mcp_servers::RESEARCH_SERVER,
                ]
            }
            novel_profiles::RESEARCHER => {
                // 研究员主要需要研究服务器
                vec![novel_mcp_servers::RESEARCH_SERVER]
            }
            novel_profiles::EDITOR => {
                // 编辑主要需要编辑服务器
                vec![novel_mcp_servers::EDITING_SERVER]
            }
            novel_profiles::OUTLINER => {
                // 大纲师需要大纲和时间线服务器
                vec![
                    novel_mcp_servers::OUTLINE_SERVER,
                    novel_mcp_servers::TIMELINE_SERVER,
                ]
            }
            novel_profiles::CHARACTER_DEVELOPER => {
                // 角色发展师需要角色和世界构建服务器
                vec![
                    novel_mcp_servers::CHARACTER_SERVER,
                    novel_mcp_servers::WORLD_BUILDING_SERVER,
                ]
            }
            _ => Vec::new(),
        }
        .into_iter()
        .filter_map(|server_id| self.available_servers.get(server_id))
        .collect()
    }
    
    /// 自动配置服务器 (基于agent配置文件)
    pub fn auto_configure_for_profile(&mut self, profile_id: &str) -> Result<()> {
        let recommended_servers = self.get_recommended_servers_for_profile(profile_id);
        
        // 禁用所有服务器
        for config in self.available_servers.values_mut() {
            config.enabled = false;
        }
        
        // 启用推荐的服务器
        for server_config in recommended_servers {
            if let Some(config) = self.available_servers.get_mut(&server_config.id) {
                config.enabled = true;
                config.auto_start = true;
            }
        }
        
        Ok(())
    }
    
    /// 启动所有已启用的自动启动服务器
    pub async fn start_auto_servers(&mut self, project: &Entity<Project>, cx: &mut App) -> Result<()> {
        for (server_id, config) in &self.available_servers {
            if config.enabled && config.auto_start && !self.active_servers.contains_key(server_id) {
                if let Err(e) = self.start_server(server_id, project, cx).await {
                    eprintln!("启动服务器失败 {}: {}", server_id, e);
                }
            }
        }
        Ok(())
    }
    
    /// 启动特定服务器
    pub async fn start_server(&mut self, server_id: &str, project: &Entity<Project>, cx: &mut App) -> Result<()> {
        if self.active_servers.contains_key(server_id) {
            return Ok(()); // 服务器已经在运行
        }
        
        let config = self.available_servers
            .get(server_id)
            .ok_or_else(|| anyhow::anyhow!("服务器配置不存在: {}", server_id))?;
        
        if !config.enabled {
            bail!("服务器未启用: {}", server_id);
        }
        
        // 这里可以添加实际的服务器启动逻辑
        // 目前作为占位符实现
        println!("启动MCP服务器: {} ({})", config.name, config.command);
        
        Ok(())
    }
    
    /// 停止特定服务器
    pub fn stop_server(&mut self, server_id: &str) -> Result<()> {
        if let Some(_server) = self.active_servers.remove(server_id) {
            println!("停止MCP服务器: {}", server_id);
            Ok(())
        } else {
            bail!("服务器未运行: {}", server_id);
        }
    }
    
    /// 停止所有服务器
    pub fn stop_all_servers(&mut self) {
        let server_ids: Vec<String> = self.active_servers.keys().cloned().collect();
        for server_id in server_ids {
            let _ = self.stop_server(&server_id);
        }
    }
    
    /// 获取服务器状态摘要
    pub fn get_status_summary(&self) -> NovelMcpStatus {
        let mut enabled_count = 0;
        let mut running_count = 0;
        let mut auto_start_count = 0;
        
        for config in self.available_servers.values() {
            if config.enabled {
                enabled_count += 1;
            }
            if config.auto_start {
                auto_start_count += 1;
            }
        }
        
        running_count = self.active_servers.len();
        
        NovelMcpStatus {
            total_servers: self.available_servers.len(),
            enabled_servers: enabled_count,
            running_servers: running_count,
            auto_start_servers: auto_start_count,
        }
    }
}

/// MCP服务器状态摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelMcpStatus {
    pub total_servers: usize,
    pub enabled_servers: usize,
    pub running_servers: usize,
    pub auto_start_servers: usize,
}

impl Default for NovelMcpManager {
    fn default() -> Self {
        Self::new()
    }
}
