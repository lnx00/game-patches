use crate::{config::CONFIG, framework::patch::Patch};

struct ManagedPatch {
    patch: Box<dyn Patch>,
    name: &'static str,
    is_applied: bool,
}

pub struct PatchManager {
    patches: Vec<ManagedPatch>,
}

impl PatchManager {
    pub fn new() -> Self {
        Self {
            patches: Vec::new(),
        }
    }

    pub fn register<P: Patch + 'static>(&mut self) {
        match P::init() {
            Ok(patch) => {
                self.patches.push(ManagedPatch {
                    patch,
                    name: P::name(),
                    is_applied: false,
                });

                println!("- registered patch '{}'", P::name());
            }
            Err(e) => {
                eprintln!("- failed to register patch '{}': {}", P::name(), e);
            }
        };
    }

    pub fn apply_all(&mut self) {
        for managed in &mut self.patches {
            let enabled = match managed.patch.config_key() {
                Some(key) => CONFIG.patch_enabled(key, true),
                None => true,
            };

            if !enabled {
                println!("- skipping patch '{}': disabled", managed.name);
                continue;
            }

            if managed.is_applied {
                println!("- skipping patch '{}': already applied", managed.name);
                continue;
            }

            match managed.patch.apply() {
                Ok(_) => {
                    println!("- applied patch '{}'", managed.name);
                    managed.is_applied = true;
                }
                Err(e) => eprintln!("- failed to apply patch '{}': {}", managed.name, e),
            }
        }
    }

    pub fn revert_all(&mut self) {
        for managed in self.patches.iter_mut().rev() {
            if managed.is_applied {
                if let Err(e) = managed.patch.revert() {
                    eprintln!("- faild to revert patch '{}': {}", managed.name, e);
                } else {
                    println!("- reverted patch '{}'", managed.name);
                }
            }
        }
    }
}
