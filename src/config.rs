use anyhow::{Context, Result};
use crate::metadata::{ScriptMetadata, ActivityMetadata};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub struct XConfig {
    pub base_dir: PathBuf,
    pub scripts_dir: PathBuf,
    pub apps_dir: PathBuf,
    pub metadata_dir: PathBuf,
    pub activity_metadata_path: PathBuf,
    pub config_path: PathBuf,
}

impl XConfig {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .context("Could not find home directory")?;
        let base_dir = home.join(".x.sh");
        let scripts_dir = base_dir.join("scripts");
        let apps_dir = base_dir.join("apps");
        let metadata_dir = base_dir.join("metadata");
        let activity_metadata_path = base_dir.join("metadata.json");
        let config_path = base_dir.join("config.json");
        
        Ok(Self {
            base_dir,
            scripts_dir,
            apps_dir,
            metadata_dir,
            activity_metadata_path,
            config_path,
        })
    }
    
    pub fn ensure_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.scripts_dir)
            .context("Failed to create scripts directory")?;
        fs::create_dir_all(&self.metadata_dir)
            .context("Failed to create metadata directory")?;
        Ok(())
    }
    
    pub fn ensure_app_dirs(&self) -> Result<()> {
        fs::create_dir_all(&self.apps_dir)
            .context("Failed to create apps directory")?;
        Ok(())
    }
    
    /// Compute the path of an app config file in the global apps directory.
    pub fn global_app_path(&self, name: &str) -> PathBuf {
        self.apps_dir.join(format!("{}.x.yml", name))
    }
    
    /// Compute the path of an app config file in the current working directory.
    pub fn local_app_path(name: &str) -> Result<PathBuf> {
        let cwd = std::env::current_dir()
            .context("Could not get current directory")?;
        Ok(cwd.join(format!("{}.x.yml", name)))
    }
    
    /// Find an app config file by name.
    ///
    /// Resolution order:
    /// 1. Current directory
    /// 2. Each parent directory recursively (nearest first)
    /// 3. Global apps directory (`~/.x.sh/apps`)
    ///
    /// Returns the absolute path if found.
    pub fn find_app(&self, name: &str) -> Result<Option<PathBuf>> {
        let app_filename = format!("{}.x.yml", name);
        let cwd = std::env::current_dir()
            .context("Could not get current directory")?;

        for dir in cwd.ancestors() {
            let candidate = dir.join(&app_filename);
            if candidate.is_file() {
                return Ok(Some(candidate));
            }
        }

        let global = self.global_app_path(name);
        if global.is_file() {
            return Ok(Some(global));
        }
        Ok(None)
    }
    
    pub fn get_metadata_path(&self, name: &str) -> PathBuf {
        self.metadata_dir.join(format!("{}.toml", name))
    }
    
    pub fn load_metadata(&self, name: &str) -> Result<Option<ScriptMetadata>> {
        let metadata_path = self.get_metadata_path(name);
        
        if !metadata_path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&metadata_path)
            .context(format!("Failed to read metadata file: {}", metadata_path.display()))?;
        let metadata: ScriptMetadata = toml::from_str(&content)
            .context(format!("Failed to parse metadata file: {}", metadata_path.display()))?;
        Ok(Some(metadata))
    }
    
    pub fn save_metadata(&self, name: &str, metadata: &ScriptMetadata) -> Result<()> {
        let metadata_path = self.get_metadata_path(name);
        let content = toml::to_string_pretty(metadata)
            .context("Failed to serialize metadata")?;
        fs::write(&metadata_path, content)
            .context(format!("Failed to write metadata file: {}", metadata_path.display()))?;
        Ok(())
    }
    
    pub fn remove_metadata(&self, name: &str) -> Result<()> {
        let metadata_path = self.get_metadata_path(name);
        if metadata_path.exists() {
            fs::remove_file(&metadata_path)
                .context(format!("Failed to remove metadata file: {}", metadata_path.display()))?;
        }
        Ok(())
    }
    
    pub fn load_activity_metadata(&self) -> Result<ActivityMetadata> {
        if !self.activity_metadata_path.exists() {
            return Ok(ActivityMetadata {
                scripts: std::collections::HashMap::new(),
            });
        }
        
        let content = fs::read_to_string(&self.activity_metadata_path)
            .context("Failed to read activity metadata file")?;
        let metadata: ActivityMetadata = serde_json::from_str(&content)
            .context("Failed to parse activity metadata file")?;
        Ok(metadata)
    }
    
    pub fn save_activity_metadata(&self, metadata: &ActivityMetadata) -> Result<()> {
        let content = serde_json::to_string_pretty(metadata)
            .context("Failed to serialize activity metadata")?;
        fs::write(&self.activity_metadata_path, content)
            .context("Failed to write activity metadata file")?;
        Ok(())
    }
    
    pub fn get_script_path(&self, name: &str) -> PathBuf {
        self.scripts_dir.join(name)
    }
    
    /// Find a script file by name, trying exact match first, then files where the name (without extension) matches
    /// Returns the full filename if found, or None if not found
    /// App names discovered in resolution order: CWD ancestors (nearest wins), then global.
    pub fn list_app_names(&self) -> Result<Vec<String>> {
        let mut seen = HashSet::new();
        let mut names = Vec::new();
        let cwd = std::env::current_dir().context("Could not get current directory")?;

        for dir in cwd.ancestors() {
            let Ok(entries) = fs::read_dir(dir) else {
                continue;
            };
            let mut dir_names: Vec<String> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if !path.is_file() {
                        return None;
                    }
                    let file_name = path.file_name()?.to_str()?;
                    file_name
                        .strip_suffix(".x.yml")
                        .map(|name| name.to_string())
                })
                .collect();
            dir_names.sort();
            for name in dir_names {
                if seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }

        if self.apps_dir.is_dir() {
            let mut global_names: Vec<String> = fs::read_dir(&self.apps_dir)
                .context("Failed to read apps directory")?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if !path.is_file() {
                        return None;
                    }
                    let file_name = path.file_name()?.to_str()?;
                    file_name
                        .strip_suffix(".x.yml")
                        .map(|name| name.to_string())
                })
                .collect();
            global_names.sort();
            for name in global_names {
                if seen.insert(name.clone()) {
                    names.push(name);
                }
            }
        }

        Ok(names)
    }

    /// Global script display names (basename without extension), sorted.
    pub fn list_script_names(&self) -> Result<Vec<String>> {
        if !self.scripts_dir.exists() {
            return Ok(Vec::new());
        }

        let mut names: Vec<String> = fs::read_dir(&self.scripts_dir)
            .context("Failed to read scripts directory")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if !path.is_file() {
                    return None;
                }
                let file_name = path.file_name()?.to_str()?;
                let display_name = if let Some(dot_pos) = file_name.rfind('.') {
                    &file_name[..dot_pos]
                } else {
                    file_name
                };
                Some(display_name.to_string())
            })
            .collect();

        names.sort();
        names.dedup();
        Ok(names)
    }

    pub fn find_script(&self, name: &str) -> Result<Option<String>> {
        if !self.scripts_dir.exists() {
            return Ok(None);
        }
        
        // First try exact match
        let exact_path = self.scripts_dir.join(name);
        if exact_path.exists() && exact_path.is_file() {
            return Ok(Some(name.to_string()));
        }
        
        // Then try to find files where the name (without extension) matches
        let entries = fs::read_dir(&self.scripts_dir)
            .context("Failed to read scripts directory")?;
        
        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // Check if the file name (without extension) matches the requested name
                // This handles cases like "c-test" matching "c-test.c"
                if let Some(dot_pos) = file_name.find('.') {
                    let name_without_ext = &file_name[..dot_pos];
                    if name_without_ext == name {
                        return Ok(Some(file_name.to_string()));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    pub fn load_default_program(&self) -> Result<Option<String>> {
        if !self.config_path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&self.config_path)
            .context("Failed to read config file")?;
        let config: serde_json::Value = serde_json::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config.get("default_program")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    }
    
    pub fn save_default_program(&self, program: &str) -> Result<()> {
        let config = serde_json::json!({
            "default_program": program
        });
        let content = serde_json::to_string_pretty(&config)
            .context("Failed to serialize config")?;
        fs::write(&self.config_path, content)
            .context("Failed to write config file")?;
        Ok(())
    }
}

