/*
 Copyright 2026 seasnail1
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 
 */
use serde::Deserialize;
use crate::mc::plugin::Plugin;

#[derive(Debug, Deserialize, Clone)]
pub enum ServerBrand {
    Vanilla,
    Paper,
}

#[derive(Clone)]
pub struct BuildInfo {
    version: String,
}
 

#[derive(Clone)]
pub struct MinecraftServer {
    brand: ServerBrand,
    build: BuildInfo,
    name: Option<String>,
    plugins: Option<Vec<Plugin>>
}

impl MinecraftServer {
    pub(crate) fn new() -> Self {
        Self {
            brand: ServerBrand::Vanilla,
            build: BuildInfo {
                version: String::from("NaN"),
            },
            name: None,
            plugins: None
        }
    }

    pub(crate) fn with_version(&mut self, version: BuildInfo) -> &mut Self {
        self.build = version;
        self
    }

    pub(crate) fn with_name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }

    pub(crate) fn with_brand(&mut self, brand: ServerBrand) -> &mut Self {
        self.brand = brand;
        self
    }
    
    pub(crate) fn build_info(&self) -> &BuildInfo {
        &self.build
    }

    pub(crate) fn plugins(&self) -> Option<&Vec<Plugin>> {
        self.plugins.as_ref()
    }
    
    pub(crate) fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    
    pub(crate) fn brand(&self) -> &ServerBrand {
        &self.brand
    }
    
    pub(crate) fn version(&self) -> &str {
        &self.build.version
    }

    pub(crate) fn build(&self) -> &MinecraftServer {
        self
    }
}