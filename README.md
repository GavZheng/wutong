<h1 style="text-align: center;">Wutong - The Swiss Army Knife for Developers</h1>

[![Author: Gavin Zheng](https://img.shields.io/badge/Author-Gavin_Zheng-f2f28d)](https://github.com/GavZheng)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange)
![Version: 0.2.0](https://img.shields.io/badge/Version-0.2.0-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
![Github Stars](https://img.shields.io/github/stars/WutongDev/wutong?style=flat&color=red)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

[English](./README.md) | [简体中文](./docs/zh/README_zh.md)

---

## Contents
- [Contents](#Contents)
- [Why We Need Wutong](#why-we-need-wutong)
- [How to Install Wutong](#how-to-install-wutong)
- [How to Use Wutong](#how-to-use-wutong)
- [How to Contribute to Wutong](#how-to-contribute-to-wutong)
- [Special Thanks](#special-thanks)
- [Contributors](#contributors)
- [License](#license)

---

## Why We Need Wutong
In our daily project development, we frequently encounter situations where we need to encode or convert certain data. For instance, converting a string to Base64 encoding or representing a number in binary format. Often, to accomplish these tasks, we have to leave our development environment, search for specialized programs or websites, perform multiple interactive operations, copy the results, and then return to our development environment. This process is not only time-consuming but also disrupts our workflow by causing frequent context-switching, which can lead to a loss of focus and valuable inspiration.

**Wutong** was created to address this issue. It is a command-line tool that allows us to complete various mundane tasks that we may encounter in programming directly within the terminal, thereby significantly enhancing our coding efficiency.

---

## How to Install Wutong
If you are a macOS user, you can use:
```bash
brew tap GavZheng/wutong
brew install wutong
```
PS: Only macOS 13 or later is supported.  
If you are a Windows or Linux user, please wait for the subsequent version.  
*Of course, you can compile Wutong yourself.*

---

## How to Use Wutong
You can obtain detailed information by typing `wutong --help` in the command line.  
The general functions of Wutong are listed below (v0.2.1):

| Function | Description                                                  |
|----------|--------------------------------------------------------------|
| base     | Encode input strings in base32 or base64                     |
| bc       | Convert input numbers to binary, octal, decimal, hexadecimal |
| color    | Convert between RGB and Hex colors                           |
| md5      | Hash input strings and files using MD5                       |
| charcode | Convert string encodings                                     |
| flow     | Manage git repositories according to the Gitflow workflow    |

---

## How to Contribute to Wutong
Wutong is an open-source project, and we warmly welcome and highly anticipate developers from around the world to join and participate in its development.

You can contribute to Wutong in the following ways:
1. Submit bug reports and feature suggestions: If you encounter any bugs or have any feature suggestions while using Wutong, please refer to the [Wutong Security Guidelines](SECURITY.md).
2. Contribute code: If you have the ability and are willing to contribute code to Wutong, please refer to the [Wutong Contributor Guidelines](./CONTRIBUTING.md).

---

## Special Thanks
We sincerely thank the following individuals who have made outstanding contributions to Wutong (in alphabetical order):
- [Bob](https://github.com/ChepleBob26): Made numerous non-code contributions to the development of Wutong and was the first user.
- Silent: Suggested the excellent name for Wutong.

---

## Contributors

<a href="https://github.com/WutongDev/wutong/contributors">
  <img src="https://contrib.rocks/image?repo=WutongDev/wutong" alt="Contributors"/>
</a>

---

FAQ

Q1: Why is it named "Wutong"?
A: The name originates from developer Gavin's friend, Silent, whose favorite tree species is the Chinese parasol tree (梧桐). "Wutong" is the Pinyin transliteration of the tree's Chinese name.

Q2: Why was Rust chosen as the development language?
A: Rust was selected as the optimal technical choice after evaluating languages like Python, C++, and C, due to its cross-platform capabilities, memory safety features, and high-performance execution that align with the project's requirements.

Q3: Why is only macOS currently supported?
A: Current development prioritizes core feature refinement and rapid iteration. Windows and Linux versions are on the roadmap, with multi-platform adaptation scheduled to commence once core functionalities are stabilized.

Q4: Where can I learn more about Gavin?
A: Visit [Gavin's GitHub profile](https://github.com/GavZheng) to explore his technical contributions and open-source projects.

---

## License
[MIT](./LICENSE) © 2025 Gavin Zheng
