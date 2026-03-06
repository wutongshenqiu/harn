use harn_core::module::Module;

use crate::agent::AgentModule;
use crate::build::BuildModule;
use crate::ci::CiModule;
use crate::docker::DockerModule;
use crate::env::EnvModule;
use crate::git::GitModule;
use crate::ide::IdeModule;
use crate::quality::QualityModule;
use crate::sdd::SddModule;

/// Registry of all available modules.
///
/// Modules are ordered by recommended execution order:
/// docs first, then infrastructure, then tooling.
pub struct ModuleRegistry {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: vec![
                Box::new(SddModule),
                Box::new(GitModule),
                Box::new(EnvModule),
                Box::new(BuildModule),
                Box::new(CiModule),
                Box::new(AgentModule),
                Box::new(IdeModule),
                Box::new(DockerModule),
                Box::new(QualityModule),
            ],
        }
    }

    pub fn all(&self) -> &[Box<dyn Module>] {
        &self.modules
    }

    pub fn get(&self, id: &str) -> Option<&dyn Module> {
        self.modules.iter().find(|m| m.id() == id).map(|m| &**m)
    }

    pub fn ids(&self) -> Vec<&'static str> {
        self.modules.iter().map(|m| m.id()).collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
