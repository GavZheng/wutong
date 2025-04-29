<h1 style="text-align: center;">梧桐 - 开发者的瑞士军刀</h1>

[![Author: Gavin Zheng](https://img.shields.io/badge/Author-Gavin_Zheng-f2f28d)](https://github.com/GavZheng)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange)
![Version: 0.2.0](https://img.shields.io/badge/Version-0.2.0-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
![Github Stars](https://img.shields.io/github/stars/WutongDev/wutong?style=flat&color=red)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT_zh.md)

[English](../../README.md) | [简体中文](README_zh.md)

梧桐是一个由**Rust**编写的跨平台工具箱，用于帮助开发者快速完成各种任务。

---

## 目录
- [目录](#目录)
- [为什么我们需要梧桐](#为什么我们需要梧桐)
- [如何安装梧桐](#如何安装梧桐)
- [如何使用梧桐](#如何使用梧桐)
- [如何参与梧桐的开发](#如何参与梧桐的开发)
- [特别致谢](#特别致谢)
- [贡献者](#贡献者)
- [许可证](#许可证)

---

## 为什么我们需要梧桐
在日常的项目开发中，我们经常会遇到需要对某些数据进行编码或转换的情况，比如将字符串转换为Base64编码，或者将数字转换为二进制表示。通常，为了完成这些任务，我们不得不离开我们的开发环境，去寻找专门的程序或网站，进行多次交互操作，最后复制结果再回到开发环境中。这个过程不仅耗时，更重要的是，频繁的上下文切换可能会打断我们的工作流，是我们失去专注力和宝贵的灵感。

**梧桐**正是为了解决这个问题而诞生的。它是一个命令行工具，允许我们在终端中直接完成各种在编程中可能遇到的琐屑任务，从而大大提高了我们的编码效率。

---

## 如何安装梧桐
如果您是 macOS 用户，则可以使用：
```bash
brew tap GavZheng/wutong
brew install wutong
```
PS：仅支持 macOS 13 或更高版本。  
如果您是 Windows 或 Linux 用户，请等待后续版本。   
*当然，您可以自己编译梧桐。*
---

## 如何使用梧桐
你可以通过在命令行键入`wutong --help`来获取详细信息。  
梧桐的大致功能如下表（v0.2.0）

| 功能       | 描述                         |
|----------|----------------------------|
| base     | 将输入的字符串进行base32、base64编码编码 |
| bc       | 将输入的数字转换为二、八、十、十六进制        |
| color    | RGB和Hex颜色的相互转换             |
| md5      | 将输入的字符串进行md5哈希             |
| charcode | 字符串编码转换                    |
| flow     | 根据 Gitflow 工作流程管理 git 存储库  |

---

## 如何参与梧桐的开发
梧桐是一个开源项目，我们热忱欢迎并高度期待来自全球各地的开发者能够加入并参与到梧桐的开发进程中来。
你可以通过以下方式参与梧桐的开发：
1. 提交漏洞报告和功能建议：如果你在使用梧桐的过程中发现了漏洞或者有任何功能建议，请参阅[梧桐安全指南](SECURITY_zh.md)
2. 贡献代码：如果你有能力并且愿意为梧桐贡献代码，请参阅[梧桐贡献者指南](CONTRIBUTING_zh.md)文件。

---

## 特别致谢
对以下为Wutong做出突出贡献的人员表示真挚地感谢（以首字母为序）：
- [Bob](https://github.com/ChepleBob26)：为梧桐的开发做出了非常多非代码的贡献，是梧桐的第一位用户。
- Silent：为梧桐起了一个很好的名字。

---

## 贡献者
<a href="https://github.com/WutongDev/wutong/contributors">
  <img src="https://contrib.rocks/image?repo=WutongDev/wutong" alt="Contributors"/>
</a>

---

## FAQ
Q1: 为什么叫梧桐？  
A：这个名字来自我（Gavin）的一个朋友（Silent），梧桐是她最喜欢的树。“Wutong”是梧桐的拼音。

Q2: 为何选择Rust作为开发语言？  
A: Rust凭借其卓越的跨平台能力、内存安全特性和高效执行性能，在综合评估Python/C++/C等候选语言后，被确认为满足项目需求的最佳技术选型。

Q3: 当前为何仅支持macOS平台？
A: 现阶段开发重心聚焦于核心功能完善与快速迭代。Windows/Linux版本已列入roadmap，将在主体功能稳定后启动多平台适配工作。

Q4：更多关于Gavin？  
A：你可以访问[Gavin的Github主页](https://github.com/GavZheng)。

---

## 许可证
[MIT](../../LICENSE) © 2025 Gavin Zheng
