use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration, NaiveDate};

/// 写作分析系统
#[derive(Debug, Clone)]
pub struct WritingAnalytics {
    writing_sessions: Vec<WritingSession>,
    daily_stats: HashMap<NaiveDate, DailyWritingStats>,
    text_analyzer: TextAnalyzer,
    goal_tracker: GoalTracker,
}

/// 写作会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingSession {
    pub id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: u32,
    pub words_written: u32,
    pub words_deleted: u32,
    pub net_words: i32,
    pub files_modified: Vec<String>,
    pub session_type: SessionType,
    pub notes: Option<String>,
}

/// 会话类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Writing,        // 纯写作
    Editing,        // 编辑修改
    Research,       // 研究资料
    Planning,       // 规划大纲
    Review,         // 审阅校对
    Mixed,          // 混合类型
}

/// 每日写作统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyWritingStats {
    pub date: NaiveDate,
    pub total_words_written: u32,
    pub total_time_minutes: u32,
    pub session_count: u32,
    pub average_wpm: f32,
    pub goal_progress: f32,
    pub peak_hour: Option<u8>,
    pub productivity_score: f32,
    pub files_touched: Vec<String>,
}

/// 文本分析器
#[derive(Debug, Clone)]
pub struct TextAnalyzer {
    readability_cache: HashMap<String, ReadabilityScore>,
}

/// 可读性评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityScore {
    pub flesch_reading_ease: f32,
    pub flesch_kincaid_grade: f32,
    pub average_sentence_length: f32,
    pub average_syllables_per_word: f32,
    pub difficulty_level: DifficultyLevel,
    pub suggestions: Vec<String>,
}

/// 难度等级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    VeryEasy,       // 90-100
    Easy,           // 80-90
    FairlyEasy,     // 70-80
    Standard,       // 60-70
    FairlyDifficult, // 50-60
    Difficult,      // 30-50
    VeryDifficult,  // 0-30
}

/// 目标跟踪器
#[derive(Debug, Clone)]
pub struct GoalTracker {
    goals: Vec<WritingGoal>,
    milestones: Vec<Milestone>,
}

/// 写作目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingGoal {
    pub id: String,
    pub title: String,
    pub goal_type: GoalType,
    pub target_value: u32,
    pub current_value: u32,
    pub deadline: Option<String>,
    pub created_at: String,
    pub status: GoalStatus,
    pub priority: Priority,
}

/// 目标类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    DailyWords(u32),        // 每日字数
    WeeklyWords(u32),       // 每周字数
    MonthlyWords(u32),      // 每月字数
    ProjectWords(u32),      // 项目总字数
    DailyTime(u32),         // 每日写作时间(分钟)
    ConsecutiveDays(u32),   // 连续写作天数
    ChapterCount(u32),      // 章节数量
    CustomMetric(String, u32), // 自定义指标
}

/// 目标状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    Active,
    Completed,
    Paused,
    Failed,
    Overdue,
}

/// 优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// 里程碑
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_date: String,
    pub achieved_date: Option<String>,
    pub value: u32,
    pub milestone_type: MilestoneType,
    pub celebration_message: Option<String>,
}

/// 里程碑类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneType {
    WordCount,
    ChapterCount,
    TimeSpent,
    ConsecutiveDays,
    ProjectCompletion,
}

/// 写作分析报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingAnalysisReport {
    pub period_start: String,
    pub period_end: String,
    pub total_words: u32,
    pub total_time_minutes: u32,
    pub average_daily_words: f32,
    pub average_wpm: f32,
    pub most_productive_day: Option<NaiveDate>,
    pub most_productive_hour: Option<u8>,
    pub streak_days: u32,
    pub goal_completion_rate: f32,
    pub productivity_trend: ProductivityTrend,
    pub text_quality_score: f32,
    pub recommendations: Vec<String>,
}

/// 生产力趋势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductivityTrend {
    Increasing,
    Stable,
    Decreasing,
    Fluctuating,
}

impl WritingAnalytics {
    /// 创建新的写作分析系统
    pub fn new() -> Self {
        Self {
            writing_sessions: Vec::new(),
            daily_stats: HashMap::new(),
            text_analyzer: TextAnalyzer::new(),
            goal_tracker: GoalTracker::new(),
        }
    }
    
    /// 开始新的写作会话
    pub fn start_session(&mut self, session_type: SessionType) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = WritingSession {
            id: session_id.clone(),
            start_time: Utc::now().to_rfc3339(),
            end_time: None,
            duration_minutes: 0,
            words_written: 0,
            words_deleted: 0,
            net_words: 0,
            files_modified: Vec::new(),
            session_type,
            notes: None,
        };
        
        self.writing_sessions.push(session);
        session_id
    }
    
    /// 结束写作会话
    pub fn end_session(&mut self, session_id: &str, final_word_count: u32, files_modified: Vec<String>) -> Result<()> {
        if let Some(session) = self.writing_sessions.iter_mut().find(|s| s.id == session_id) {
            session.end_time = Some(Utc::now().to_rfc3339());
            session.files_modified = files_modified;
            
            // 计算持续时间
            if let Ok(start) = DateTime::parse_from_rfc3339(&session.start_time) {
                let duration = Utc::now().signed_duration_since(start);
                session.duration_minutes = duration.num_minutes().max(0) as u32;
            }
            
            // 更新每日统计
            self.update_daily_stats(session);
            
            // 更新目标进度
            self.goal_tracker.update_progress(session);
        }
        
        Ok(())
    }
    
    /// 更新会话进度
    pub fn update_session(&mut self, session_id: &str, words_written: u32, words_deleted: u32) {
        if let Some(session) = self.writing_sessions.iter_mut().find(|s| s.id == session_id) {
            session.words_written = words_written;
            session.words_deleted = words_deleted;
            session.net_words = words_written as i32 - words_deleted as i32;
        }
    }
    
    /// 更新每日统计
    fn update_daily_stats(&mut self, session: &WritingSession) {
        if let Ok(start_time) = DateTime::parse_from_rfc3339(&session.start_time) {
            let date = start_time.date_naive();
            
            let daily_stats = self.daily_stats.entry(date).or_insert_with(|| DailyWritingStats {
                date,
                total_words_written: 0,
                total_time_minutes: 0,
                session_count: 0,
                average_wpm: 0.0,
                goal_progress: 0.0,
                peak_hour: None,
                productivity_score: 0.0,
                files_touched: Vec::new(),
            });
            
            daily_stats.total_words_written += session.words_written;
            daily_stats.total_time_minutes += session.duration_minutes;
            daily_stats.session_count += 1;
            
            // 计算平均WPM
            if daily_stats.total_time_minutes > 0 {
                daily_stats.average_wpm = daily_stats.total_words_written as f32 / (daily_stats.total_time_minutes as f32 / 60.0);
            }
            
            // 合并文件列表
            for file in &session.files_modified {
                if !daily_stats.files_touched.contains(file) {
                    daily_stats.files_touched.push(file.clone());
                }
            }
            
            // 计算生产力评分
            daily_stats.productivity_score = self.calculate_productivity_score(daily_stats);
        }
    }
    
    /// 计算生产力评分
    fn calculate_productivity_score(&self, stats: &DailyWritingStats) -> f32 {
        let word_score = (stats.total_words_written as f32 / 1000.0).min(1.0) * 40.0;
        let time_score = (stats.total_time_minutes as f32 / 120.0).min(1.0) * 30.0;
        let consistency_score = if stats.session_count >= 2 { 20.0 } else { stats.session_count as f32 * 10.0 };
        let efficiency_score = if stats.average_wpm > 0.0 { (stats.average_wpm / 50.0).min(1.0) * 10.0 } else { 0.0 };
        
        word_score + time_score + consistency_score + efficiency_score
    }
    
    /// 分析文本质量
    pub fn analyze_text(&mut self, text: &str) -> ReadabilityScore {
        self.text_analyzer.analyze(text)
    }
    
    /// 生成写作分析报告
    pub fn generate_report(&self, days: u32) -> WritingAnalysisReport {
        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(days as i64);
        
        let relevant_stats: Vec<&DailyWritingStats> = self.daily_stats
            .values()
            .filter(|stats| stats.date >= start_date && stats.date <= end_date)
            .collect();
        
        let total_words: u32 = relevant_stats.iter().map(|s| s.total_words_written).sum();
        let total_time: u32 = relevant_stats.iter().map(|s| s.total_time_minutes).sum();
        let average_daily_words = if days > 0 { total_words as f32 / days as f32 } else { 0.0 };
        let average_wpm = if total_time > 0 { total_words as f32 / (total_time as f32 / 60.0) } else { 0.0 };
        
        let most_productive_day = relevant_stats
            .iter()
            .max_by_key(|s| s.total_words_written)
            .map(|s| s.date);
        
        let streak_days = self.calculate_writing_streak();
        let goal_completion_rate = self.goal_tracker.calculate_completion_rate();
        let productivity_trend = self.analyze_productivity_trend(&relevant_stats);
        
        WritingAnalysisReport {
            period_start: start_date.to_string(),
            period_end: end_date.to_string(),
            total_words,
            total_time_minutes: total_time,
            average_daily_words,
            average_wpm,
            most_productive_day,
            most_productive_hour: self.find_most_productive_hour(),
            streak_days,
            goal_completion_rate,
            productivity_trend,
            text_quality_score: 75.0, // 占位符
            recommendations: self.generate_recommendations(),
        }
    }
    
    /// 计算写作连续天数
    fn calculate_writing_streak(&self) -> u32 {
        let mut streak = 0;
        let mut current_date = Utc::now().date_naive();
        
        loop {
            if let Some(stats) = self.daily_stats.get(&current_date) {
                if stats.total_words_written > 0 {
                    streak += 1;
                    current_date = current_date - Duration::days(1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        streak
    }
    
    /// 分析生产力趋势
    fn analyze_productivity_trend(&self, stats: &[&DailyWritingStats]) -> ProductivityTrend {
        if stats.len() < 3 {
            return ProductivityTrend::Stable;
        }
        
        let scores: Vec<f32> = stats.iter().map(|s| s.productivity_score).collect();
        let first_half = &scores[0..scores.len()/2];
        let second_half = &scores[scores.len()/2..];
        
        let first_avg: f32 = first_half.iter().sum::<f32>() / first_half.len() as f32;
        let second_avg: f32 = second_half.iter().sum::<f32>() / second_half.len() as f32;
        
        let change_rate = (second_avg - first_avg) / first_avg;
        
        if change_rate > 0.1 {
            ProductivityTrend::Increasing
        } else if change_rate < -0.1 {
            ProductivityTrend::Decreasing
        } else {
            ProductivityTrend::Stable
        }
    }
    
    /// 找到最有生产力的小时
    fn find_most_productive_hour(&self) -> Option<u8> {
        let mut hour_stats: HashMap<u8, u32> = HashMap::new();
        
        for session in &self.writing_sessions {
            if let Ok(start_time) = DateTime::parse_from_rfc3339(&session.start_time) {
                let hour = start_time.hour() as u8;
                *hour_stats.entry(hour).or_insert(0) += session.words_written;
            }
        }
        
        hour_stats
            .into_iter()
            .max_by_key(|(_, words)| *words)
            .map(|(hour, _)| hour)
    }
    
    /// 生成写作建议
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        let recent_days = 7;
        let recent_stats: Vec<&DailyWritingStats> = self.daily_stats
            .values()
            .filter(|stats| {
                let days_ago = Utc::now().date_naive() - stats.date;
                days_ago.num_days() <= recent_days
            })
            .collect();
        
        if recent_stats.is_empty() {
            recommendations.push("开始记录您的写作活动以获得个性化建议。".to_string());
            return recommendations;
        }
        
        let avg_daily_words: f32 = recent_stats.iter().map(|s| s.total_words_written).sum::<u32>() as f32 / recent_stats.len() as f32;
        let avg_wpm: f32 = recent_stats.iter().map(|s| s.average_wpm).sum::<f32>() / recent_stats.len() as f32;
        
        // 字数建议
        if avg_daily_words < 500.0 {
            recommendations.push("考虑设定每日500字的小目标来建立写作习惯。".to_string());
        } else if avg_daily_words > 2000.0 {
            recommendations.push("您的写作产量很高！注意保持质量与数量的平衡。".to_string());
        }
        
        // 写作速度建议
        if avg_wpm < 20.0 {
            recommendations.push("尝试定时写作练习来提高写作速度。".to_string());
        } else if avg_wpm > 60.0 {
            recommendations.push("您的写作速度很快，可以多花时间在编辑和校对上。".to_string());
        }
        
        // 一致性建议
        let writing_days = recent_stats.len();
        if writing_days < recent_days as usize / 2 {
            recommendations.push("尝试更频繁地写作，即使每次只写几分钟。".to_string());
        }
        
        // 目标完成建议
        let goal_rate = self.goal_tracker.calculate_completion_rate();
        if goal_rate < 0.5 {
            recommendations.push("考虑调整写作目标，使其更现实可达。".to_string());
        } else if goal_rate > 0.9 {
            recommendations.push("您的目标完成率很高！可以设定更有挑战性的目标。".to_string());
        }
        
        recommendations
    }
    
    /// 获取活跃会话
    pub fn get_active_sessions(&self) -> Vec<&WritingSession> {
        self.writing_sessions
            .iter()
            .filter(|session| session.end_time.is_none())
            .collect()
    }
    
    /// 获取最近的统计数据
    pub fn get_recent_stats(&self, days: u32) -> Vec<&DailyWritingStats> {
        let cutoff_date = Utc::now().date_naive() - Duration::days(days as i64);
        self.daily_stats
            .values()
            .filter(|stats| stats.date >= cutoff_date)
            .collect()
    }
    
    /// 获取目标跟踪器
    pub fn get_goal_tracker(&self) -> &GoalTracker {
        &self.goal_tracker
    }
    
    /// 获取可变目标跟踪器
    pub fn get_goal_tracker_mut(&mut self) -> &mut GoalTracker {
        &mut self.goal_tracker
    }
    
    /// 计算写作连续天数（公开方法）
    pub fn calculate_writing_streak(&self) -> u32 {
        let mut streak = 0;
        let mut current_date = Utc::now().date_naive();
        
        loop {
            if let Some(stats) = self.daily_stats.get(&current_date) {
                if stats.total_words_written > 0 {
                    streak += 1;
                    current_date = current_date - Duration::days(1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        streak
    }
    
    /// 获取特定日期的统计数据
    pub fn get_daily_stats(&self, date: &chrono::NaiveDate) -> Option<&DailyWritingStats> {
        self.daily_stats.get(date)
    }
    
    /// 获取所有写作会话
    pub fn get_writing_sessions(&self) -> &[WritingSession] {
        &self.writing_sessions
    }
}

impl TextAnalyzer {
    pub fn new() -> Self {
        Self {
            readability_cache: HashMap::new(),
        }
    }
    
    /// 分析文本可读性
    pub fn analyze(&mut self, text: &str) -> ReadabilityScore {
        // 简化的可读性分析
        let words = self.count_words(text);
        let sentences = self.count_sentences(text);
        let syllables = self.count_syllables(text);
        
        let avg_sentence_length = if sentences > 0 { words as f32 / sentences as f32 } else { 0.0 };
        let avg_syllables_per_word = if words > 0 { syllables as f32 / words as f32 } else { 0.0 };
        
        // Flesch Reading Ease
        let flesch_reading_ease = 206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word);
        
        // Flesch-Kincaid Grade Level
        let flesch_kincaid_grade = (0.39 * avg_sentence_length) + (11.8 * avg_syllables_per_word) - 15.59;
        
        let difficulty_level = match flesch_reading_ease {
            x if x >= 90.0 => DifficultyLevel::VeryEasy,
            x if x >= 80.0 => DifficultyLevel::Easy,
            x if x >= 70.0 => DifficultyLevel::FairlyEasy,
            x if x >= 60.0 => DifficultyLevel::Standard,
            x if x >= 50.0 => DifficultyLevel::FairlyDifficult,
            x if x >= 30.0 => DifficultyLevel::Difficult,
            _ => DifficultyLevel::VeryDifficult,
        };
        
        let suggestions = self.generate_readability_suggestions(avg_sentence_length, avg_syllables_per_word, &difficulty_level);
        
        ReadabilityScore {
            flesch_reading_ease,
            flesch_kincaid_grade,
            average_sentence_length: avg_sentence_length,
            average_syllables_per_word: avg_syllables_per_word,
            difficulty_level,
            suggestions,
        }
    }
    
    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    fn count_sentences(&self, text: &str) -> usize {
        text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count().max(1)
    }
    
    fn count_syllables(&self, text: &str) -> usize {
        // 简化的音节计数
        text.split_whitespace()
            .map(|word| self.count_word_syllables(word))
            .sum()
    }
    
    fn count_word_syllables(&self, word: &str) -> usize {
        let word = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();
        if word.is_empty() { return 0; }
        
        let vowels = "aeiou";
        let mut syllable_count = 0;
        let mut previous_was_vowel = false;
        
        for c in word.chars() {
            let is_vowel = vowels.contains(c);
            if is_vowel && !previous_was_vowel {
                syllable_count += 1;
            }
            previous_was_vowel = is_vowel;
        }
        
        // 如果单词以无声e结尾，减去一个音节
        if word.ends_with('e') && syllable_count > 1 {
            syllable_count -= 1;
        }
        
        syllable_count.max(1)
    }
    
    fn generate_readability_suggestions(&self, avg_sentence_length: f32, avg_syllables_per_word: f32, difficulty: &DifficultyLevel) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if avg_sentence_length > 20.0 {
            suggestions.push("考虑将长句子拆分为更短的句子以提高可读性。".to_string());
        }
        
        if avg_syllables_per_word > 1.7 {
            suggestions.push("尝试使用更简单的单词来降低阅读难度。".to_string());
        }
        
        match difficulty {
            DifficultyLevel::VeryDifficult | DifficultyLevel::Difficult => {
                suggestions.push("文本较难理解，考虑简化语言和句式结构。".to_string());
            }
            DifficultyLevel::VeryEasy => {
                suggestions.push("文本很容易理解，如果目标读者是成年人，可以适当增加复杂度。".to_string());
            }
            _ => {}
        }
        
        suggestions
    }
}

impl GoalTracker {
    pub fn new() -> Self {
        Self {
            goals: Vec::new(),
            milestones: Vec::new(),
        }
    }
    
    /// 添加新目标
    pub fn add_goal(&mut self, title: String, goal_type: GoalType, deadline: Option<String>, priority: Priority) -> String {
        let goal_id = uuid::Uuid::new_v4().to_string();
        let target_value = match &goal_type {
            GoalType::DailyWords(target) => *target,
            GoalType::WeeklyWords(target) => *target,
            GoalType::MonthlyWords(target) => *target,
            GoalType::ProjectWords(target) => *target,
            GoalType::DailyTime(target) => *target,
            GoalType::ConsecutiveDays(target) => *target,
            GoalType::ChapterCount(target) => *target,
            GoalType::CustomMetric(_, target) => *target,
        };
        
        let goal = WritingGoal {
            id: goal_id.clone(),
            title,
            goal_type,
            target_value,
            current_value: 0,
            deadline,
            created_at: Utc::now().to_rfc3339(),
            status: GoalStatus::Active,
            priority,
        };
        
        self.goals.push(goal);
        goal_id
    }
    
    /// 更新目标进度
    pub fn update_progress(&mut self, session: &WritingSession) {
        for goal in &mut self.goals {
            if goal.status != GoalStatus::Active {
                continue;
            }
            
            match &goal.goal_type {
                GoalType::DailyWords(_) => {
                    // 每日目标在新的一天重置
                    goal.current_value += session.words_written;
                }
                GoalType::WeeklyWords(_) | GoalType::MonthlyWords(_) | GoalType::ProjectWords(_) => {
                    goal.current_value += session.words_written;
                }
                GoalType::DailyTime(_) => {
                    goal.current_value += session.duration_minutes;
                }
                _ => {}
            }
            
            // 检查目标是否完成
            if goal.current_value >= goal.target_value {
                goal.status = GoalStatus::Completed;
            }
        }
    }
    
    /// 计算目标完成率
    pub fn calculate_completion_rate(&self) -> f32 {
        let active_goals: Vec<&WritingGoal> = self.goals.iter().filter(|g| g.status == GoalStatus::Active).collect();
        
        if active_goals.is_empty() {
            return 1.0;
        }
        
        let total_progress: f32 = active_goals
            .iter()
            .map(|goal| (goal.current_value as f32 / goal.target_value as f32).min(1.0))
            .sum();
        
        total_progress / active_goals.len() as f32
    }
    
    /// 获取活跃目标
    pub fn get_active_goals(&self) -> Vec<&WritingGoal> {
        self.goals.iter().filter(|g| g.status == GoalStatus::Active).collect()
    }
    
    /// 获取完成的目标
    pub fn get_completed_goals(&self) -> Vec<&WritingGoal> {
        self.goals.iter().filter(|g| g.status == GoalStatus::Completed).collect()
    }
}

impl Default for WritingAnalytics {
    fn default() -> Self {
        Self::new()
    }
}
