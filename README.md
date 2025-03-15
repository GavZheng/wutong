<h1 style="text-align: center;">Wutong - The Swiss Army Knife for Developers</h1>

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
If you are a MacOS user, you can use:
```bash
brew install wutong
```
If you are a Windows or Linux user, please wait for the subsequent version.  
P.S.: Of course, you can compile Wutong yourself.

---

## How to Use Wutong
You can obtain detailed information by typing `wutong --help` in the command line.  
The general functions of Wutong are listed below (v0.1.0-beta):

| Function | Description                                                  |
|----------|--------------------------------------------------------------|
| base     | Encode input strings in base32 or base64                     |
| bc       | Convert input numbers to binary, octal, decimal, hexadecimal |
| color    | Convert between RGB and Hex colors                           |
| md5      | Hash input strings using MD5                                 |
| charcode | Convert string encodings                                     |

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

## License
[MIT](./LICENSE) © 2025 Gavin Zheng
