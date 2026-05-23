# Ale, My Eyes! 前端界面

## 设计语言

本项目使用与 **Rakuraku Music Station NG** 相同的设计语言：

- **Material 2 设计系统**
- **Vuetify 组件库**
- **CSS 变量主题系统**
- **Inter 字体**
- **支持深色/浅色主题**

## 技术栈

- **Vue 3** - 前端框架
- **Vuetify 3** - Material Design 组件库
- **Vite** - 构建工具
- **TypeScript** - 类型系统
- **Tailwind CSS v4** - 工具类 CSS

## 开发

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build
```

## 项目结构

```
frontend/
├── public/
│   ├── icon.svg          # 应用图标
│   └── manifest.json     # PWA 配置
├── src/
│   ├── api/              # API 调用
│   ├── assets/           # 静态资源
│   ├── components/       # 共享组件
│   ├── views/            # 页面组件
│   ├── App.vue           # 根组件
│   ├── main.ts           # 入口文件
│   ├── router.ts         # 路由配置
│   ├── style.css         # 全局样式
│   └── env.d.ts          # 类型声明
├── index.html            # HTML 模板
├── package.json          # 依赖配置
├── tsconfig.json         # TypeScript 配置
└── vite.config.ts        # Vite 配置
```

## 主题系统

使用 CSS 变量定义主题，支持：

- 浅色主题
- 深色主题
- 自动跟随系统
- 高对比度模式

## 无障碍特性

- 高对比度文本
- 键盘导航支持
- 屏幕阅读器友好
- 可调整字体大小

## 组件说明

### StatusCard
状态显示卡片，显示系统当前状态。

### VoiceButton
语音录制按钮，支持动画效果。

### HomeView
主页面，包含语音交互和图像描述功能。

### SettingsView
设置页面，配置 API、推理模式、界面等。

### ModelView
模型管理页面，下载和管理本地模型。