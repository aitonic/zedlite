use std::time::Duration;
use gpui::{App, Context, IntoElement, Render, Window, px};
use ui::{
    Button, ButtonSize, ButtonStyle, h_flex, v_flex, Label, LabelSize, 
    prelude::*, IconName, TextInput, Toggle
};
use crate::writing_analytics::{
    WritingAnalytics, SessionType, WritingSession, DailyWritingStats, 
    WritingGoal, GoalType, GoalStatus, Priority, WritingAnalysisReport, ProductivityTrend
};

/// 写作助手视图
pub struct WritingAssistantView {
    analytics: WritingAnalytics,
    
    // 界面状态
    current_tab: WritingTab,
    active_session_id: Option<String>,
    
    // 目标创建
    new_goal_title: String,
    new_goal_type: GoalType,
    new_goal_target: String,
    new_goal_priority: Priority,
    
    // 设置
    show_advanced_stats: bool,
    auto_track_sessions: bool,
    daily_word_goal: u32,
    report_period_days: u32,
    
    // 当前报告
    current_report: Option<WritingAnalysisReport>,
}

/// 写作助手标签页
#[derive(Debug, Clone, PartialEq)]
pub enum WritingTab {
    Dashboard,      // 仪表板
    Sessions,       // 会话管理
    Goals,          // 目标跟踪
    Analytics,      // 深度分析
    TextAnalysis,   // 文本分析
    Settings,       // 设置
}

impl WritingAssistantView {
    pub fn new() -> Self {
        Self {
            analytics: WritingAnalytics::new(),
            current_tab: WritingTab::Dashboard,
            active_session_id: None,
            new_goal_title: String::new(),
            new_goal_type: GoalType::DailyWords(1000),
            new_goal_target: "1000".to_string(),
            new_goal_priority: Priority::Medium,
            show_advanced_stats: false,
            auto_track_sessions: true,
            daily_word_goal: 1000,
            report_period_days: 7,
            current_report: None,
        }
    }
    
    pub fn get_analytics(&self) -> &WritingAnalytics {
        &self.analytics
    }
    
    pub fn get_analytics_mut(&mut self) -> &mut WritingAnalytics {
        &mut self.analytics
    }
}

impl Render for WritingAssistantView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .padding(px(16.))
            .child(self.render_header(cx))
            .child(self.render_tab_selector(cx))
            .child(self.render_tab_content(cx))
    }
}

impl WritingAssistantView {
    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .justify_between()
            .items_center()
            .child(
                Label::new("Writing Assistant")
                    .size(LabelSize::Large)
            )
            .child(self.render_session_control(cx))
    }
    
    fn render_session_control(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(_session_id) = &self.active_session_id {
            h_flex()
                .gap_2()
                .child(
                    Label::new("● Writing Session Active")
                        .size(LabelSize::Small)
                )
                .child(
                    Button::new("end_session", "End Session")
                        .style(ButtonStyle::Subtle)
                        .size(ButtonSize::Small)
                        .on_click(cx.listener(|this, _, _window, _cx| {
                            if let Some(session_id) = &this.active_session_id {
                                let _ = this.analytics.end_session(session_id, 0, vec![]);
                                this.active_session_id = None;
                            }
                        }))
                )
        } else {
            h_flex()
                .gap_2()
                .child(
                    Button::new("start_writing", "Start Writing")
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Small)
                        .on_click(cx.listener(|this, _, _window, _cx| {
                            let session_id = this.analytics.start_session(SessionType::Writing);
                            this.active_session_id = Some(session_id);
                        }))
                )
                .child(
                    Button::new("start_editing", "Start Editing")
                        .style(ButtonStyle::Subtle)
                        .size(ButtonSize::Small)
                        .on_click(cx.listener(|this, _, _window, _cx| {
                            let session_id = this.analytics.start_session(SessionType::Editing);
                            this.active_session_id = Some(session_id);
                        }))
                )
        }
    }
    
    fn render_tab_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(self.tab_button("Dashboard", WritingTab::Dashboard, cx))
            .child(self.tab_button("Sessions", WritingTab::Sessions, cx))
            .child(self.tab_button("Goals", WritingTab::Goals, cx))
            .child(self.tab_button("Analytics", WritingTab::Analytics, cx))
            .child(self.tab_button("Text Analysis", WritingTab::TextAnalysis, cx))
            .child(self.tab_button("Settings", WritingTab::Settings, cx))
    }
    
    fn tab_button(&self, label: &str, tab: WritingTab, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.current_tab == tab;
        let tab_for_click = tab.clone();
        
        Button::new(format!("writing_tab_{:?}", tab), label)
            .style(if is_active { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.current_tab = tab_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_tab_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.current_tab {
            WritingTab::Dashboard => self.render_dashboard_tab(cx),
            WritingTab::Sessions => self.render_sessions_tab(cx),
            WritingTab::Goals => self.render_goals_tab(cx),
            WritingTab::Analytics => self.render_analytics_tab(cx),
            WritingTab::TextAnalysis => self.render_text_analysis_tab(cx),
            WritingTab::Settings => self.render_settings_tab(cx),
        }
    }
    
    fn render_dashboard_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(self.render_quick_stats())
            .child(self.render_today_progress())
            .child(self.render_active_goals())
            .child(self.render_recent_recommendations())
    }
    
    fn render_quick_stats(&self) -> impl IntoElement {
        let recent_stats = self.analytics.get_recent_stats(7);
        let total_words: u32 = recent_stats.iter().map(|s| s.total_words_written).sum();
        let total_time: u32 = recent_stats.iter().map(|s| s.total_time_minutes).sum();
        let avg_wpm = if total_time > 0 { total_words as f32 / (total_time as f32 / 60.0) } else { 0.0 };
        let streak = self.analytics.calculate_writing_streak();
        
        v_flex()
            .gap_2()
            .child(
                Label::new("7-Day Summary")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_6()
                    .child(self.stat_card("Words Written", &total_words.to_string()))
                    .child(self.stat_card("Time Spent", &format!("{}h {}m", total_time / 60, total_time % 60)))
                    .child(self.stat_card("Avg Speed", &format!("{:.1} WPM", avg_wpm)))
                    .child(self.stat_card("Streak", &format!("{} days", streak)))
            )
    }
    
    fn stat_card(&self, label: &str, value: &str) -> impl IntoElement {
        v_flex()
            .gap_1()
            .padding(px(12.))
            .border_1()
            .border_color(ui::colors::border())
            .rounded(px(4.))
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(value)
                    .size(LabelSize::Default)
            )
    }
    
    fn render_today_progress(&self) -> impl IntoElement {
        let today = chrono::Utc::now().date_naive();
        let today_stats = self.analytics.get_daily_stats(&today);
        
        let words_today = today_stats.map(|s| s.total_words_written).unwrap_or(0);
        let time_today = today_stats.map(|s| s.total_time_minutes).unwrap_or(0);
        let progress = words_today as f32 / self.daily_word_goal as f32;
        
        v_flex()
            .gap_2()
            .child(
                Label::new("Today's Progress")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new(&format!("{} / {} words", words_today, self.daily_word_goal))
                    )
                    .child(
                        Label::new(&format!("{:.1}%", progress * 100.0))
                            .size(LabelSize::Small)
                    )
            )
            .child(
                // 简化的进度条
                h_flex()
                    .w_full()
                    .h(px(8.))
                    .bg(ui::colors::surface())
                    .rounded(px(4.))
                    .child(
                        h_flex()
                            .w(px((progress * 300.0).min(300.0)))
                            .h_full()
                            .bg(ui::colors::accent())
                            .rounded(px(4.))
                    )
            )
            .child(
                Label::new(&format!("Time: {}h {}m", time_today / 60, time_today % 60))
                    .size(LabelSize::Small)
            )
    }
    
    fn render_active_goals(&self) -> impl IntoElement {
        let active_goals = self.analytics.get_goal_tracker().get_active_goals();
        
        v_flex()
            .gap_2()
            .child(
                Label::new("Active Goals")
                    .size(LabelSize::Default)
            )
            .children(
                active_goals.iter().take(3).map(|goal| {
                    let progress = goal.current_value as f32 / goal.target_value as f32;
                    
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
                                .child(Label::new(&goal.title))
                                .child(
                                    Label::new(&format!("{} / {} ({:.1}%)", 
                                        goal.current_value, goal.target_value, progress * 100.0))
                                        .size(LabelSize::Small)
                                )
                        )
                        .child(
                            Label::new(&format!("{:?}", goal.priority))
                                .size(LabelSize::Small)
                        )
                })
            )
            .when(active_goals.is_empty(), |this| {
                this.child(
                    Label::new("No active goals. Create some goals to track your progress!")
                        .size(LabelSize::Small)
                )
            })
    }
    
    fn render_recent_recommendations(&self) -> impl IntoElement {
        let report = self.analytics.generate_report(7);
        
        v_flex()
            .gap_2()
            .child(
                Label::new("Writing Recommendations")
                    .size(LabelSize::Default)
            )
            .children(
                report.recommendations.iter().take(3).map(|recommendation| {
                    Label::new(&format!("• {}", recommendation))
                        .size(LabelSize::Small)
                })
            )
    }
    
    fn render_sessions_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let recent_sessions = self.analytics.get_writing_sessions()
            .iter()
            .rev()
            .take(10)
            .collect::<Vec<_>>();
        
        v_flex()
            .gap_3()
            .child(
                Label::new("Writing Sessions")
                    .size(LabelSize::Default)
            )
            .child(self.render_session_controls(cx))
            .children(
                recent_sessions.iter().map(|session| {
                    self.render_session_item(session)
                })
            )
    }
    
    fn render_session_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                Button::new("start_writing_session", "Start Writing")
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        let session_id = this.analytics.start_session(SessionType::Writing);
                        this.active_session_id = Some(session_id);
                    }))
            )
            .child(
                Button::new("start_editing_session", "Start Editing")
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        let session_id = this.analytics.start_session(SessionType::Editing);
                        this.active_session_id = Some(session_id);
                    }))
            )
            .child(
                Button::new("start_research_session", "Start Research")
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        let session_id = this.analytics.start_session(SessionType::Research);
                        this.active_session_id = Some(session_id);
                    }))
            )
    }
    
    fn render_session_item(&self, session: &WritingSession) -> impl IntoElement {
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
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Label::new(&format!("{:?}", session.session_type))
                                    .size(LabelSize::Default)
                            )
                            .when(session.end_time.is_none(), |this| {
                                this.child(
                                    Label::new("● Active")
                                        .size(LabelSize::Small)
                                )
                            })
                    )
                    .child(
                        Label::new(&format!("Started: {}", 
                            chrono::DateTime::parse_from_rfc3339(&session.start_time)
                                .map(|dt| dt.format("%H:%M").to_string())
                                .unwrap_or_else(|_| "Unknown".to_string())))
                            .size(LabelSize::Small)
                    )
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(&format!("{} words", session.words_written))
                            .size(LabelSize::Small)
                    )
                    .child(
                        Label::new(&format!("{}m", session.duration_minutes))
                            .size(LabelSize::Small)
                    )
            )
    }
    
    fn render_goals_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(self.render_goal_creation_form(cx))
            .child(self.render_goals_list())
    }
    
    fn render_goal_creation_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Create New Goal")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        TextInput::new("goal_title")
                            .placeholder("Goal title")
                            .on_input(cx.listener(|this, input, _window, cx| {
                                this.new_goal_title = input;
                                cx.notify();
                            }))
                    )
                    .child(
                        TextInput::new("goal_target")
                            .placeholder("Target value")
                            .on_input(cx.listener(|this, input, _window, cx| {
                                this.new_goal_target = input;
                                cx.notify();
                            }))
                    )
            )
            .child(self.render_goal_type_selector(cx))
            .child(self.render_priority_selector(cx))
            .child(
                Button::new("create_goal", "Create Goal")
                    .style(ButtonStyle::Filled)
                    .disabled(self.new_goal_title.is_empty() || self.new_goal_target.is_empty())
                    .on_click(cx.listener(|this, _, _window, _cx| {
                        if let Ok(target) = this.new_goal_target.parse::<u32>() {
                            let goal_type = match this.new_goal_type {
                                GoalType::DailyWords(_) => GoalType::DailyWords(target),
                                GoalType::WeeklyWords(_) => GoalType::WeeklyWords(target),
                                GoalType::MonthlyWords(_) => GoalType::MonthlyWords(target),
                                GoalType::ProjectWords(_) => GoalType::ProjectWords(target),
                                GoalType::DailyTime(_) => GoalType::DailyTime(target),
                                GoalType::ConsecutiveDays(_) => GoalType::ConsecutiveDays(target),
                                GoalType::ChapterCount(_) => GoalType::ChapterCount(target),
                                GoalType::CustomMetric(ref name, _) => GoalType::CustomMetric(name.clone(), target),
                            };
                            
                            this.analytics.get_goal_tracker_mut().add_goal(
                                this.new_goal_title.clone(),
                                goal_type,
                                None,
                                this.new_goal_priority.clone(),
                            );
                            
                            // Clear form
                            this.new_goal_title.clear();
                            this.new_goal_target.clear();
                        }
                    }))
            )
    }
    
    fn render_goal_type_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Label::new("Type:"))
            .child(self.goal_type_button("Daily Words", GoalType::DailyWords(1000), cx))
            .child(self.goal_type_button("Weekly Words", GoalType::WeeklyWords(7000), cx))
            .child(self.goal_type_button("Monthly Words", GoalType::MonthlyWords(30000), cx))
            .child(self.goal_type_button("Daily Time", GoalType::DailyTime(120), cx))
    }
    
    fn goal_type_button(&self, label: &str, goal_type: GoalType, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = std::mem::discriminant(&self.new_goal_type) == std::mem::discriminant(&goal_type);
        let goal_type_for_click = goal_type.clone();
        
        Button::new(format!("goal_type_{:?}", goal_type), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.new_goal_type = goal_type_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_priority_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(Label::new("Priority:"))
            .child(self.priority_button("Low", Priority::Low, cx))
            .child(self.priority_button("Medium", Priority::Medium, cx))
            .child(self.priority_button("High", Priority::High, cx))
            .child(self.priority_button("Critical", Priority::Critical, cx))
    }
    
    fn priority_button(&self, label: &str, priority: Priority, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = std::mem::discriminant(&self.new_goal_priority) == std::mem::discriminant(&priority);
        let priority_for_click = priority.clone();
        
        Button::new(format!("priority_{:?}", priority), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.new_goal_priority = priority_for_click.clone();
                cx.notify();
            }))
    }
    
    fn render_goals_list(&self) -> impl IntoElement {
        let active_goals = self.analytics.get_goal_tracker().get_active_goals();
        let completed_goals = self.analytics.get_goal_tracker().get_completed_goals();
        
        v_flex()
            .gap_3()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Label::new("Active Goals")
                            .size(LabelSize::Default)
                    )
                    .children(
                        active_goals.iter().map(|goal| self.render_goal_item(goal))
                    )
                    .when(active_goals.is_empty(), |this| {
                        this.child(
                            Label::new("No active goals")
                                .size(LabelSize::Small)
                        )
                    })
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Label::new("Completed Goals")
                            .size(LabelSize::Default)
                    )
                    .children(
                        completed_goals.iter().take(5).map(|goal| self.render_goal_item(goal))
                    )
            )
    }
    
    fn render_goal_item(&self, goal: &WritingGoal) -> impl IntoElement {
        let progress = goal.current_value as f32 / goal.target_value as f32;
        let is_completed = goal.status == GoalStatus::Completed;
        
        v_flex()
            .gap_2()
            .padding(px(12.))
            .border_1()
            .border_color(ui::colors::border())
            .rounded(px(4.))
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new(&goal.title)
                            .size(LabelSize::Default)
                    )
                    .child(
                        Label::new(&format!("{:?}", goal.priority))
                            .size(LabelSize::Small)
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new(&format!("{:?}", goal.goal_type))
                            .size(LabelSize::Small)
                    )
                    .child(
                        Label::new(&format!("{} / {}", goal.current_value, goal.target_value))
                            .size(LabelSize::Small)
                    )
            )
            .child(
                // 进度条
                h_flex()
                    .w_full()
                    .h(px(6.))
                    .bg(ui::colors::surface())
                    .rounded(px(3.))
                    .child(
                        h_flex()
                            .w(px((progress * 200.0).min(200.0)))
                            .h_full()
                            .bg(if is_completed { ui::colors::success() } else { ui::colors::accent() })
                            .rounded(px(3.))
                    )
            )
            .child(
                Label::new(&format!("{:.1}% complete", progress * 100.0))
                    .size(LabelSize::Small)
            )
    }
    
    fn render_analytics_tab(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.current_report.is_none() {
            self.current_report = Some(self.analytics.generate_report(self.report_period_days));
        }
        
        let report = self.current_report.as_ref().unwrap();
        
        v_flex()
            .gap_4()
            .child(self.render_report_controls(cx))
            .child(self.render_report_summary(report))
            .child(self.render_productivity_trend(report))
            .child(self.render_detailed_recommendations(report))
    }
    
    fn render_report_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                Label::new("Report Period:")
            )
            .child(self.period_button("7 days", 7, cx))
            .child(self.period_button("30 days", 30, cx))
            .child(self.period_button("90 days", 90, cx))
            .child(
                Button::new("refresh_report", "Refresh")
                    .style(ButtonStyle::Subtle)
                    .size(ButtonSize::Small)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.current_report = Some(this.analytics.generate_report(this.report_period_days));
                        cx.notify();
                    }))
            )
    }
    
    fn period_button(&self, label: &str, days: u32, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.report_period_days == days;
        
        Button::new(format!("period_{}", days), label)
            .style(if is_selected { ButtonStyle::Filled } else { ButtonStyle::Subtle })
            .size(ButtonSize::Small)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.report_period_days = days;
                this.current_report = Some(this.analytics.generate_report(days));
                cx.notify();
            }))
    }
    
    fn render_report_summary(&self, report: &WritingAnalysisReport) -> impl IntoElement {
        h_flex()
            .gap_4()
            .child(self.stat_card("Total Words", &report.total_words.to_string()))
            .child(self.stat_card("Total Time", &format!("{}h {}m", 
                report.total_time_minutes / 60, report.total_time_minutes % 60)))
            .child(self.stat_card("Avg Daily", &format!("{:.0} words", report.average_daily_words)))
            .child(self.stat_card("Avg Speed", &format!("{:.1} WPM", report.average_wpm)))
            .child(self.stat_card("Streak", &format!("{} days", report.streak_days)))
    }
    
    fn render_productivity_trend(&self, report: &WritingAnalysisReport) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Productivity Trend")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new(&format!("Trend: {:?}", report.productivity_trend))
                    )
                    .child(
                        Label::new(&format!("Goal Completion: {:.1}%", report.goal_completion_rate * 100.0))
                    )
            )
            .child(
                Label::new(&format!("Most Productive: {} at {}:00", 
                    report.most_productive_day
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    report.most_productive_hour.unwrap_or(0)))
                    .size(LabelSize::Small)
            )
    }
    
    fn render_detailed_recommendations(&self, report: &WritingAnalysisReport) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                Label::new("Detailed Recommendations")
                    .size(LabelSize::Default)
            )
            .children(
                report.recommendations.iter().map(|rec| {
                    Label::new(&format!("• {}", rec))
                        .size(LabelSize::Small)
                })
            )
    }
    
    fn render_text_analysis_tab(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                Label::new("Text Quality Analysis")
                    .size(LabelSize::Default)
            )
            .child(
                Label::new("Paste text below to analyze its readability:")
                    .size(LabelSize::Small)
            )
            .child(
                TextInput::new("analysis_text")
                    .placeholder("Enter text to analyze...")
            )
            .child(
                Button::new("analyze_text", "Analyze Text")
                    .style(ButtonStyle::Filled)
                    .on_click(cx.listener(|_this, _, _window, _cx| {
                        // TODO: Implement text analysis trigger
                    }))
            )
    }
    
    fn render_settings_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                Label::new("Writing Assistant Settings")
                    .size(LabelSize::Default)
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Auto-track Sessions"))
                    .child(
                        Toggle::new("auto_track")
                            .checked(self.auto_track_sessions)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.auto_track_sessions = checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Show Advanced Stats"))
                    .child(
                        Toggle::new("advanced_stats")
                            .checked(self.show_advanced_stats)
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.show_advanced_stats = checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Daily Word Goal:"))
                    .child(
                        TextInput::new("daily_goal")
                            .placeholder(&self.daily_word_goal.to_string())
                            .on_input(cx.listener(|this, input, _window, cx| {
                                if let Ok(goal) = input.parse::<u32>() {
                                    this.daily_word_goal = goal;
                                    cx.notify();
                                }
                            }))
                    )
            )
    }
}

impl Default for WritingAssistantView {
    fn default() -> Self {
        Self::new()
    }
}
