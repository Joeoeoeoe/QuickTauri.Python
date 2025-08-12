# develop.md —— 项目开发及二次开发的辅助文档

## 添加工具
1. 指定工具的前端界面和路由
   - 方式1：在 src/stores/toolStore.js 中直接更新 categories 和 toolComponents.value
   - 方式2：根据工具添加实际，在合适的位置调用 toolStore.js 中的函数进行注册
        ```js
        import { useToolStore } from '../stores/toolStore';
        const toolStore = useToolStore();
        toolStore.registerToolComponent('ImageMergeTool', () => import('../components/ImageMergeTool.vue'));
        ```

2. 编辑上面添加的.vue文件，自定义布局

3. 编写需要交互的python函数
   - 在 src-tauri/python_scripts 下新建.py文件并编写函数
   - 将使用的库放入 requirements.txt
   - 激活 .venv 虚拟环境 (.venv\Scripts\activate.bat)
   - pip install -r requirements.txt 安装相关库

4. 调用编写的python函数
      - callPython调用 xxx.py/def yyy():pass, 三个参数分别是：
        - .py相对于src-tauri的路径或绝对路径
        - python函数名（用字符串）
        - 若干字符串类型参数
      - .vue 中调用zzz函数

    ```js
    // 在需要进行调用的.vue中编写：
    import { callPython } from '@/services/rustService';
    const zzz = async () => {
    try {
        greeting.value = await callPython(
        './python_scripts/xxx.py',
        'yyy', 
        str_arg_1,
        str_arg_2
        );
    } catch (e) {
        console.log('callPython error: ', e)
    }
    };
    ```
