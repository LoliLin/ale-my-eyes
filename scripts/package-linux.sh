#!/bin/bash
# Ale, My Eyes! Linux 打包脚本
# 用法: ./scripts/package-linux.sh

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}开始打包 Ale, My Eyes! Linux 版本...${NC}"

# 检查是否在项目根目录
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}错误: 请在项目根目录运行此脚本${NC}"
    exit 1
fi

# 检查 Rust 工具链
if ! command -v rustup &> /dev/null; then
    echo -e "${RED}错误: 未找到 rustup，请先安装 Rust${NC}"
    exit 1
fi

# 构建 Linux 版本
echo -e "${YELLOW}构建 Linux 版本...${NC}"
cargo build --release

# 创建打包目录
PACKAGE_DIR="ale-my-eyes-linux"
echo -e "${YELLOW}创建打包目录: ${PACKAGE_DIR}${NC}"
rm -rf "${PACKAGE_DIR}"
mkdir -p "${PACKAGE_DIR}"

# 复制可执行文件
echo -e "${YELLOW}复制可执行文件...${NC}"
cp target/release/ale-server "${PACKAGE_DIR}/"
cp target/release/ale-cli "${PACKAGE_DIR}/"
cp target/release/ale-gui "${PACKAGE_DIR}/"

# 创建模型目录
echo -e "${YELLOW}创建模型目录...${NC}"
mkdir -p "${PACKAGE_DIR}/models"
mkdir -p "${PACKAGE_DIR}/models/bundled"
mkdir -p "${PACKAGE_DIR}/models/downloaded"

# 创建配置目录
echo -e "${YELLOW}创建配置目录...${NC}"
mkdir -p "${PACKAGE_DIR}/config"

# 创建默认配置文件
echo -e "${YELLOW}创建默认配置文件...${NC}"
cat > "${PACKAGE_DIR}/config/config.json" << EOF
{
  "cloud_api": {
    "provider": "openai",
    "api_key": "",
    "api_url": "https://api.openai.com/v1",
    "model": "gpt-4o",
    "max_tokens": 1024,
    "timeout": 30
  },
  "models": {
    "auto_download": true,
    "max_download_size": 524288000,
    "preferred_quality": "balanced",
    "offline_mode": false,
    "models_dir": "models"
  },
  "inference": {
    "mode": "adaptive",
    "prefer_cloud": true,
    "timeout": 30,
    "fallback_to_local": true
  },
  "audio": {
    "sample_rate": 16000,
    "channels": 1,
    "buffer_size": 4096,
    "voice": "default",
    "speed": 1.0
  },
  "ui": {
    "language": "zh-CN",
    "theme": "system",
    "font_size": 16,
    "high_contrast": false,
    "screen_reader": true
  }
}
EOF

# 创建启动脚本
echo -e "${YELLOW}创建启动脚本...${NC}"
cat > "${PACKAGE_DIR}/start-server.sh" << EOF
#!/bin/bash
echo "启动 Ale, My Eyes! 服务器..."
./ale-server
EOF

cat > "${PACKAGE_DIR}/start-gui.sh" << EOF
#!/bin/bash
echo "启动 Ale, My Eyes! 图形界面..."
./ale-gui
EOF

# 设置执行权限
chmod +x "${PACKAGE_DIR}/start-server.sh"
chmod +x "${PACKAGE_DIR}/start-gui.sh"
chmod +x "${PACKAGE_DIR}/ale-server"
chmod +x "${PACKAGE_DIR}/ale-cli"
chmod +x "${PACKAGE_DIR}/ale-gui"

# 创建 desktop 文件（用于 Linux 桌面集成）
echo -e "${YELLOW}创建 desktop 文件...${NC}"
mkdir -p "${PACKAGE_DIR}/share/applications"
cat > "${PACKAGE_DIR}/share/applications/ale-my-eyes.desktop" << EOF
[Desktop Entry]
Name=Ale, My Eyes!
Comment=智能辅助系统
Exec=$(pwd)/${PACKAGE_DIR}/ale-gui
Icon=$(pwd)/${PACKAGE_DIR}/icon.png
Terminal=false
Type=Application
Categories=Utility;Accessibility;
Keywords=accessibility;assistant;vision;speech;
EOF

# 创建 README
echo -e "${YELLOW}创建 README...${NC}"
cat > "${PACKAGE_DIR}/README.md" << EOF
# Ale, My Eyes! - 智能辅助系统

这是一个为视障人士设计的智能辅助系统，使用 VLM/ASR/TTS 技术帮助用户更好地使用电脑。

## 快速开始

### 1. 配置 API 密钥
编辑 config/config.json 文件，设置您的 OpenAI API 密钥：
\`\`\`json
"api_key": "sk-your-api-key-here"
\`\`\`

### 2. 启动服务器
\`\`\`bash
./start-server.sh
\`\`\`

### 3. 启动图形界面
\`\`\`bash
./start-gui.sh
\`\`\`

## 功能特性

- 语音识别：通过麦克风输入语音指令
- 语音合成：系统状态和屏幕内容的语音反馈
- 图像描述：上传图像获取描述
- 自然语言交互：支持自然语言指令

## 系统要求

- Linux (Ubuntu 20.04+, Fedora 34+, Debian 11+)
- 至少 4GB 内存
- 麦克风和扬声器
- 网络连接（用于云端 API）

## 配置说明

配置文件位于 config/config.json，包含以下设置：

- cloud_api: 云端 API 配置
- models: 模型下载和管理设置
- inference: 推理模式设置
- audio: 音频设置
- ui: 界面设置

## 安装依赖

某些功能可能需要系统依赖：

\`\`\`bash
# Ubuntu/Debian
sudo apt-get install libspeechd-dev libasound2-dev

# Fedora
sudo dnf install speech-dispatcher-devel alsa-lib-devel

# Arch Linux
sudo pacman -S speech-dispatcher alsa-lib
\`\`\`

## 获取帮助

如需帮助，请访问项目主页或提交 Issue。
EOF

# 创建 icon.png（占位符）
echo -e "${YELLOW}创建图标文件...${NC}"
# 这里应该有一个实际的图标文件，现在创建一个占位符
echo "图标占位符" > "${PACKAGE_DIR}/icon.png"

# 创建压缩包
echo -e "${YELLOW}创建压缩包...${NC}"
if command -v tar &> /dev/null; then
    tar -czf "ale-my-eyes-linux.tar.gz" "${PACKAGE_DIR}"
elif command -v zip &> /dev/null; then
    zip -r "ale-my-eyes-linux.zip" "${PACKAGE_DIR}"
else
    echo -e "${YELLOW}警告: 未找到 tar 或 zip，跳过压缩包创建${NC}"
fi

echo -e "${GREEN}打包完成！${NC}"
echo -e "${GREEN}输出目录: ${PACKAGE_DIR}${NC}"
if [ -f "ale-my-eyes-linux.tar.gz" ]; then
    echo -e "${GREEN}压缩包: ale-my-eyes-linux.tar.gz${NC}"
fi

echo -e "${YELLOW}下一步：${NC}"
echo -e "1. 将 ${PACKAGE_DIR} 目录复制到目标 Linux 机器"
echo -e "2. 编辑 config/config.json 设置 API 密钥"
echo -e "3. 运行 ./start-server.sh 启动服务器"
echo -e "4. 运行 ./start-gui.sh 启动图形界面"