use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub description: String,
    pub timeline_position: Option<u32>,
    pub characters: Vec<String>, // character IDs
    pub location: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Scene {
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Scene {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: String::new(),
            timeline_position: None,
            characters: Vec::new(),
            location: String::new(),
            created_at: now.clone(),
            updated_at: now,
        }
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
    
    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Enemy,
    Family,
    Romance,
    Colleague,
    Stranger,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub relationship_type: RelationshipType,
    pub description: String,
    pub strength: u8, // 1-10 scale
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: String,
    pub age: Option<u32>,
    pub occupation: String,
    pub personality_traits: Vec<String>,
    pub physical_description: String,
    pub backstory: String,
    pub relationships: HashMap<String, Relationship>, // other character ID -> relationship
    pub appearance_scenes: Vec<String>, // scene IDs
    pub created_at: String,
    pub updated_at: String,
}

impl Character {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Character {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            age: None,
            occupation: String::new(),
            personality_traits: Vec::new(),
            physical_description: String::new(),
            backstory: String::new(),
            relationships: HashMap::new(),
            appearance_scenes: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
    
    pub fn add_relationship(&mut self, other_character_id: String, relationship: Relationship) {
        self.relationships.insert(other_character_id, relationship);
        self.update_timestamp();
    }
    
    pub fn remove_relationship(&mut self, other_character_id: &str) {
        self.relationships.remove(other_character_id);
        self.update_timestamp();
    }
    
    pub fn add_scene_appearance(&mut self, scene_id: String) {
        if !self.appearance_scenes.contains(&scene_id) {
            self.appearance_scenes.push(scene_id);
            self.update_timestamp();
        }
    }
    
    pub fn remove_scene_appearance(&mut self, scene_id: &str) {
        self.appearance_scenes.retain(|id| id != scene_id);
        self.update_timestamp();
    }
    
    pub fn add_personality_trait(&mut self, trait_name: String) {
        if !self.personality_traits.contains(&trait_name) {
            self.personality_traits.push(trait_name);
            self.update_timestamp();
        }
    }
    
    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManuscriptFile {
    pub path: String,
    pub file_type: ManuscriptFileType,
    pub title: String,
    pub word_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManuscriptFileType {
    Chapter,
    Outline,
    Notes,
    Research,
    CharacterProfile,
    SceneDescription,
    Other,
}

impl ManuscriptFile {
    pub fn new(path: String, file_type: ManuscriptFileType) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let title = std::path::Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();
            
        ManuscriptFile {
            path,
            file_type,
            title,
            word_count: 0,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
