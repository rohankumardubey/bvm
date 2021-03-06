use crate::plugins::{BinaryEnvironment, PlatformInfo, PlatformInfoCommand, SerializedPluginFile};
use crate::types::{CommandName, Version};
use std::collections::HashMap;

pub struct PluginFileBuilder {
  file: SerializedPluginFile,
  windows: Option<PlatformInfoBuilder>,
  linux: Option<PlatformInfoBuilder>,
  mac: Option<PlatformInfoBuilder>,
}

impl PluginFileBuilder {
  pub fn new() -> Self {
    PluginFileBuilder {
      file: SerializedPluginFile {
        schema_version: 1,
        owner: "owner".to_string(),
        name: "name".to_string(),
        version: "1.0.0".into(),
        description: "Some description.".to_string(),
        linux: None,
        mac: None,
        windows: None,
      },
      windows: None,
      linux: None,
      mac: None,
    }
  }

  pub fn build(&self) -> SerializedPluginFile {
    let mut file = self.file.clone();
    if let Some(builder) = &self.windows {
      file.windows = Some(builder.build());
    }
    if let Some(builder) = &self.linux {
      file.linux = Some(builder.build());
    }
    if let Some(builder) = &self.mac {
      file.mac = Some(builder.build());
    }
    file
  }

  pub fn get_name(&self) -> String {
    self.file.name.clone()
  }

  pub fn get_version(&self) -> Version {
    self.file.version.clone()
  }

  pub fn get_command_names(&mut self) -> Vec<String> {
    self.linux().get_command_names()
  }

  pub fn to_json_text(&self) -> String {
    serde_json::to_string(&self.build()).unwrap()
  }

  pub fn name<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PluginFileBuilder {
    self.file.name = value.as_ref().to_string();
    self
  }

  pub fn owner<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PluginFileBuilder {
    self.file.owner = value.as_ref().to_string();
    self
  }

  pub fn version<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PluginFileBuilder {
    self.file.version = value.as_ref().into();
    self
  }

  pub fn description<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PluginFileBuilder {
    self.file.description = value.as_ref().to_string();
    self
  }

  pub fn windows<'a>(&'a mut self) -> &'a mut PlatformInfoBuilder {
    if self.windows.is_none() {
      self.windows = Some(PlatformInfoBuilder::new());
    }
    self.windows.as_mut().unwrap()
  }

  pub fn linux<'a>(&'a mut self) -> &'a mut PlatformInfoBuilder {
    if self.linux.is_none() {
      self.linux = Some(PlatformInfoBuilder::new());
    }
    self.linux.as_mut().unwrap()
  }

  pub fn mac<'a>(&'a mut self) -> &'a mut PlatformInfoBuilder {
    if self.mac.is_none() {
      self.mac = Some(PlatformInfoBuilder::new());
    }
    self.mac.as_mut().unwrap()
  }
}

pub struct PlatformInfoBuilder {
  info: PlatformInfo,
}

impl PlatformInfoBuilder {
  pub fn new() -> Self {
    PlatformInfoBuilder {
      info: PlatformInfo {
        path: "".to_string(),
        checksum: "".to_string(),
        download_type: "zip".to_string(),
        commands: Vec::new(),
        output_dir: None,
        on_pre_install: None,
        on_post_install: None,
        environment: None,
      },
    }
  }

  pub fn build(&self) -> PlatformInfo {
    self.info.clone()
  }

  pub fn get_command_names(&self) -> Vec<String> {
    self.info.commands.iter().map(|c| c.name.as_str().to_string()).collect()
  }

  pub fn path<'a>(&'a mut self, value: &str) -> &'a mut PlatformInfoBuilder {
    self.info.path = value.to_string();
    self
  }

  pub fn checksum<'a>(&'a mut self, value: &str) -> &'a mut PlatformInfoBuilder {
    self.info.checksum = value.to_string();
    self
  }

  pub fn download_type<'a>(&'a mut self, value: &str) -> &'a mut PlatformInfoBuilder {
    self.info.download_type = value.to_string();
    self
  }

  pub fn add_command<'a>(&'a mut self, name: impl AsRef<str>, path: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.info.commands.push(PlatformInfoCommand {
      name: CommandName::from_string(name.as_ref().to_string()),
      path: path.as_ref().to_string(),
    });
    self
  }

  pub fn output_dir<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.info.output_dir = Some(value.as_ref().to_string());
    self
  }

  pub fn on_pre_install<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.info.on_pre_install = Some(value.as_ref().to_string());
    self
  }

  pub fn on_post_install<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.info.on_post_install = Some(value.as_ref().to_string());
    self
  }

  pub fn add_env_path<'a>(&'a mut self, value: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.ensure_environment();
    self
      .info
      .environment
      .as_mut()
      .unwrap()
      .paths
      .as_mut()
      .unwrap()
      .push(value.as_ref().to_string());
    self
  }

  pub fn add_env_var<'a>(&'a mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &'a mut PlatformInfoBuilder {
    self.ensure_environment();
    self
      .info
      .environment
      .as_mut()
      .unwrap()
      .variables
      .as_mut()
      .unwrap()
      .insert(key.as_ref().to_string(), value.as_ref().to_string());
    self
  }

  fn ensure_environment(&mut self) {
    if self.info.environment.is_none() {
      self.info.environment = Some(BinaryEnvironment {
        paths: Some(Vec::new()),
        variables: Some(HashMap::new()),
      });
    }
  }
}
