use std::sync::Arc;
use collections::IndexMap;
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use agent_settings::{AgentProfileId, AgentProfileSettings, ContextServerPreset};

/// 专为小说写作设计的Agent配置文件
pub mod novel_profiles {
    use super::*;

    // 小说写作专用配置文件ID
    pub const NOVELIST: &str = "novelist";
    pub const RESEARCHER: &str = "researcher";
    pub const EDITOR: &str = "editor";
    pub const OUTLINER: &str = "outliner";
    pub const CHARACTER_DEVELOPER: &str = "character_developer";

    /// 检查是否为小说写作配置文件
    pub fn is_novel_profile(profile_id: &AgentProfileId) -> bool {
        match profile_id.as_str() {
            NOVELIST | RESEARCHER | EDITOR | OUTLINER | CHARACTER_DEVELOPER => true,
            _ => false,
        }
    }

    /// 获取所有小说写作配置文件
    pub fn get_all_novel_profiles() -> Vec<AgentProfileId> {
        vec![
            AgentProfileId(NOVELIST.into()),
            AgentProfileId(RESEARCHER.into()),
            AgentProfileId(EDITOR.into()),
            AgentProfileId(OUTLINER.into()),
            AgentProfileId(CHARACTER_DEVELOPER.into()),
        ]
    }
}

/// 小说写作MCP服务器配置
pub mod novel_mcp_servers {
    pub const RESEARCH_SERVER: &str = "novel_research";
    pub const OUTLINE_SERVER: &str = "novel_outline";
    pub const EDITING_SERVER: &str = "novel_editing";
    pub const CHARACTER_SERVER: &str = "character_development";
    pub const WORLD_BUILDING_SERVER: &str = "world_building";
    pub const TIMELINE_SERVER: &str = "timeline_management";
}

/// 小说写作工具配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelWritingTools {
    // 基础文件操作
    pub read_file: bool,
    pub edit_file: bool,
    pub create_file: bool,
    
    // 小说写作专用工具
    pub manuscript_manager: bool,
    pub chapter_organizer: bool,
    pub character_tracker: bool,
    pub scene_builder: bool,
    pub timeline_manager: bool,
    
    // 研究和资料工具
    pub research_database: bool,
    pub source_manager: bool,
    pub fact_checker: bool,
    pub reference_linker: bool,
    
    // 编辑和校对工具
    pub prose_analyzer: bool,
    pub style_checker: bool,
    pub consistency_checker: bool,
    pub grammar_helper: bool,
    
    // 大纲和结构工具
    pub plot_outliner: bool,
    pub story_structure: bool,
    pub arc_tracker: bool,
    pub pacing_analyzer: bool,
    
    // 角色发展工具
    pub character_profiler: bool,
    pub relationship_mapper: bool,
    pub dialogue_coach: bool,
    pub character_arc_tracker: bool,
}

impl Default for NovelWritingTools {
    fn default() -> Self {
        Self {
            // 基础文件操作默认启用
            read_file: true,
            edit_file: true,
            create_file: true,
            
            // 小说写作工具默认启用
            manuscript_manager: true,
            chapter_organizer: true,
            character_tracker: true,
            scene_builder: true,
            timeline_manager: true,
            
            // 其他工具按需启用
            research_database: false,
            source_manager: false,
            fact_checker: false,
            reference_linker: false,
            prose_analyzer: false,
            style_checker: false,
            consistency_checker: false,
            grammar_helper: false,
            plot_outliner: false,
            story_structure: false,
            arc_tracker: false,
            pacing_analyzer: false,
            character_profiler: false,
            relationship_mapper: false,
            dialogue_coach: false,
            character_arc_tracker: false,
        }
    }
}

impl NovelWritingTools {
    /// 转换为Agent配置工具集
    pub fn to_agent_tools(&self) -> IndexMap<Arc<str>, bool> {
        let mut tools = IndexMap::new();
        
        // 基础文件操作
        tools.insert("read_file".into(), self.read_file);
        tools.insert("edit_file".into(), self.edit_file);
        tools.insert("create_file".into(), self.create_file);
        
        // 小说写作专用工具
        tools.insert("manuscript_manager".into(), self.manuscript_manager);
        tools.insert("chapter_organizer".into(), self.chapter_organizer);
        tools.insert("character_tracker".into(), self.character_tracker);
        tools.insert("scene_builder".into(), self.scene_builder);
        tools.insert("timeline_manager".into(), self.timeline_manager);
        
        // 研究和资料工具
        tools.insert("research_database".into(), self.research_database);
        tools.insert("source_manager".into(), self.source_manager);
        tools.insert("fact_checker".into(), self.fact_checker);
        tools.insert("reference_linker".into(), self.reference_linker);
        
        // 编辑和校对工具
        tools.insert("prose_analyzer".into(), self.prose_analyzer);
        tools.insert("style_checker".into(), self.style_checker);
        tools.insert("consistency_checker".into(), self.consistency_checker);
        tools.insert("grammar_helper".into(), self.grammar_helper);
        
        // 大纲和结构工具
        tools.insert("plot_outliner".into(), self.plot_outliner);
        tools.insert("story_structure".into(), self.story_structure);
        tools.insert("arc_tracker".into(), self.arc_tracker);
        tools.insert("pacing_analyzer".into(), self.pacing_analyzer);
        
        // 角色发展工具
        tools.insert("character_profiler".into(), self.character_profiler);
        tools.insert("relationship_mapper".into(), self.relationship_mapper);
        tools.insert("dialogue_coach".into(), self.dialogue_coach);
        tools.insert("character_arc_tracker".into(), self.character_arc_tracker);
        
        tools
    }
}

/// 预定义的小说写作Agent配置文件
pub struct NovelAgentProfilesBuilder;

impl NovelAgentProfilesBuilder {
    /// 创建小说家配置文件 - 全功能写作助手
    pub fn novelist_profile() -> AgentProfileSettings {
        let mut tools = NovelWritingTools::default();
        tools.manuscript_manager = true;
        tools.chapter_organizer = true;
        tools.character_tracker = true;
        tools.scene_builder = true;
        tools.prose_analyzer = true;
        tools.style_checker = true;
        
        AgentProfileSettings {
            name: "Novelist".into(),
            tools: tools.to_agent_tools(),
            enable_all_context_servers: false,
            context_servers: Self::novelist_context_servers(),
        }
    }
    
    /// 创建研究员配置文件 - 专注于资料收集和事实核查
    pub fn researcher_profile() -> AgentProfileSettings {
        let mut tools = NovelWritingTools::default();
        tools.research_database = true;
        tools.source_manager = true;
        tools.fact_checker = true;
        tools.reference_linker = true;
        
        AgentProfileSettings {
            name: "Researcher".into(),
            tools: tools.to_agent_tools(),
            enable_all_context_servers: false,
            context_servers: Self::researcher_context_servers(),
        }
    }
    
    /// 创建编辑配置文件 - 专注于文本校对和风格
    pub fn editor_profile() -> AgentProfileSettings {
        let mut tools = NovelWritingTools::default();
        tools.prose_analyzer = true;
        tools.style_checker = true;
        tools.consistency_checker = true;
        tools.grammar_helper = true;
        
        AgentProfileSettings {
            name: "Editor".into(),
            tools: tools.to_agent_tools(),
            enable_all_context_servers: false,
            context_servers: Self::editor_context_servers(),
        }
    }
    
    /// 创建大纲师配置文件 - 专注于故事结构和情节
    pub fn outliner_profile() -> AgentProfileSettings {
        let mut tools = NovelWritingTools::default();
        tools.plot_outliner = true;
        tools.story_structure = true;
        tools.arc_tracker = true;
        tools.pacing_analyzer = true;
        tools.timeline_manager = true;
        
        AgentProfileSettings {
            name: "Outliner".into(),
            tools: tools.to_agent_tools(),
            enable_all_context_servers: false,
            context_servers: Self::outliner_context_servers(),
        }
    }
    
    /// 创建角色发展师配置文件 - 专注于角色创建和发展
    pub fn character_developer_profile() -> AgentProfileSettings {
        let mut tools = NovelWritingTools::default();
        tools.character_profiler = true;
        tools.relationship_mapper = true;
        tools.dialogue_coach = true;
        tools.character_arc_tracker = true;
        tools.character_tracker = true;
        
        AgentProfileSettings {
            name: "Character Developer".into(),
            tools: tools.to_agent_tools(),
            enable_all_context_servers: false,
            context_servers: Self::character_developer_context_servers(),
        }
    }
    
    // Context Servers配置
    
    fn novelist_context_servers() -> IndexMap<Arc<str>, ContextServerPreset> {
        let mut servers = IndexMap::new();
        
        servers.insert(
            novel_mcp_servers::EDITING_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("edit_chapter".into(), true),
                    ("check_consistency".into(), true),
                    ("analyze_prose".into(), true),
                ]),
            },
        );
        
        servers.insert(
            novel_mcp_servers::OUTLINE_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("create_outline".into(), true),
                    ("track_plot".into(), true),
                ]),
            },
        );
        
        servers
    }
    
    fn researcher_context_servers() -> IndexMap<Arc<str>, ContextServerPreset> {
        let mut servers = IndexMap::new();
        
        servers.insert(
            novel_mcp_servers::RESEARCH_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("search_sources".into(), true),
                    ("fact_check".into(), true),
                    ("create_reference".into(), true),
                    ("manage_bibliography".into(), true),
                ]),
            },
        );
        
        servers
    }
    
    fn editor_context_servers() -> IndexMap<Arc<str>, ContextServerPreset> {
        let mut servers = IndexMap::new();
        
        servers.insert(
            novel_mcp_servers::EDITING_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("analyze_prose".into(), true),
                    ("check_grammar".into(), true),
                    ("style_suggestions".into(), true),
                    ("consistency_check".into(), true),
                ]),
            },
        );
        
        servers
    }
    
    fn outliner_context_servers() -> IndexMap<Arc<str>, ContextServerPreset> {
        let mut servers = IndexMap::new();
        
        servers.insert(
            novel_mcp_servers::OUTLINE_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("create_outline".into(), true),
                    ("analyze_structure".into(), true),
                    ("track_plot".into(), true),
                    ("pacing_analysis".into(), true),
                ]),
            },
        );
        
        servers.insert(
            novel_mcp_servers::TIMELINE_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("manage_timeline".into(), true),
                    ("track_events".into(), true),
                ]),
            },
        );
        
        servers
    }
    
    fn character_developer_context_servers() -> IndexMap<Arc<str>, ContextServerPreset> {
        let mut servers = IndexMap::new();
        
        servers.insert(
            novel_mcp_servers::CHARACTER_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("create_character".into(), true),
                    ("develop_character".into(), true),
                    ("track_relationships".into(), true),
                    ("analyze_dialogue".into(), true),
                ]),
            },
        );
        
        servers.insert(
            novel_mcp_servers::WORLD_BUILDING_SERVER.into(),
            ContextServerPreset {
                tools: IndexMap::from_iter([
                    ("build_world".into(), true),
                    ("track_locations".into(), true),
                ]),
            },
        );
        
        servers
    }
}
