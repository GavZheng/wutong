<h1 style="text-align: center;">梧桐 - 开发者的瑞士军刀</h1>

[English](../../README.md) | [简体中文](README_zh.md)

梧桐是一个由**Rust**编写的跨平台工具箱，用于帮助开发者快速完成各种任务。

![LICENSE](https://img.shields.io/badge/License-MIT-blue)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Version](https://img.shields.io/badge/Version-v0.1.0%20alpha-green)
![Github Stars](https://img.shields.io/github/stars/WutongDev/wutong?style=flat&color=red)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT_zh.md)

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
> [!NOTE]  
> 梧桐在测试版不提供安装方式，如希望体验请自行编译。在正式版发布后，梧桐将会提供安装方式。

---

## 如何使用梧桐
你可以通过在命令行键入`wutong --help`来获取详细信息。  
梧桐的大致功能如下表（v0.1.0-alpha）

| 功能    | 描述                                          |
|-------|---------------------------------------------|
| base  | 将输入的字符串进行base32、base64编码编码                  |
| bc    | 将输入的数字转换为二、八、十、十六进制或存储空间大小单位（KB, MB, GB等）表示 |
| color | RGB和Hex颜色的相互转换                              |
| md5   | 将输入的字符串进行md5哈希                              |
| diff  | 比较文本差异                                      |
| char  | 字符串编码转换                                     |

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

## 许可证
[MIT](../../LICENSE) © 2025 Gavin Zheng
