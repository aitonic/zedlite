use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use regex::Regex;

/// 高级文件搜索系统
#[derive(Debug, Clone)]
pub struct AdvancedFileSearch {
    search_history: Vec<SearchQuery>,
    file_index: FileIndex,
    search_filters: SearchFilters,
}

/// 搜索查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub id: String,
    pub query: String,
    pub search_type: SearchType,
    pub filters: SearchFilters,
    pub timestamp: String,
    pub results_count: usize,
}

/// 搜索类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    FileName,        // 文件名搜索
    FileContent,     // 文件内容搜索
    Metadata,        // 元数据搜索
    Combined,        // 组合搜索
    Fuzzy,          // 模糊搜索
    Regex,          // 正则表达式搜索
}

/// 搜索过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub file_types: Vec<FileType>,
    pub date_range: Option<DateRange>,
    pub size_range: Option<SizeRange>,
    pub word_count_range: Option<WordCountRange>,
    pub tags: Vec<String>,
    pub status: Vec<ContentStatus>,
    pub folders: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

/// 文件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Markdown,
    Text,
    Chapter,
    Character,
    Scene,
    Research,
    Note,
    Outline,
    Draft,
    Template,
    Export,
    All,
}

/// 日期范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: Option<String>,
    pub end: Option<String>,
}

/// 文件大小范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeRange {
    pub min_bytes: Option<u64>,
    pub max_bytes: Option<u64>,
}

/// 字数范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordCountRange {
    pub min_words: Option<u32>,
    pub max_words: Option<u32>,
}

/// 内容状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentStatus {
    Draft,
    InProgress,
    Review,
    Complete,
    Archived,
}

/// 文件索引
#[derive(Debug, Clone)]
pub struct FileIndex {
    files: HashMap<PathBuf, FileMetadata>,
    content_index: HashMap<String, Vec<PathBuf>>,  // word -> files
    tag_index: HashMap<String, Vec<PathBuf>>,      // tag -> files
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub word_count: u32,
    pub character_count: u32,
    pub line_count: u32,
    pub created_at: String,
    pub modified_at: String,
    pub tags: Vec<String>,
    pub status: ContentStatus,
    pub description: Option<String>,
    pub checksum: String,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: SearchQuery,
    pub results: Vec<SearchMatch>,
    pub total_matches: usize,
    pub search_time_ms: u64,
    pub suggestions: Vec<String>,
}

/// 搜索匹配
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub file: FileMetadata,
    pub relevance_score: f32,
    pub match_type: MatchType,
    pub context_snippets: Vec<ContextSnippet>,
}

/// 匹配类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    FileName,
    FileContent,
    Metadata,
    Tag,
}

/// 上下文片段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnippet {
    pub line_number: u32,
    pub content: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

impl AdvancedFileSearch {
    /// 创建新的文件搜索系统
    pub fn new() -> Self {
        Self {
            search_history: Vec::new(),
            file_index: FileIndex::new(),
            search_filters: SearchFilters::default(),
        }
    }
    
    /// 建立文件索引
    pub fn build_index(&mut self, root_path: &Path) -> Result<()> {
        self.file_index.clear();
        self.scan_directory(root_path)?;
        Ok(())
    }
    
    /// 扫描目录并建立索引
    fn scan_directory(&mut self, dir_path: &Path) -> Result<()> {
        use std::fs;
        
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.scan_directory(&path)?;
            } else if self.should_index_file(&path) {
                if let Ok(metadata) = self.extract_file_metadata(&path) {
                    self.file_index.add_file(metadata)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 判断是否应该索引文件
    fn should_index_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            match extension.to_str() {
                Some("md") | Some("txt") | Some("markdown") => true,
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// 提取文件元数据
    fn extract_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        use std::fs;
        
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;
        
        let word_count = self.count_words(&content);
        let character_count = content.chars().count() as u32;
        let line_count = content.lines().count() as u32;
        
        let file_type = self.detect_file_type(path, &content);
        let tags = self.extract_tags(&content);
        let status = self.detect_content_status(&content);
        
        Ok(FileMetadata {
            path: path.to_path_buf(),
            name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            file_type,
            size: metadata.len(),
            word_count,
            character_count,
            line_count,
            created_at: chrono::DateTime::<chrono::Utc>::from(metadata.created().unwrap_or(std::time::SystemTime::now())).to_rfc3339(),
            modified_at: chrono::DateTime::<chrono::Utc>::from(metadata.modified().unwrap_or(std::time::SystemTime::now())).to_rfc3339(),
            tags,
            status,
            description: self.extract_description(&content),
            checksum: self.calculate_checksum(&content),
        })
    }
    
    /// 计算字数
    fn count_words(&self, content: &str) -> u32 {
        content
            .split_whitespace()
            .filter(|word| !word.is_empty())
            .count() as u32
    }
    
    /// 检测文件类型
    fn detect_file_type(&self, path: &Path, content: &str) -> FileType {
        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        
        if filename.contains("chapter") || filename.contains("ch_") {
            FileType::Chapter
        } else if filename.contains("character") || content.contains("# Character:") {
            FileType::Character
        } else if filename.contains("scene") || content.contains("# Scene:") {
            FileType::Scene
        } else if filename.contains("research") || content.contains("# Research:") {
            FileType::Research
        } else if filename.contains("note") || content.contains("# Note:") {
            FileType::Note
        } else if filename.contains("outline") || content.contains("# Outline:") {
            FileType::Outline
        } else if filename.contains("draft") {
            FileType::Draft
        } else if filename.contains("template") {
            FileType::Template
        } else if path.extension().map(|e| e.to_str()) == Some(Some("md")) {
            FileType::Markdown
        } else {
            FileType::Text
        }
    }
    
    /// 提取标签
    fn extract_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        // 提取 #tag 格式的标签
        if let Ok(regex) = Regex::new(r"#(\w+)") {
            for cap in regex.captures_iter(content) {
                if let Some(tag) = cap.get(1) {
                    tags.push(tag.as_str().to_string());
                }
            }
        }
        
        // 提取 tags: 字段
        if let Ok(regex) = Regex::new(r"(?i)tags?\s*:\s*(.+)") {
            for cap in regex.captures_iter(content) {
                if let Some(tags_str) = cap.get(1) {
                    for tag in tags_str.as_str().split(',') {
                        tags.push(tag.trim().to_string());
                    }
                }
            }
        }
        
        tags.sort();
        tags.dedup();
        tags
    }
    
    /// 检测内容状态
    fn detect_content_status(&self, content: &str) -> ContentStatus {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("status: draft") || content_lower.contains("[draft]") {
            ContentStatus::Draft
        } else if content_lower.contains("status: in progress") || content_lower.contains("[wip]") {
            ContentStatus::InProgress
        } else if content_lower.contains("status: review") || content_lower.contains("[review]") {
            ContentStatus::Review
        } else if content_lower.contains("status: complete") || content_lower.contains("[complete]") {
            ContentStatus::Complete
        } else if content_lower.contains("status: archived") || content_lower.contains("[archived]") {
            ContentStatus::Archived
        } else if content.trim().is_empty() || content.len() < 100 {
            ContentStatus::Draft
        } else {
            ContentStatus::InProgress
        }
    }
    
    /// 提取描述
    fn extract_description(&self, content: &str) -> Option<String> {
        // 尝试提取第一段非标题内容作为描述
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') && !line.starts_with("---") {
                if line.len() > 10 && line.len() < 200 {
                    return Some(line.to_string());
                }
            }
        }
        None
    }
    
    /// 计算校验和
    fn calculate_checksum(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// 执行搜索
    pub fn search(&mut self, query: String, search_type: SearchType, filters: Option<SearchFilters>) -> Result<SearchResult> {
        let start_time = std::time::Instant::now();
        
        let filters = filters.unwrap_or_else(|| self.search_filters.clone());
        let query_id = uuid::Uuid::new_v4().to_string();
        
        let mut matches = Vec::new();
        
        // 根据搜索类型执行不同的搜索策略
        match search_type {
            SearchType::FileName => {
                matches.extend(self.search_file_names(&query, &filters)?);
            }
            SearchType::FileContent => {
                matches.extend(self.search_file_content(&query, &filters)?);
            }
            SearchType::Metadata => {
                matches.extend(self.search_metadata(&query, &filters)?);
            }
            SearchType::Combined => {
                matches.extend(self.search_file_names(&query, &filters)?);
                matches.extend(self.search_file_content(&query, &filters)?);
                matches.extend(self.search_metadata(&query, &filters)?);
            }
            SearchType::Fuzzy => {
                matches.extend(self.fuzzy_search(&query, &filters)?);
            }
            SearchType::Regex => {
                matches.extend(self.regex_search(&query, &filters)?);
            }
        }
        
        // 去重和排序
        matches.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        matches.dedup_by(|a, b| a.file.path == b.file.path);
        
        let search_time = start_time.elapsed().as_millis() as u64;
        let total_matches = matches.len();
        
        // 创建搜索查询记录
        let search_query = SearchQuery {
            id: query_id,
            query: query.clone(),
            search_type,
            filters: filters.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            results_count: total_matches,
        };
        
        // 添加到搜索历史
        self.search_history.insert(0, search_query.clone());
        if self.search_history.len() > 50 {
            self.search_history.truncate(50);
        }
        
        // 生成搜索建议
        let suggestions = self.generate_suggestions(&query, &matches);
        
        Ok(SearchResult {
            query: search_query,
            results: matches,
            total_matches,
            search_time_ms: search_time,
            suggestions,
        })
    }
    
    /// 搜索文件名
    fn search_file_names(&self, query: &str, filters: &SearchFilters) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (_, file_meta) in &self.file_index.files {
            if !self.matches_filters(file_meta, filters) {
                continue;
            }
            
            let filename_lower = file_meta.name.to_lowercase();
            if filename_lower.contains(&query_lower) {
                let relevance = self.calculate_filename_relevance(&filename_lower, &query_lower);
                
                matches.push(SearchMatch {
                    file: file_meta.clone(),
                    relevance_score: relevance,
                    match_type: MatchType::FileName,
                    context_snippets: vec![],
                });
            }
        }
        
        Ok(matches)
    }
    
    /// 搜索文件内容
    fn search_file_content(&self, query: &str, filters: &SearchFilters) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (_, file_meta) in &self.file_index.files {
            if !self.matches_filters(file_meta, filters) {
                continue;
            }
            
            if let Ok(content) = std::fs::read_to_string(&file_meta.path) {
                let content_lower = content.to_lowercase();
                if content_lower.contains(&query_lower) {
                    let snippets = self.extract_context_snippets(&content, query);
                    let relevance = self.calculate_content_relevance(&content_lower, &query_lower, &snippets);
                    
                    matches.push(SearchMatch {
                        file: file_meta.clone(),
                        relevance_score: relevance,
                        match_type: MatchType::FileContent,
                        context_snippets: snippets,
                    });
                }
            }
        }
        
        Ok(matches)
    }
    
    /// 搜索元数据
    fn search_metadata(&self, query: &str, filters: &SearchFilters) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (_, file_meta) in &self.file_index.files {
            if !self.matches_filters(file_meta, filters) {
                continue;
            }
            
            let mut relevance = 0.0;
            let mut found = false;
            
            // 搜索标签
            for tag in &file_meta.tags {
                if tag.to_lowercase().contains(&query_lower) {
                    relevance += 0.8;
                    found = true;
                }
            }
            
            // 搜索描述
            if let Some(description) = &file_meta.description {
                if description.to_lowercase().contains(&query_lower) {
                    relevance += 0.6;
                    found = true;
                }
            }
            
            if found {
                matches.push(SearchMatch {
                    file: file_meta.clone(),
                    relevance_score: relevance,
                    match_type: MatchType::Metadata,
                    context_snippets: vec![],
                });
            }
        }
        
        Ok(matches)
    }
    
    /// 模糊搜索
    fn fuzzy_search(&self, query: &str, filters: &SearchFilters) -> Result<Vec<SearchMatch>> {
        let mut matches = Vec::new();
        
        for (_, file_meta) in &self.file_index.files {
            if !self.matches_filters(file_meta, filters) {
                continue;
            }
            
            let filename_score = self.calculate_fuzzy_score(&file_meta.name, query);
            
            if filename_score > 0.3 {
                matches.push(SearchMatch {
                    file: file_meta.clone(),
                    relevance_score: filename_score,
                    match_type: MatchType::FileName,
                    context_snippets: vec![],
                });
            }
        }
        
        Ok(matches)
    }
    
    /// 正则表达式搜索
    fn regex_search(&self, pattern: &str, filters: &SearchFilters) -> Result<Vec<SearchMatch>> {
        let regex = Regex::new(pattern)?;
        let mut matches = Vec::new();
        
        for (_, file_meta) in &self.file_index.files {
            if !self.matches_filters(file_meta, filters) {
                continue;
            }
            
            if let Ok(content) = std::fs::read_to_string(&file_meta.path) {
                if regex.is_match(&content) {
                    let snippets = self.extract_regex_snippets(&content, &regex);
                    
                    matches.push(SearchMatch {
                        file: file_meta.clone(),
                        relevance_score: 1.0,
                        match_type: MatchType::FileContent,
                        context_snippets: snippets,
                    });
                }
            }
        }
        
        Ok(matches)
    }
    
    /// 检查文件是否匹配过滤器
    fn matches_filters(&self, file_meta: &FileMetadata, filters: &SearchFilters) -> bool {
        // 文件类型过滤
        if !filters.file_types.is_empty() && !filters.file_types.contains(&FileType::All) {
            if !filters.file_types.contains(&file_meta.file_type) {
                return false;
            }
        }
        
        // 字数范围过滤
        if let Some(word_range) = &filters.word_count_range {
            if let Some(min) = word_range.min_words {
                if file_meta.word_count < min {
                    return false;
                }
            }
            if let Some(max) = word_range.max_words {
                if file_meta.word_count > max {
                    return false;
                }
            }
        }
        
        // 标签过滤
        if !filters.tags.is_empty() {
            let has_matching_tag = filters.tags.iter().any(|filter_tag| {
                file_meta.tags.iter().any(|file_tag| file_tag.contains(filter_tag))
            });
            if !has_matching_tag {
                return false;
            }
        }
        
        // 状态过滤
        if !filters.status.is_empty() {
            if !filters.status.contains(&file_meta.status) {
                return false;
            }
        }
        
        true
    }
    
    /// 计算文件名相关度
    fn calculate_filename_relevance(&self, filename: &str, query: &str) -> f32 {
        if filename == query {
            1.0
        } else if filename.starts_with(query) {
            0.9
        } else if filename.ends_with(query) {
            0.8
        } else {
            0.5
        }
    }
    
    /// 计算内容相关度
    fn calculate_content_relevance(&self, content: &str, query: &str, snippets: &[ContextSnippet]) -> f32 {
        let match_count = content.matches(query).count();
        let content_length = content.len();
        
        let frequency_score = (match_count as f32) / (content_length as f32 / 1000.0);
        let snippet_score = snippets.len() as f32 * 0.1;
        
        (frequency_score + snippet_score).min(1.0)
    }
    
    /// 计算模糊匹配分数
    fn calculate_fuzzy_score(&self, text: &str, query: &str) -> f32 {
        // 简单的模糊匹配算法
        let text_chars: Vec<char> = text.to_lowercase().chars().collect();
        let query_chars: Vec<char> = query.to_lowercase().chars().collect();
        
        let mut matches = 0;
        let mut query_index = 0;
        
        for &ch in &text_chars {
            if query_index < query_chars.len() && ch == query_chars[query_index] {
                matches += 1;
                query_index += 1;
            }
        }
        
        if query_index == query_chars.len() {
            matches as f32 / text_chars.len().max(query_chars.len()) as f32
        } else {
            0.0
        }
    }
    
    /// 提取上下文片段
    fn extract_context_snippets(&self, content: &str, query: &str) -> Vec<ContextSnippet> {
        let mut snippets = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (line_num, line) in content.lines().enumerate() {
            let line_lower = line.to_lowercase();
            if let Some(start) = line_lower.find(&query_lower) {
                let end = start + query.len();
                
                snippets.push(ContextSnippet {
                    line_number: (line_num + 1) as u32,
                    content: line.to_string(),
                    highlight_start: start,
                    highlight_end: end,
                });
            }
        }
        
        // 限制片段数量
        snippets.truncate(5);
        snippets
    }
    
    /// 提取正则表达式匹配片段
    fn extract_regex_snippets(&self, content: &str, regex: &Regex) -> Vec<ContextSnippet> {
        let mut snippets = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            if let Some(mat) = regex.find(line) {
                snippets.push(ContextSnippet {
                    line_number: (line_num + 1) as u32,
                    content: line.to_string(),
                    highlight_start: mat.start(),
                    highlight_end: mat.end(),
                });
            }
        }
        
        snippets.truncate(5);
        snippets
    }
    
    /// 生成搜索建议
    fn generate_suggestions(&self, query: &str, matches: &[SearchMatch]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // 基于搜索历史的建议
        for search in &self.search_history {
            if search.query != query && search.query.contains(query) {
                suggestions.push(search.query.clone());
            }
        }
        
        // 基于标签的建议
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        for search_match in matches {
            for tag in &search_match.file.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        let mut sorted_tags: Vec<_> = tag_counts.into_iter().collect();
        sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (tag, _) in sorted_tags.into_iter().take(3) {
            suggestions.push(format!("tag:{}", tag));
        }
        
        suggestions.truncate(5);
        suggestions
    }
    
    /// 获取搜索历史
    pub fn get_search_history(&self) -> &[SearchQuery] {
        &self.search_history
    }
    
    /// 清除搜索历史
    pub fn clear_search_history(&mut self) {
        self.search_history.clear();
    }
    
    /// 设置默认过滤器
    pub fn set_default_filters(&mut self, filters: SearchFilters) {
        self.search_filters = filters;
    }
    
    /// 获取文件统计信息
    pub fn get_file_statistics(&self) -> FileStatistics {
        let mut stats = FileStatistics::default();
        
        for (_, file_meta) in &self.file_index.files {
            stats.total_files += 1;
            stats.total_size += file_meta.size;
            stats.total_words += file_meta.word_count;
            
            match file_meta.file_type {
                FileType::Chapter => stats.chapters += 1,
                FileType::Character => stats.characters += 1,
                FileType::Scene => stats.scenes += 1,
                FileType::Research => stats.research_files += 1,
                FileType::Note => stats.notes += 1,
                _ => {}
            }
        }
        
        stats
    }
}

/// 文件统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileStatistics {
    pub total_files: u32,
    pub total_size: u64,
    pub total_words: u32,
    pub chapters: u32,
    pub characters: u32,
    pub scenes: u32,
    pub research_files: u32,
    pub notes: u32,
}

impl FileIndex {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            content_index: HashMap::new(),
            tag_index: HashMap::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.files.clear();
        self.content_index.clear();
        self.tag_index.clear();
    }
    
    pub fn add_file(&mut self, file_meta: FileMetadata) -> Result<()> {
        let path = file_meta.path.clone();
        
        // 添加到标签索引
        for tag in &file_meta.tags {
            self.tag_index.entry(tag.clone()).or_insert_with(Vec::new).push(path.clone());
        }
        
        // 添加文件元数据
        self.files.insert(path, file_meta);
        
        Ok(())
    }
}

impl Default for AdvancedFileSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            file_types: vec![FileType::All],
            date_range: None,
            size_range: None,
            word_count_range: None,
            tags: Vec::new(),
            status: Vec::new(),
            folders: Vec::new(),
            exclude_patterns: Vec::new(),
        }
    }
}
