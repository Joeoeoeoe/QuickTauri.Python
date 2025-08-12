// 缓存：用于存储当前脚本的 Python 接口实例；通过路径识别python脚本
use std::path::PathBuf;
use std::sync::Mutex;
use crate::python_interface::PythonInterface;

#[derive(Default)]
pub struct InstanceCache {
    current_path: Option<PathBuf>,
    interface: Option<PythonInterface>,
}

#[allow(dead_code)]
impl InstanceCache {
    pub fn new() -> Self {
        Self {
            current_path: None,
            interface: None,
        }
    }

    pub fn get_or_create(&mut self, script_path: PathBuf) -> &PythonInterface {
        if let Some(current_path) = &self.current_path {
            if current_path == &script_path {
                return self.interface.as_ref().unwrap();
            }
        }

        self.current_path = Some(script_path.clone());
        self.interface = Some(PythonInterface::new(script_path));
        self.interface.as_ref().unwrap()
    }
}

#[allow(dead_code)]
pub type CachedInterface = Mutex<InstanceCache>;