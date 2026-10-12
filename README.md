<div align="center">

# MDHero 中文版

**MDHero** 的简体中文本地构建版 —— 一款轻快、好看的原生 Markdown 阅读与编辑工具

基于官方 [v0.2.10](https://github.com/vaibhav-kakde-in/mdhero) + 官方 i18n PR [#141](https://github.com/vaibhav-kakde-in/mdhero/pull/141) 构建

![Version](https://img.shields.io/badge/MDHero-v0.2.10-teal) ![UI](https://img.shields.io/badge/界面-简体中文-red) ![Platform](https://img.shields.io/badge/Windows-x64-blue) ![License](https://img.shields.io/badge/License-MIT-green)

</div>

![中文界面截图](assets/screenshot-zh.png)

---

> 🌐 English version: see [README.en.md](README.en.md)

## ✨ 这是什么

[MDHero](https://github.com/vaibhav-kakde-in/mdhero) 是一款基于 Tauri 的原生 Markdown 阅读器与轻量编辑器（约 8 MB、免费开源），但官方界面目前只有英文。
本仓库是我个人使用的**中文构建版**：官方最新代码 + 官方 i18n 分支的简体中文翻译，本地编译打包，开箱即用。

## 🔧 与官方版的差异

| 项目 | 官方版 v0.2.10 | 本仓库构建版 |
| --- | --- | --- |
| 界面语言 | 仅英文 | **简体中文 / English 可切换**（设置 → 常规 → 语言），原生菜单栏同步切换，含首次启动语言选择 |
| 代码基线 | `main` @ v0.2.10 | 官方 main + PR #141（i18n 分支，5 commits） |
| 代码签名 | 未签名（Windows） | 未签名（自构建） |
| 应用内更新 | 指向官方发布源 | **请勿使用**（见下方注意事项） |
| AI 功能 | 官方 AI Lookup（浏览器跳转） | 保留官方 AI Lookup；**未**集成任何第三方 AI 面板 |

除 i18n 外**没有任何功能改动**——渲染、编辑、分屏、Marp 演示、热重载、打印等全部与官方 v0.2.10 一致。

## 📥 安装

前往 [Releases](../../releases) 下载（二选一）：

- `MDHero_0.2.10_x64_en-US.msi` —— 推荐，标准安装
- `MDHero_0.2.10_x64-setup.exe` —— NSIS 安装器

> ⚠️ **首次运行提示**：安装包未经代码签名，Windows SmartScreen 可能弹出"未知发布者"警告 ——
> 点击 **更多信息 → 仍要运行** 即可。这是所有未签名开源程序的正常现象。

> ⚠️ **如已安装官方英文版**：建议先在「设置 → 应用 → 已安装的应用」中卸载官方 MDHero，再安装本中文版，避免更新检测互相干扰。

> ⚠️ **请勿点击应用内"检查更新 / 立即更新"**：构建时沿用了官方更新源，应用内更新会把中文版覆盖回英文官方版。需要新版本时，回到本仓库取新的 Release，或按下方说明自行构建。

安装后首次启动会弹出语言选择框，选择 **简体中文** 即可；之后可在 设置 → 常规 → 语言 中随时切换（支持"跟随系统"）。

## 🛠️ 从源码构建

环境要求（Windows）：

- [Node.js](https://nodejs.org/) ≥ 20（实测 v25）+ [pnpm](https://pnpm.io/) ≥ 9
- [Rust](https://rustup.rs/) stable-msvc 工具链
- Visual Studio Build Tools（C++ 桌面开发工作负载）+ Windows 11 SDK

```bash
# 1. 安装前端依赖
pnpm install

# 2. （可选）验证
pnpm check        # 类型检查，0 错误
pnpm test         # 单元测试，225 用例

# 3. 构建安装包（产物在 src-tauri/target/release/bundle/）
pnpm tauri build --bundles msi,nsis
```

> 💡 在 Windows 上克隆本仓库后，建议先执行 `git config core.autocrlf false` 再重新检出，
> 否则 CRLF 换行符会导致 `version-sync`、`quicklook-bundle` 两个测试失败（与汉化无关，是仓库脚本按 LF 编写）。

## 📁 仓库结构

本仓库保留上游完整目录结构，改动集中在：

```
src/lib/i18n/            # 国际化模块（来自官方 PR #141）
  locales/zh-CN.json     #   简体中文语言包（262 键；缺的 2 键为英文复数形式，中文不适用）
  locales/en.json        #   英文（回退）
  locales/{de,es,fr}.json#   其他官方翻译
src-tauri/src/menu.rs    # 原生菜单跟随界面语言切换
assets/screenshot-zh.png # 本 README 截图
README.md                # 本文件（中文）
README.en.md             # 英文版
README.upstream.md       # 官方原版 README（未改动）
```

## ⚖️ 许可与免责声明

- 上游 MDHero 与本构建版均基于 **MIT License**（见 [LICENSE](LICENSE)），翻译文本随仓库一并开源。
- 本项目为**个人非官方构建**，与 MDHero 官方无任何关联；不代表官方发布，也不获得官方支持。
- 安装包未做代码签名，仅建议**自行构建后使用**（这也是把源码完整放在这里的原因）。
- 官方原文请见 [README.upstream.md](README.upstream.md) 与 [mdhero.app](https://mdhero.app)。

## 🙏 致谢

- [vaibhav-kakde-in/mdhero](https://github.com/vaibhav-kakde-in/mdhero) —— 优秀的原作者与维护者
- [@hdo-projects](https://github.com/hdo-projects) 等 —— 官方 i18n PR #141 作者（zh-CN 翻译基于其工作）
- 官方中文版合并进主线后，本仓库将退役或转为纯构建指南
