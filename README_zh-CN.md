简体中文 | [English](./README.md)

# Dialog flow chat bot
这是 **只有一个执行文件** 的AI工具, 它包含了一个可视化的流程编辑器和应答逻辑.  
<img src="https://img.shields.io/badge/Latest_version-v1.17.5-blue" />

![Homepage](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/screenshots/screenshot1.png)

# ✨ 关键特性
* 🛒 **轻量级** 只有一个执行文件, 可以在没有GPU的笔记本上平滑的执行 (数据文件会在运行期动态的生成).
* 🐱‍🏍 **AI 驱动** 集成了 `Huggingface 本地模型`, `Ollama` 和 `OpenAI`, 可以用于 `流程聊天`, `答案节点文本生成` 以及 `意图识别` 等.
* 🚀 **快速** 使用`Rust`和`Vue`构建.
* 😀 **简单** 通过使用可视化的流程编辑器，只需要用鼠标拖拽几个不同类型的节点, 即可创建一个简单的对话机器人.
* 🔐 **安全** 100% 开源, 所有运行时的数据, 都保存在本地 (使用 `OpenAI API` 可能会暴露一些数据).

# 现在就尝试一下!
* 🐋 **Docker** 我们提供了一个`Docker`镜像: [dialogflowchatbot/demo](https://hub.docker.com/repository/docker/dialogflowchatbot/demo)
* 💻 **可直接执行的发布版本**, 请通过发布页: [here](https://github.com/dialogflowchatbot/dialogflow/releases) , 根据不同的平台下载

> 默认情况下, 应用会监听: `127.0.0.1:12715`, 你可以使用 `-ip` 参数和 `-port` 参数, 来指定新的监听地址和端口, 例如: `dialogflow -ip 0.0.0.0 -port 8888`

<!-- # Releases and source code
* 💾 If you're looking for **binary releases**, please check [here](https://github.com/dialogflowchatbot/dialogflow/releases)
* 🎈 The **back end** of this application is [here](https://github.com/dialogflowchatbot/dialogflow-backend)
* 🎨 The **front end** of this application is [here](https://github.com/dialogflowchatbot/dialogflow-frontend) -->

# 查看详细介绍, 了解更多信息
[https://dialogflowchatbot.github.io/](https://dialogflowchatbot.github.io/#/)

# 功能节点列表
|节点|名称|
|----|----|
|![DialogNode](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/dialogNode.png)|对话答案节点|
|![LLM chat node](https://dialogflowchatbot.github.io/assets/llmChatNode-IFUpFC-1.png)|大模型聊天节点|
|![](https://dialogflowchatbot.github.io/assets/knowledgeBaseAnswerNode-nPaXLuCc.png)|知识库答案节点|
|![](https://dialogflowchatbot.github.io/assets/conditionNode-DyKXzgYH.png)|条件节点|
|![](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/gotoNode.png)|跳转节点|
|![](https://dialogflowchatbot.github.io/assets/collectNode-8FKuiM1E.png)|信息收集节点|
|![](https://dialogflowchatbot.github.io/assets/externalApiNode-Cq5407hi.png)|请求外部接口节点|
|![](https://dialogflowchatbot.github.io/assets/sendEmailNode-CSpJZw-P.png)|邮件发送节点|
|![](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/endNode.png)|流程结束节点|

通过使用上面不同的节点来排列和组合, 就可以创建解决不同场景问题的机器人.

# 截图
![Robot detail](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/screenshots/screenshot2.png)

### 体验演示机器人
![Demo](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/ae15f7fabebe154ebc8dec8511cb1ec063163358/src/assets/demo1.gif)

### 创建一个条件节点的分支
![Setup a condition branch](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/18f8b2821921f1732e7699f515615a3d7838f16a/screenshots/condition1.gif)

### 对话答案节点的自动文本生成

![Text generation](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/main/src/assets/screenshots/textGeneration.gif?raw=true "Text generation")

### 测试一个对话机器人
![Flow testing tool](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/main/src/assets/screenshots/testing.png?raw=true "Flow testing tool")


## 现在就开始使用

### Docker 镜像
1. docker pull dialogflowchatbot/demo
2. docker run -dp 127.0.0.1:12715:12715 --name dialogflowdemo dialogflowchatbot/demo
3. 打开浏览器并访问: http://127.0.0.1:12715/ 打开应用界面

### 发布版本
1. 从 [Github release page](https://github.com/dialogflowchatbot/dialogflow/releases), 选择不同系统并下载.
1. 直接执行, 或者使用 `-ip` 和 `-port` 修改监听地址, 如: `dialogflow -ip 0.0.0.0 -port 8888`.
1. 打开浏览器并访问 http://localhost:12715 (默认) 或 http://`新的IP`:`新的端口` 打开应用界面
1. 进入一个机器人
2. 创建一个对话流程，并点击名称进入编辑器
1. 构建属于自己的机器人
1. 测试
