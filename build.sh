#!/bin/bash

# Rust WebAssembly 构建脚本

echo "🦀 开始构建 Rust WebAssembly 项目..."

# 检查是否安装了 wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack 未安装"
    echo "📦 正在安装 wasm-pack..."
    cargo install wasm-pack

    if [ $? -eq 0 ]; then
        echo "✅ wasm-pack 安装成功!"
    else
        echo "❌ wasm-pack 安装失败"
        exit 1
    fi
fi

# 构建 WebAssembly 包
echo "🔨 正在编译 WebAssembly..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ 构建成功!"
    echo "🌐 现在可以通过 HTTP 服务器访问 index.html 来测试您的 WebAssembly 应用"
    echo ""
    echo "💡 提示: 您可以运行以下命令启动本地服务器:"
    echo "   python3 -m http.server 8000"
    echo "   或者使用 Node.js: npx serve ."
    echo ""
    echo "然后在浏览器中访问: http://localhost:8000"
else
    echo "❌ 构建失败"
    exit 1
fi