use pyo3::{prelude::*, types::PyModule, types::PyString, types::PyTuple};
use std::path::PathBuf;
use uuid::Uuid;

// 定义一个结构体来封装Python脚本的调用
#[derive(Clone)]
// Allow dead code for the struct and its methods, as its usage is primarily through PythonInterfaceHolder and Tauri commands.
#[allow(dead_code)]
pub struct PythonInterface {
    // 存储Python脚本的路径
    pub script_path: PathBuf,
    // 使用UUID作为模块标识符
    module_id: String,
}

#[allow(dead_code)] // Applying it here covers all associated items (new, call_python_function, load_python_module).
impl PythonInterface {
    // 初始化PythonInterface
    pub fn new(script_path: PathBuf) -> Self {
        PythonInterface {
            script_path,
            module_id: Uuid::new_v4().to_string(),
        }
    }

    // 调用Python脚本中的函数
    pub fn call_python_function(
        &self,
        function_name: &str,
        args: Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Python::with_gil(|py| {
            // 1. 加载Python模块
            // 明确指定转换目标类型
            let module = self
                .load_python_module(py)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>)?;

            // 2. 获取Python函数
            // 明确指定转换目标类型
            let function = module
                .getattr(function_name)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>)?;

            // 3. 准备参数
            let py_args: Vec<_> = args
                .iter()
                .map(|arg| PyString::new(py, arg).to_object(py))
                .collect();
            let py_args_tuple = PyTuple::new(py, py_args.as_slice());

            // 4. 调用Python函数
            // 明确指定转换目标类型
            let result = function
                .call1(py_args_tuple)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>)?;

            // 5. 处理结果
            // 明确指定转换目标类型
            result
                .extract::<String>()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>)
        })
    }

    // 加载Python模块
    fn load_python_module<'a>(&self, py: Python<'a>) -> PyResult<&'a PyModule> {
        let module_id = &self.module_id;
        let script_path = &self.script_path;

        // 1. 检查脚本是否存在
        if !script_path.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                format!("Python script not found: {:?}", script_path),
            ));
        }

        // 2. 添加脚本所在目录到Python的搜索路径
        let script_dir = script_path.parent().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Could not determine parent directory for script: {:?}",
                script_path
            ))
        })?;

        let syspath: &PyModule = PyModule::import(py, "sys")?;
        // 建议在 .to_str() 后面也做错误处理，而不是直接 unwrap()，以防路径不是有效的UTF-8
        let script_dir_str = script_dir.to_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
                format!("Script directory path {:?} contains invalid Unicode characters", script_dir)
            )
        })?;
        let _syspath_add = syspath
            .getattr("path")?
            .call_method1("insert", (0, script_dir_str))?;

        // 3. 导入Python模块
        let script_content = std::fs::read_to_string(script_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read script file {:?}: {}",
                script_path, e
            ))
        })?;

        let module = PyModule::from_code(py, &script_content, module_id, module_id)?;

        Ok(module)
    }
}
