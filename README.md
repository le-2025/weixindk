# 械式微信多开器

一款免费的 Windows 桌面工具，通过独立数据目录隔离技术实现微信多开，让您可以在同一台电脑上同时登录多个微信账号。

## 特性

- 🚀 一键启动多个独立微信实例
- 📁 独立数据目录隔离，互不干扰
- 🏷️ 自定义实例标签，方便管理
- 🔄 实时状态监控与同步
- 📌 关闭时最小化到系统托盘
- 🎨 现代化 UI，科技感交互体验

## 安全说明

- **不修改**微信程序本体
- **不注入**任何代码到微信进程
- **不收集**任何用户隐私数据
- **不联网**传输任何信息
- 仅通过 Windows 系统级数据目录隔离实现多开功能

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React 18 + TypeScript + Ant Design 5 + Tailwind CSS 3 |
| 后端 | Rust + Tauri v2 |
| 数据库 | SQLite (rusqlite) |
| 构建 | Vite 5 |

## 开发环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- [pnpm](https://pnpm.io/) >= 8

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/le-2025/weixindk.git
cd weixindk

# 安装前端依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 构建发布版本
pnpm tauri build
```

## 项目结构

```
weixindk/
├── src/                        # 前端源码
│   ├── components/
│   │   ├── About/              # 关于页面
│   │   ├── Instance/           # 实例管理组件
│   │   ├── Layout/             # 布局组件
│   │   └── Settings/           # 设置面板
│   ├── hooks/                  # 自定义 Hooks
│   ├── services/               # Tauri IPC 封装
│   ├── styles/                 # 全局样式
│   └── types/                  # TypeScript 类型定义
├── src-tauri/                  # 后端源码 (Rust)
│   └── src/
│       ├── commands/           # Tauri 命令
│       ├── mutex/              # Mutex 引擎
│       ├── process/            # 进程管理
│       └── storage/            # 数据存储
├── LICENSE                     # MIT 许可证
└── package.json
```

## 免责声明

1. 本工具仅供学习和研究使用，严禁用于任何违法违规用途。
2. 本工具不修改微信程序本体，不破解任何功能限制，仅利用系统级数据目录隔离实现多开。
3. 使用本工具产生的任何直接或间接损失，作者不承担任何责任。
4. 微信是腾讯公司的注册商标，本工具与腾讯公司无任何关联。

> 更多信息请查看应用内的「关于」页面。

## 许可证

[MIT License](LICENSE)
