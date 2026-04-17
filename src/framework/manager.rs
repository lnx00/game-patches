use crate::{
    config::CONFIG,
    framework::patch::Patch,
};

struct ManagedPatch {
    patch: Box<dyn Patch>,
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

    pub fn register(&mut self, patch: Result<Box<dyn Patch>, String>) {
        match patch {
            Ok(patch) => {
                let patch_name = patch.name();
                self.patches.push(ManagedPatch {
                    patch,
                    is_applied: false,
                });

                println!("- registered patch '{}'", patch_name);
            }
            Err(e) => {
                eprintln!("- failed to register patch: {}", e);
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
                println!("- skipping patch '{}': disabled", managed.patch.name());
                continue;
            }

            if managed.is_applied {
                println!(
                    "- skipping patch '{}': already applied",
                    managed.patch.name()
                );
                continue;
            }

            match managed.patch.apply() {
                Ok(_) => {
                    println!("- applied patch '{}'", managed.patch.name());
                    managed.is_applied = true;
                }
                Err(e) => eprintln!("- failed to apply patch '{}': {}", managed.patch.name(), e),
            }
        }
    }

    pub fn revert_all(&mut self) {
        for managed in self.patches.iter_mut().rev() {
            if managed.is_applied {
                if let Err(e) = managed.patch.revert() {
                    eprintln!("- faild to revert patch '{}': {}", managed.patch.name(), e);
                } else {
                    println!("- reverted patch '{}'", managed.patch.name());
                }
            }
        }
    }
}
