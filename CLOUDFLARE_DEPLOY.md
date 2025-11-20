# Cloudflare Pages 部署指南

## 概述

本项目现在支持同时部署到 GitHub Pages 和 Cloudflare Pages。你可以选择以下两种方式之一：

1. **统一部署**：修改现有的 GitHub Actions workflow，一次构建同时部署到两个平台
2. **独立部署**：使用独立的 Cloudflare Pages workflow

## 配置步骤

### 1. 在 Cloudflare 创建 API Token

1. 登录 [Cloudflare Dashboard](https://dash.cloudflare.com)
2. 进入 "My Profile" → "API Tokens"
3. 点击 "Create Token"
4. 选择 "Custom token"
5. 配置权限：
   - Account: Cloudflare Pages - Edit
   - Zone: DNS - Edit (如果需要自定义域名)
6. 生成 Token 并保存

### 2. 获取 Account ID

1. 在 Cloudflare Dashboard 中
2. 进入你的域名
3. 右侧 Overview 页面底部可以找到 Account ID

### 3. 在 GitHub 仓库设置 Secrets

1. 进入 GitHub 仓库 → Settings → Secrets and variables → Actions
2. 添加以下 Secrets：

```
CLOUDFLARE_API_TOKEN: 你的 Cloudflare API Token
CLOUDFLARE_ACCOUNT_ID: 你的 Cloudflare Account ID
```

### 4. 在 Cloudflare 创建 Pages 项目

1. 登录 Cloudflare Dashboard
2. 进入 "Workers & Pages"
3. 点击 "Create application" → "Pages" → "Connect to Git"
4. 选择你的 GitHub 仓库
5. 配置构建设置：
   - Build command: `./trunk build --release --public-url "/"`
   - Build output directory: `dist`
6. 点击 "Save and Deploy"

或者，如果你使用 GitHub Actions 部署（推荐）：

1. 在 Cloudflare Pages 创建项目时选择 "Direct Upload"
2. 记下 Project Name（例如：treesheets-gitpage）
3. 确保 workflow 文件中的 projectName 匹配

## 部署方式选择

### 方式一：统一部署（推荐）

使用修改后的 `.github/workflows/gh-pages-deploy.yml`，一次构建同时部署到两个平台。

优点：
- 构建一次，节省时间
- 保持构建环境一致
- 简化维护

### 方式二：独立部署

使用 `.github/workflows/cloudflare-deploy.yml`，独立于 GitHub Pages 部署。

优点：
- 可以独立控制 Cloudflare 部署
- 可以有不同的触发条件
- 适合需要不同配置的情况

## 自定义域名（可选）

如果你想使用自定义域名：

1. 在 Cloudflare Pages 项目设置中添加自定义域名
2. 按照提示配置 DNS 记录
3. 等待 DNS 传播（通常几分钟到几小时）

## 故障排除

### 部署失败

1. 检查 Secrets 是否正确配置
2. 检查 Cloudflare API Token 权限
3. 检查 Account ID 是否正确
4. 查看 GitHub Actions 日志获取详细错误信息

### 页面加载问题

1. 检查 `public-url` 参数是否正确
   - GitHub Pages: `--public-url "/treesheets-gitpage"`
   - Cloudflare Pages: `--public-url "/"`
2. 检查浏览器控制台是否有资源加载错误

### 环境变量

如果需要添加环境变量，可以在 workflow 文件中添加：

```yaml
- name: Build with Trunk
  env:
    SOME_ENV_VAR: ${{ secrets.SOME_ENV_VAR }}
  run: ./trunk build --release --public-url "/"
```

## 相关资源

- [Cloudflare Pages 文档](https://developers.cloudflare.com/pages/)
- [GitHub Actions Cloudflare Pages 集成](https://github.com/cloudflare/pages-action)
- [Trunk 构建工具](https://trunkrs.dev/)