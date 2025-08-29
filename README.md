# 🦀 Rust WebAssembly 项目

这是一个使用 Rust 和 WebAssembly 构建的示例项目，展示了如何将 Rust 代码编译为 WebAssembly 并在浏览器中运行。

## 🚀 快速开始

### 1. 安装依赖

首先确保您已经安装了 Rust：

```bash
# 安装 Rust (如果还没有安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. 构建项目

使用提供的构建脚本：

```bash
chmod +x build.sh
./build.sh
```

或者手动构建：

```bash
# 安装 wasm-pack (如果还没有安装)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 构建 WebAssembly 包
wasm-pack build --target web --out-dir pkg
```

### 3. 运行项目

由于浏览器的安全限制，您需要通过 HTTP 服务器来访问项目：

```bash
# 使用 Python 启动本地服务器
python3 -m http.server 8000

# 或者使用 Node.js
npx serve .
```

然后在浏览器中访问 `http://localhost:8000`

## 📁 项目结构

```
world-wasm/
├── Cargo.toml          # Rust 项目配置
├── src/
│   └── lib.rs         # Rust 源代码
├── pkg/               # 编译生成的 WebAssembly 文件 (构建后生成)
├── index.html         # 测试页面
├── build.sh          # 构建脚本
└── README.md         # 项目说明
```

## 🔧 项目功能

本项目包含以下示例功能：

1. **问候功能** - 在控制台输出问候信息
2. **数学计算** - 两个数字相加
3. **斐波那契数列** - 计算斐波那契数列

## 📚 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [WebAssembly 官方网站](https://webassembly.org/)
- [wasm-pack 文档](https://rustwasm.github.io/wasm-pack/)
- [wasm-bindgen 文档](https://rustwasm.github.io/wasm-bindgen/)

## 🛠️ 开发工作流

1. 修改 `src/lib.rs` 中的 Rust 代码
2. 运行 `./build.sh` 重新构建
3. 刷新浏览器查看更改

## 📝 注意事项

- WebAssembly 文件必须通过 HTTP(S) 协议访问，不能直接打开 HTML 文件
- 确保在 `Cargo.toml` 中正确配置了 `crate-type = ["cdylib"]`
- 使用 `#[wasm_bindgen]` 属性来导出 Rust 函数到 JavaScript

## 🎯 下一步

尝试添加更多功能：

- 更复杂的数据结构操作
- 图像处理功能
- 游戏逻辑
- 数据可视化

祝您学习愉快！🎉
