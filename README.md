# RCLI - Rust CSV 处理命令行工具

一个使用 Rust 编写的 CSV 文件处理命令行工具，可以将 CSV 文件转换为 JSON、YAML 或 TOML 格式。

## 功能特性

- 📄 读取和解析 CSV 文件
- 🔄 支持将 CSV 转换为多种格式（JSON、YAML、TOML）
- 🎯 灵活的命令行参数配置
- ⚡ 高性能的 Rust 实现

## 环境要求

### Rust 工具链

本项目需要 Rust 开发环境。如果您还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 安装 Rust 工具链。

**最低要求：**
- Rust 1.70.0 或更高版本
- Cargo（随 Rust 工具链一起安装）

### 安装 Rust

```bash
# 使用 rustup 安装 Rust（推荐）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或使用 Homebrew（macOS）
brew install rust
```

## 依赖包

项目依赖的 Rust 包（已在 `Cargo.toml` 中配置）：

| 包名 | 版本 | 用途 |
|------|------|------|
| `anyhow` | 1.0.100 | 错误处理和异常管理 |
| `clap` | 4.5.51 | 命令行参数解析（支持 derive 特性） |
| `csv` | 1.4.0 | CSV 文件读写处理 |
| `serde` | 1.0.228 | 序列化和反序列化框架（支持 derive 特性） |
| `serde_json` | 1.0.145 | JSON 格式支持 |
| `serde_yaml` | 0.9.34 | YAML 格式支持 |

## 安装

### 从源码构建

```bash
# 克隆仓库（如果适用）
git clone <repository-url>
cd rcli

# 编译项目
cargo build --release

# 可执行文件位于 target/release/rcli
```

### 开发模式

```bash
# 运行项目（开发模式）
cargo run -- csv -i <input.csv> -f json

# 或直接使用构建的二进制文件
cargo build
./target/debug/rcli csv -i <input.csv> -f json
```

## 使用方法

### 基本语法

```bash
rcli csv [选项]
```

### 命令参数

- `-i, --input <FILE>`: 指定输入的 CSV 文件路径（必需）
- `-o, --output <FILE>`: 指定输出文件路径（可选，默认根据格式生成文件名）
- `-f, --format <FORMAT>`: 指定输出格式（json/yaml/toml，默认为 json）
- `-d, --delimiter <CHAR>`: 指定 CSV 分隔符（默认为逗号 `,`）
- `--header`: 指定 CSV 是否包含表头（默认为 true）

### 使用示例

```bash
# 将 CSV 转换为 JSON（默认格式）
rcli csv -i assets/test.csv

# 指定输出文件
rcli csv -i assets/test.csv -o output.json

# 转换为 YAML 格式
rcli csv -i assets/test.csv -f yaml -o output.yaml

# 转换为 TOML 格式
rcli csv -i assets/test.csv -f toml -o output.toml

# 使用自定义分隔符（例如制表符）
rcli csv -i data.tsv -d '\t' -f json

# 处理无表头的 CSV 文件
rcli csv -i data.csv --header false
```

## 项目结构

```
rcli/
├── Cargo.toml          # 项目配置和依赖
├── Cargo.lock          # 依赖版本锁定文件
├── deny.toml           # 依赖审查配置
├── README.md           # 项目说明文档
├── assets/             # 测试资源文件
│   └── test.csv
├── src/                # 源代码目录
│   ├── main.rs         # 程序入口
│   ├── lib.rs          # 库文件导出
│   ├── opts.rs         # 命令行参数定义
│   └── process.rs      # CSV 处理逻辑
└── target/             # 编译输出目录
```

## 开发

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
