[简体中文](./README_zh-CN.md) | English

# Dialog flow chat bot
This is a software with **only one executable file**, including a visual process editor and a response system.  
<img src="https://img.shields.io/badge/Latest_version-v1.17.5-blue" />

![Dialog flow editor](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/7f412d0746cfbddd43a7a75abb246add63d12200/src/assets/screenshots/flow-editor.png)

# ✨ Features
* 🛒 **Light** Only ONE executable file, it can run smoothly on laptops without GPUs (data files will be created at runtime automatically).
* 🐱‍🏍 **AI powered** Integrated `Huggingface local model`, `Ollama` and `OpenAI`, this can be used for `Chat`, `Text generation` and `Intent detection`.
* 🚀 **Fast** Built on Rust and Vue3.
* 😀 **Simple** Use the mouse to drag and drop with our intuitive node-based editor.
* 🔐 **Safe** 100% open source, all runtime data is saved locally (Using `OpenAI API` may expose some data).

# Give it a try!
* 🐋 **Docker** We provided an image on Docker Hub at [dialogflowchatbot/demo](https://hub.docker.com/repository/docker/dialogflowchatbot/demo)
* 💻 **Binary releases**, please check [here](https://github.com/dialogflowchatbot/dialogflow/releases)

> By default application will listen to `127.0.0.1:12715`, you can use `-ip` and `-port` specify new value, e.g.: `dialogflow -ip 0.0.0.0 -port 8888`

<!-- # Releases and source code
* 💾 If you're looking for **binary releases**, please check [here](https://github.com/dialogflowchatbot/dialogflow/releases)
* 🎈 The **back end** of this application is [here](https://github.com/dialogflowchatbot/dialogflow-backend)
* 🎨 The **front end** of this application is [here](https://github.com/dialogflowchatbot/dialogflow-frontend) -->

# Check out introduction page
[https://dialogflowchatbot.github.io/](https://dialogflowchatbot.github.io/#/)

# Function nodes
|Node|Name|
|----|----|
|![DialogNode](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/dialogNode.png)|Dialog Node|
|![LLM chat node](https://dialogflowchatbot.github.io/assets/llmChatNode-IFUpFC-1.png)|Large language model chat node|
|![](https://dialogflowchatbot.github.io/assets/knowledgeBaseAnswerNode-nPaXLuCc.png)|Knowledge base answer node|
|![](https://dialogflowchatbot.github.io/assets/conditionNode-DyKXzgYH.png)|Conditions node|
|![](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/gotoNode.png)|Goto node|
|![](https://dialogflowchatbot.github.io/assets/collectNode-8FKuiM1E.png)|Collect node|
|![](https://dialogflowchatbot.github.io/assets/externalApiNode-Cq5407hi.png)|External HTTP node|
|![](https://dialogflowchatbot.github.io/assets/sendEmailNode-CSpJZw-P.png)|Send email node|
|![](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/endNode.png)|The end node|

Using the different nodes above, to arrange and combine, you can get a conversational bot that can handle problems in different scenarios.

# Screenshots
![Homepage](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/screenshots/screenshot1.png)

![Robot detail](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/c30533aead90dfe563f1dbe89e4623c215bd0f2d/src/assets/screenshots/screenshot2.png)

### Try a demo dialog flow
![Demo](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/ae15f7fabebe154ebc8dec8511cb1ec063163358/src/assets/demo1.gif)

### Setup a condition branch
![Setup a condition branch](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/18f8b2821921f1732e7699f515615a3d7838f16a/screenshots/condition1.gif)

### Text generation

![Text generation](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/main/src/assets/screenshots/textGeneration.gif?raw=true "Text generation")

### Testing a dialog flow
![Flow testing tool](https://github.com/dialogflowchatbot/dialogflow-showcase/blob/main/src/assets/screenshots/testing.png?raw=true "Flow testing tool")


## Get started

### Docker image
1. docker pull dialogflowchatbot/demo
2. docker run -dp 127.0.0.1:12715:12715 --name dialogflowdemo dialogflowchatbot/demo
3. Open your browser and visit: http://127.0.0.1:12715/

### Binary release
1. From [Github release page](https://github.com/dialogflowchatbot/dialogflow/releases), depending on the operating system, download the application.
1. Run it directly, or use the `-ip` and `-port` parameters to perform the listening IP address and port, e.g.: `dialogflow -ip 0.0.0.0 -port 8888`.
1. Open your browser and visit http://localhost:12715 (by default) or http://`new IP`:`new port` to see the application in action
1. Add a main flow and click its name into it
1. Create dialog flow by dragging and drop nodes onto canvas
1. Test it
