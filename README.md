# rust-template

## 一、环境配置

### 1.1 安装 Rust

#### 1.1.1 Linux/MacOS

```bash
# 安装
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
# 检查
rustc --version
cargo --version
# 卸载
rustup self uninstall
```

#### 1.1.2 Windows

下载运行[rust-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)，然后执行以下命令安装工具链：

```bash
# 安装工具链解决报错：error: linker `link.exe` not found
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### 1.2 设置镜像源

#### 1.2.1 全局配置

新增配置文件：$HOME/.cargo/config

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

#### 1.2.2 当前项目配置

项目根目录下新增配置文件：.cargo/config.toml

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

### 1.3 配置开发工具

#### 1.3.1 VSCode 插件

- crates：Rust 包管理
- Even Better TOML：TOML 文件支持
- Better Comments：优化注释显示
- Error Lens：错误提示优化
- GitLens：Git 增强
- indent-rainbow：缩进显示优化
- Prettier - Code formatter：代码格式化
- REST client：REST API 测试
- rust-analyzer：Rust 语言支持
- Rust Test lens：Rust 测试支持
- Rust Test Explorer：Rust 测试概览
- TODO Highlight：TODO 高亮
- vscode-icons：图标优化
- YAML：YAML 文件支持

#### 1.3.2 VSCode 自动格式化与字体

Ctrl+Shift+P 搜索 settings.json 打开设置（JSON）：

```json
{
  "debug.console.fontSize": 18,
  "debug.console.fontFamily": "Dank Mono, Fira Code Light, Consolas, Microsoft YaHei",
  "terminal.integrated.fontSize": 18,
  "terminal.integrated.fontFamily": "Dank Mono, Fira Code Light, Consolas, Microsoft YaHei",
  "terminal.integrated.allowChords": false,
  "editor.fontSize": 18,
  "editor.fontFamily": "Dank Mono, Fira Code Light, Consolas, Microsoft YaHei",
  "editor.fontLigatures": true,
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

#### 1.3.3 其它工具

cargo generate：代码生成工具。

```bash
cargo install cargo-generate
```

[pre-commit](https://pre-commit.com/#install)：代码检查工作。

```bash
# 通过pip安装
pip install pre-commit
# 通过homebrew安装
brew install pre-commit
# 初始化项目
pre-commit install
```

deny：依赖安全性检查。

```bash
# 安装
cargo install cargo-deny --locked
# 初始化项目
cargo deny init
# 执行检查
cargo deny check
```

typos：拼写检查工具。

```bash
# 安装
cargo install typos-cli
```

git cliff：changelog 生成工具。

```bash
# 安装
cargo install git-cliff
# 生成CHANGELOG.md
git cliff -o CHANGELOG.md
```

cargo nextest：Rust 增强测试工具。

```bash
# 安装
cargo install cargo-nextest --locked
```
