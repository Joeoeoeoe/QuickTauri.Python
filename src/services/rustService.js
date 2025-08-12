import { invoke } from '@tauri-apps/api/core'

export async function callPython(scriptPath, functionName, ...args) {
  try {
    const result = await invoke('call_python_script', {
      scriptPath,
      functionName,
      args,
    });
    return result;
  } catch (error) {
    console.error("Error calling Python:", error);
    throw error; // 重新抛出错误
  }
}
