/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context};
use yaml_rust::{yaml, Yaml};

use g3_yaml::YamlDocPosition;

use super::{UserConfig, UserDynamicSource};

const DEFAULT_REFRESH_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub(crate) struct UserGroupConfig {
    name: String,
    position: Option<YamlDocPosition>,
    pub(crate) static_users: HashMap<String, Arc<UserConfig>>,
    pub(crate) dynamic_source: Option<UserDynamicSource>,
    pub(crate) refresh_interval: Duration,
}

impl UserGroupConfig {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn position(&self) -> Option<YamlDocPosition> {
        self.position.clone()
    }

    pub(crate) fn empty(name: &str) -> Self {
        UserGroupConfig {
            name: name.to_string(),
            position: None,
            static_users: HashMap::new(),
            dynamic_source: None,
            refresh_interval: DEFAULT_REFRESH_INTERVAL,
        }
    }

    pub(crate) fn new(position: Option<YamlDocPosition>) -> Self {
        UserGroupConfig {
            name: String::new(),
            position,
            static_users: HashMap::new(),
            dynamic_source: None,
            refresh_interval: DEFAULT_REFRESH_INTERVAL,
        }
    }

    pub(crate) fn parse(&mut self, map: &yaml::Hash) -> anyhow::Result<()> {
        g3_yaml::foreach_kv(map, |k, v| self.set(k, v))?;
        self.check()?;
        Ok(())
    }

    fn check(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            return Err(anyhow!("name is not set"));
        }

        Ok(())
    }

    fn set(&mut self, k: &str, v: &Yaml) -> anyhow::Result<()> {
        match g3_yaml::key::normalize(k).as_str() {
            "name" => {
                if let Yaml::String(name) = v {
                    self.name.clone_from(name);
                    Ok(())
                } else {
                    Err(anyhow!("invalid string value for key {k}"))
                }
            }
            "static_users" => {
                if let Yaml::Array(seq) = v {
                    for (i, obj) in seq.iter().enumerate() {
                        if let Yaml::Hash(map) = obj {
                            let user = Arc::new(UserConfig::parse_yaml(map)?);
                            let username = user.name().to_string();
                            if let Some(old) = self.static_users.insert(username, user) {
                                return Err(anyhow!(
                                    "found duplicate entry for user {}",
                                    old.name()
                                ));
                            }
                        } else {
                            return Err(anyhow!("invalid hash value for key {k}#{i}"));
                        }
                    }
                    Ok(())
                } else {
                    Err(anyhow!("invalid sequence value for key {k}"))
                }
            }
            "source" => {
                let lookup_dir = crate::config::get_lookup_dir(self.position.as_ref());
                self.dynamic_source = Some(
                    UserDynamicSource::parse_config(v, &lookup_dir)
                        .context(format!("invalid value for key {k}"))?,
                );
                Ok(())
            }
            "refresh_interval" => {
                self.refresh_interval = g3_yaml::humanize::as_duration(v)
                    .context(format!("invalid duration value for key {k}"))?;
                Ok(())
            }
            _ => Err(anyhow!("invalid key {k}")),
        }
    }
}
