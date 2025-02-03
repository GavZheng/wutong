# Wutong Contributor Guide


Thank you for your attention and support to the Wutong project! Wutong is a command-line tool designed to enhance developer efficiency, and we warmly welcome your contributions, whether it's reporting issues, suggesting improvements, or directly participating in code development. Below are some guidelines to help you better contribute to the Wutong project.

> [!IMPORTANT]
> We require all contributors to read and sign the [Contributor License Agreement](CLA_zh.md).

## Wutong Design Philosophy: Focus, Efficiency, and Integration

In the process of software development, developers often need to handle various data conversion and encoding tasks. However, these tasks often require them to step away from their efficient development environment, search for external tools or websites, and engage in cumbersome interactive operations, which is not only time-consuming but also easily disrupts workflow, affecting concentration and inspiration.

Wutong's design philosophy aims to solve this problem by pursuing the following core values:

1. **Focus**: Wutong allows developers to complete data conversion and encoding tasks directly in the terminal without leaving their development environment, avoiding frequent context switching and maintaining workflow coherence and focus.

2. **Efficiency**: As a command-line tool, Wutong integrates various common data conversion and encoding functions, enabling developers to accomplish more work in the least amount of time and improve coding efficiency.

3. **Integration**: The seamless integration of Wutong in the development environment allows developers to effortlessly incorporate Wutong into their existing workflows, without any additional adaptation or learning costs.

In summary, Wutong's design philosophy revolves around enhancing developer efficiency and maintaining focus. It is committed to creating a seamless, efficient, and integrated development environment that allows developers to focus more on the code itself and create outstanding software products.

## How to Contribute

### 1. Reporting Issues

If you encounter any issues while using Wutong, please submit a new issue in the project's [Issues](https://github.com/WutongDev/wutong/issues). When submitting, please provide the following information as much as possible:

- **Issue Description**: Describe the issue you encountered in detail.
- **Steps to Reproduce**: Provide steps to reproduce the issue.
- **Expected Result**: Describe the result you expected.
- **Actual Result**: Describe the result you actually got.
- **Environment Information**: Include information such as the operating system and Wutong version.

### 2. Suggesting Features

If you believe Wutong lacks certain features or if you have suggestions for improving existing features, you are also welcome to submit a new [Issue](https://github.com/WutongDev/wutong/issues).

Before submitting, please ensure:
- You have searched and **not** found similar issues.
- You have searched the documentation and **not** found relevant content.
- You have tried using the **latest version** and the issue still exists.
- You have **not** modified or replaced any program files.

In your suggestion, please include the following information:
- **Feature Description**: Describe in detail the feature you wish to add or improve.
- **Usage Scenario**: Provide scenarios where this feature can be used in actual development.
- **Implementation Suggestions**: If you have specific implementation suggestions, you can also propose them here.

### 3. Contributing to Code Development

If you are interested in Wutong's code and wish to directly participate in development, you can follow these steps:

> [!IMPORTANT]
> You may have noticed that Wutong is using `gitflow` as its branch management strategy. Therefore, before starting development, please ensure you understand this strategy and how to use it.

1. **Fork the Repository**: Fork Wutong's repository to your personal account on GitHub.
2. **Clone the Repository**: Clone the forked repository to your local development environment.
3. **Create a Branch**: Create a new branch for the feature you want to develop or the issue you want to fix.
4. **Write Code**: Write code on the branch and conduct tests.
5. **Commit Code**: Commit your code to the new branch and push it to GitHub. For commit messages, please refer to the [Commit Message Guidelines](#commit-message-guidelines).
6. **Create a Pull Request**: Create a Pull Request on GitHub to merge your branch into the corresponding development branch of Wutong.

When committing code, please try to follow these guidelines:

- **Code Style**: Before committing, please use `Clippy` to format your code.
- **Testing**: Write test cases for new or modified features in the `/src/tests/` directory and ensure all tests pass.
- **Documentation**: Update related documentation, including files in the `/docs/` directory, the `README.md` file in the root directory, etc.

> [!CAUTION]
> Please be sure to follow the above rules; otherwise, your code may be rejected for merging.

#### Commit Message Guidelines

The commit format is as follows:

```
<type>: <subject>
<blank line>
<body>
<blank line>
<footer>
```

The commit type is specified as one of the following:
- `build`: Changes to the build system or external dependencies
- `ci`: Changes to CI configuration files or scripts
- `docs`: Changes to documentation
- `feat`: Add a new feature
- `fix`: Fix a bug
- `perf`: Code changes that improve performance
- `refactor`: Code refactoring that is neither fixing a bug nor adding a feature
- `style`: Changes that do not affect code meaning, such as spaces, formatting, missing semicolons, etc.
- `test`: Add missing tests or correct existing tests

**Subject**
A concise description of the change, requiring:
- Use the **imperative**, present tense
- **Do not** capitalize the first letter
- **Do not** add a period at the end

**Body**
- Similar to the subject setting, use the imperative and present tense.
- Should include the motivation for the change and a comparison with previous behavior.

**Footer**
If this commit aims to fix an issue, you need to reference the issue in the footer.

Start with the keyword "Closes", for example:
```Closes #234```

If multiple bugs are fixed, separate them with commas:
```Closes #123, #245, #992```

When this commit includes a revert operation, the type starts with `revert:`, and add `This reverts commit [hash]` in the body, where `hash` represents the commit being reverted.

## Contact Us

If you have any questions or suggestions, please feel free to contact us through the following methods:
- Gavin Zheng <gav.zheng@outlook.com>
- Github Repository: [WutongDev/wutong](https://github.com/WutongDev/wutong)
