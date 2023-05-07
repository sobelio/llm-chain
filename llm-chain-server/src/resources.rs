use llm_chain::serialization::{Envelope, StorableEntity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::RwLock;
use walkdir::WalkDir;

pub struct JsonFilesData {
    data: Arc<RwLock<HashMap<PathBuf, Arc<Envelope>>>>,
    folder_path: PathBuf,
}

async fn process_json_file<P: AsRef<Path>>(
    path: P,
) -> Result<Envelope, Box<dyn std::error::Error>> {
    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).await?;
    let my_struct = serde_json::from_str(&contents)?;
    Ok(my_struct)
}

impl JsonFilesData {
    pub async fn new<P: AsRef<Path>>(folder_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let folder_path = folder_path.as_ref().to_path_buf();
        let data = Arc::new(RwLock::new(HashMap::new()));
        let mut json_files_data = Self { data, folder_path };
        json_files_data.reload().await?;
        Ok(json_files_data)
    }

    pub async fn get_by_name(&self, name: &str) -> Option<Arc<Envelope>> {
        let data = self.data.read().await;
        let json_name = format!("{}.json", name);
        let s = data
            .iter()
            .find(|(path, _)| path.file_name().unwrap_or_default().to_string_lossy() == json_name)
            .map(|(_, my_struct)| my_struct)?;
        Some(s.clone())
    }

    pub async fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json_files_data = read_all_json_files_in_folder(&self.folder_path).await?;
        let mut data = self.data.write().await;
        data.clear();
        for (path, s) in json_files_data {
            data.insert(path, Arc::new(s));
        }
        Ok(())
    }
}

// The previous read_all_json_files_in_folder function should now return a HashMap with file paths as keys.
async fn read_all_json_files_in_folder<P: AsRef<Path>>(
    folder_path: P,
) -> Result<HashMap<PathBuf, Envelope>, Box<dyn std::error::Error>> {
    let mut results = HashMap::new();

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "json" {
            match process_json_file(entry.path()).await {
                Ok(my_struct) => {
                    results.insert(entry.path().to_path_buf(), my_struct);
                }
                Err(e) => eprintln!("Error processing {}: {}", entry.path().display(), e),
            }
        }
    }

    Ok(results)
}
