use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
    sync::{Arc, Mutex},
};

use asset_parser::{
    content_package::{AnyContentPackage, ContentFiles},
    loading::ConflictType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct ModManifest {
    //save as identifier + hash
    pub dependencies: Vec<Arc<Mutex<ModIdentifier>>>,
    //TODO: detect mod changes and reset the resolved state
    pub resolved_conflicts: HashMap<ConflictType, HashSet<ConflictStoreData>>,
    pub in_progress_conflicts: HashMap<ConflictType, HashSet<ConflictStoreData>>,
}

impl ModManifest {
    pub fn detect_mod_changes(
        &self,
        loaded_content_files: &Vec<(Arc<AnyContentPackage>, ContentFiles)>,
    ) -> Result<(), ModChangeDetectError> {
        let mod_hashes = loaded_content_files
            .iter()
            .map(|(p, _)| (p.package_id_prefer_ugc_id(), p.expected_hash()))
            .collect::<HashMap<_, _>>();

        let mut mods_with_changed_hash = Vec::new();
        for dep in &self.dependencies {
            let mut lock = dep.lock().unwrap();
            let Some(hash) = mod_hashes.get(&lock.identifier) else {
                return Err(ModChangeDetectError::ExpectedModNotLoaded(
                    lock.identifier.clone(),
                ));
            };

            match hash {
                Some(hash) => match &lock.mod_hash {
                    Some(dep_hash) => {
                        if dep_hash != hash {
                            mods_with_changed_hash.push(dep.clone());
                            log::info!(
                                "Hash for mod {} changed from {} to {}, setting the new hash for this mod.",
                                lock.identifier,
                                dep_hash,
                                hash
                            );
                            lock.mod_hash = Some(hash.clone())
                        }
                    }
                    None => {
                        log::info!(
                            "Hash for mod {} was not set for this Patch Mod, but was found during mod change detection, setting the new hash for this mod.",
                            lock.identifier
                        );
                        lock.mod_hash = Some(hash.clone())
                    }
                },
                None => {
                    if lock.mod_hash.is_some() {
                        log::warn!(
                            "Hash for mod {} is set in the Patch Mod but not in the mod itself, mod developer mistake? The hash will be removed in the Patch Mod and will not be checked later.",
                            lock.identifier
                        ); //how did we even get here?
                        lock.mod_hash = None;
                    } else {
                        log::warn!(
                            "Hash for mod {} is not set, it will not be checked for confict detection!",
                            lock.identifier
                        );
                    }
                }
            }
        }

        if mods_with_changed_hash.is_empty() {
            Ok(())
        } else {
            Err(ModChangeDetectError::ModChangesDetected(
                mods_with_changed_hash,
            ))
        }
    }

    pub fn load(path: &Path) -> Result<Self, LoadError> {
        let save: ModManifestSave = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        let dependencies = save
            .dependencies
            .into_iter()
            .map(Mutex::new)
            .map(Arc::new)
            .collect::<Vec<_>>();
        let mod_map: HashMap<String, &Arc<Mutex<ModIdentifier>>> = dependencies
            .iter()
            .map(|v| (v.lock().unwrap().identifier.clone(), v))
            .collect();
        Ok(ModManifest {
            resolved_conflicts: save.resolved_conflicts.into_iter().map(|(k, v)| (k, v.into_iter().map(|v| ConflictStoreData {
                identifier: v.identifier,
                conflict_between: v.conflict_between.into_iter().map(|v| (*mod_map.get(&v).expect("Invalid mod manifest: mod used in a conflict was not found in this mod's dependencies")).clone()).collect()
            }).collect())).collect(),
            in_progress_conflicts: save.in_progress_conflicts.into_iter().map(|(k, v)| (k, v.into_iter().map(|v| ConflictStoreData {
                identifier: v.identifier,
                conflict_between: v.conflict_between.into_iter().map(|v| (*mod_map.get(&v).expect("Invalid mod manifest: mod used in a conflict was not found in this mod's dependencies")).clone()).collect()
            }).collect())).collect(),
            dependencies,
        })
    }

    pub fn save(&self, path: &Path) -> Result<(), LoadError> {
        let save = ModManifestSave {
            dependencies: self
                .dependencies
                .iter()
                .map(|v| (v.lock().unwrap()).clone())
                .collect(),
            resolved_conflicts: self
                .resolved_conflicts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|v| ConflictStoreDataSave {
                                identifier: v.identifier.clone(),
                                conflict_between: v
                                    .conflict_between
                                    .iter()
                                    .map(|v| v.lock().unwrap().identifier.clone())
                                    .collect(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            in_progress_conflicts: self
                .in_progress_conflicts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|v| ConflictStoreDataSave {
                                identifier: v.identifier.clone(),
                                conflict_between: v
                                    .conflict_between
                                    .iter()
                                    .map(|v| v.lock().unwrap().identifier.clone())
                                    .collect(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        };
        Ok(std::fs::write(path, serde_json::to_string_pretty(&save)?)?)
    }
}

pub enum ModChangeDetectError {
    ModChangesDetected(Vec<Arc<Mutex<ModIdentifier>>>),
    ExpectedModNotLoaded(String),
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for LoadError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModIdentifier {
    pub identifier: String,
    pub mod_hash: Option<String>,
}

#[derive(Debug)]
pub struct ConflictStoreData {
    pub identifier: String,
    //save as identifier
    pub conflict_between: Vec<Arc<Mutex<ModIdentifier>>>,
}

impl PartialEq for ConflictStoreData {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

impl Eq for ConflictStoreData {}

impl Hash for ConflictStoreData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

#[derive(Serialize, Deserialize)]
struct ModManifestSave {
    pub dependencies: Vec<ModIdentifier>,
    pub resolved_conflicts: HashMap<ConflictType, HashSet<ConflictStoreDataSave>>,
    pub in_progress_conflicts: HashMap<ConflictType, HashSet<ConflictStoreDataSave>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConflictStoreDataSave {
    pub identifier: String,
    //save as identifier
    pub conflict_between: Vec<String>,
}
