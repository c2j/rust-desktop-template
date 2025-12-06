# 快速开始指南

欢迎使用 **{{project-name}}** 桌面应用程序！本指南将帮助您快速上手。

## 🚀 快速开始

### 前置要求

- 通过 [rustup](https://rustup.rs/) 安装 Rust 1.75+ 版本
- 具备基本的 Rust 编程知识

### 创建您的项目

```bash
cargo generate --git https://github.com/your-org/rust-desktop-template.git
cd <您的项目名称>
cargo run
```

就这样！您现在就有了一个具有 VS Code 类似界面的可用桌面应用程序。

## 📖 应用程序界面

您的新应用程序包含以下功能：

### 导航侧边栏（左侧）
- 🏠 **首页** - 仪表板和概览
- 📁 **文件浏览器** - 浏览和管理文件
- 📝 **文本编辑器** - 带语法高亮的文本编辑
- 📊 **系统监控器** - 查看系统资源使用情况
- ⚙️ **设置** - 配置应用程序设置

### 主内容区域（中央）
- **动态内容** - 根据所选模块更改内容
- **模块特定功能** - 每个模块提供自己的功能
- **响应式布局** - 适应窗口大小调整

### 状态栏（底部）
- 应用程序状态和版本
- 当前活动模块
- 系统信息
- 当前时间

## 🎯 常见任务

### 文件管理

1. 在导航中点击 **📁 文件浏览器**
2. 使用文件树浏览目录
3. 双击文件夹进入
4. 双击文件打开
5. 使用工具栏按钮进行其他操作

### 文本编辑

1. 在导航中点击 **📝 文本编辑器**
2. 开始输入编辑欢迎文本
3. 使用 **📂 打开** 加载文本文件
4. 使用 **💾 保存** 保存您的更改
5. 使用键盘快捷键提高效率

### 系统监控

1. 在导航中点击 **📊 系统监控器**
2. 查看实时 CPU 和内存使用情况
3. 监控磁盘和网络活动
4. 查看资源消耗最高的进程

## 🎨 自定义

### 主题自定义

模板提供了多个内置主题：

- **深色主题** - 护眼（默认）
- **浅色主题** - 明亮清晰
- **高对比度** - 最大可访问性
- **褐色主题** - 温暖，复古外观
- **海洋主题** - 清凉，水色调
- **森林主题** - 自然，大地色调
- **日落主题** - 温暖，日落色彩

要更改主题，编辑 `project.toml`：

```toml
[project]
default_theme = "ocean"  # 或 "light", "dark" 等
```

### 自定义主题

通过编辑 `theme.json` 创建自定义主题：

```json
{
  "name": "我的自定义主题",
  "type": "custom",
  "colors": {
    "primary": "#007ACC",
    "background": "#1E1E1E",
    "text": "#D4D4D4",
    "border": "#454545"
  }
}
```

### 模块自定义

通过实现 `ApplicationModule` trait 添加自定义模块：

```rust
use crate::modules::ApplicationModule;
use egui::{Context, Ui};

pub struct MyCustomModule {
    // 您的模块状态
}

impl ApplicationModule for MyCustomModule {
    fn id(&self) -> crate::modules::ModuleId {
        crate::modules::ModuleId::Custom(1)
    }

    fn name(&self) -> &str {
        "我的自定义模块"
    }

    fn icon(&self) -> &str {
        "🚀"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.heading("我的自定义模块");
        ui.label("这里是您的内容！");
    }
}
```

## ⚙️ 配置

### 项目配置

编辑 `project.toml` 来自定义项目设置：

```toml
[project]
name = "我的桌面应用"
version = "0.1.0"
default_theme = "dark"

[layout]
left_sidebar_width = 48.0
right_sidebar_width = 300.0
show_status_bar = true
window_min_size = [800, 600]
window_default_size = [1200, 800]
```

### 模块配置

在 `config/modules/my_module.toml` 中创建模块特定设置：

```toml
[module]
enabled = true
auto_load = true
auto_activate = false

[ui]
show_in_navigation = true
navigation_position = 5
shortcut = "Ctrl+Shift+M"

[permissions]
file_system_access = true
network_access = false
system_info_access = false
settings_access = true

[settings]
custom_setting = "value"
number_setting = 42
```

## 🔧 开发

### 构建您的应用程序

```bash
# 开发构建（更快，包含调试）
cargo run

# 发布构建（针对生产优化）
cargo build --release

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### 跨平台构建

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### 添加依赖

将依赖添加到 `Cargo.toml`：

```toml
[dependencies]
# 在这里添加您的依赖
your-crate = "1.0.0"
```

然后运行：

```bash
cargo check  # 验证依赖解析
```

## 🧪 测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test module_name

# 带输出运行测试
cargo test -- --nocapture

# 并行运行测试
cargo test --test-threads=4
```

### 编写测试

在 `tests/` 中创建单元测试：

```rust
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = MyCustomModule::new();
        assert_eq!(module.name(), "我的自定义模块");
    }

    #[test]
    fn test_ui_rendering() {
        // 在这里测试 UI 渲染
    }
}
```

### 集成测试

在 `tests/integration/` 中创建集成测试：

```rust
use crate::MyCustomModule;

#[test]
fn test_module_integration() {
    let app = crate::App::new(&cc);
    // 在这里测试集成场景
}
```

## 📚 文档

### 生成的文档

```bash
# 生成并查看文档
cargo doc --open
```

### 代码文档

使用 `///` 注释来记录您的代码：

```rust
/// 将两个数字相加
///
/// # 参数
/// * `a` - 要相加的第一个数字
/// * `b` - 要相加的第二个数字
///
/// # 返回值
/// `a` 和 `b` 的和
///
/// # 示例
/// ```
/// let result = add_numbers(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

## 🐛 部署

### 创建发行版

```bash
# 创建发布构建
cargo build --release

# 二进制文件将位于：
# target/release/your-app (Linux/macOS)
# target/release/your-app.exe (Windows)
```

### 跨平台包

对于平台特定包，使用以下工具：

- **Windows**: NSIS, WiX Toolset
- **macOS**: `create-dmg`, `pkgbuild`
- **Linux**: AppImage, Debian 包

### 代码签名

对于分发，对您的应用程序进行签名：

```bash
# macOS 代码签名
codesign -s "Developer ID Application: Your ID" target/release/your-app

# Windows 代码签名
signtool sign /a /fd "您的证书" target/release/your-app.exe
```

## 🔍 调试

### 日志记录

应用程序使用带有 tracing 的结构化日志。启用调试日志：

```bash
RUST_LOG=debug cargo run
```

### 崩溃信息

对于崩溃信息，设置回溯：

```bash
RUST_BACKTRACE=1 cargo run
```

### 性能分析

使用性能分析工具分析性能：

```bash
# CPU 性能分析
cargo install cargo-flamegraph
cargo flamegraph --bin target/release/your-app

# 内存性能分析
cargo install cargo-valgrind
valgrind --tool=massif target/release/your-app
```

## 🤝 获取帮助

### 常见问题

**应用程序无法启动：**
1. 检查 Rust 版本：`rustc --version`
2. 更新依赖：`cargo update`
3. 清理构建：`cargo clean && cargo build`

**UI 看起来不对：**
1. 检查主题设置
2. 验证 egui/eframe 版本
3. 检查系统字体可用性

**性能问题：**
1. 使用发布构建：`cargo build --release`
2. 使用上述工具进行性能分析
3. 检查渲染循环中的昂贵操作

### 社区支持

- 🐛 [问题](https://github.com/your-org/rust-desktop-template/issues) - 报告错误和请求功能
- 💬 [讨论](https://github.com/your-org/rust-desktop-template/discussions) - 提问和分享想法
- 📧 [Wiki](https://github.com/your-org/rust-desktop-template/wiki) - 附加文档

### 专业支持

对于企业支持或定制开发服务：

- 📧 邮箱：support@example.com
- 💬 网站：https://your-company.com
- 📞 电话：+1 (555) 123-4567

## 🎉 后续步骤

恭喜！您现在有了一个可用的 Rust 桌面应用程序。这里有一些您可以下一步做的事情：

### 即时改进

1. **添加您的第一个模块**：为您的特定用例实现自定义模块
2. **自定义主题**：创建独特的配色方案或视觉风格
3. **添加设置**：创建应用程序特定的配置选项
4. **添加测试**：为您的功能编写全面的测试

### 高级功能

1. **数据库集成**：添加数据库支持以进行数据持久化
2. **网络功能**：实现 REST API 客户端或网页视图
3. **插件系统**：创建可扩展的插件架构
4. **自动化**：添加脚本功能或任务自动化
5. **国际化**：添加多语言支持

### 部署

1. **分发**：为目标平台创建安装程序
2. **自动更新器**：实现自动更新机制
3. **遥测**：添加使用分析和崩溃报告
4. **CI/CD**：设置自动化构建和发布

---

**祝您编码愉快！** 🚀

更多详细信息，请参阅 [架构概览](architecture.md) 或 [模块开发指南](module-development.md)。