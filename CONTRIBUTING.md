# Contributing to Tanukeys

Thank you for your interest in contributing to Tanukeys! Your help is essential in making Tanukeys a robust and secure federated key management system. This document provides guidelines to ensure contributions are valuable and consistent.

## Table of Contents

- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features or Improvements](#suggesting-features-or-improvements)
- [Contributing Code](#contributing-code)
  - [Development Setup](#development-setup)
  - [Code Guidelines](#code-guidelines)
  - [Submitting a Pull Request](#submitting-a-pull-request)
- [Code of Conduct](#code-of-conduct)
- [Thank You](#thank-you)

## Reporting Bugs

We appreciate bug reports as they help us improve Tanukeys. Before submitting a bug report, please check the following:

- **Ensure the issue is not already reported**: Search the [issue tracker](https://github.com/tanukibox/tanukeys/issues) to see if the bug has already been reported or is being worked on.
- **Check the latest version**: Ensure you are running the latest version of Tanukeys before reporting the issue.
- **Include a clear and descriptive title**: This helps others quickly identify and triage the problem.
- **Provide detailed steps to reproduce**: Include:
  - What you did before encountering the bug.
  - What you expected to happen.
  - What actually happened.
  - Relevant logs, error messages, or screenshots.
- **Mention your environment**: Include details such as OS, Tanukeys version, database used, and deployment method.

To report a bug, [open an issue](https://github.com/tanukibox/tanukeys/issues/new?template=bug_report.md) and fill in the provided template.

## Suggesting Features or Improvements

If you have an idea for a new feature or an enhancement:

- **Search the existing issues** to check if a similar suggestion has already been made.
- **Clearly define the problem** your feature will solve.
- **Describe the proposed solution** in detail, including any potential trade-offs.
- **Include any alternative solutions** you’ve considered.
- **If possible, provide mockups or examples** to illustrate the idea.

Create a [feature request issue](https://github.com/tanukeys/tanukeys/issues/new?template=feature_request.md) with your proposal.

## Contributing Code

### Development Setup

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```sh
   git clone https://github.com/tanukibox/tanukeys.git
   ```
3. **Navigate to the project directory**:
   ```sh
   cd tanukeys
   ```
4. **Install dependencies** (if applicable):
   ```sh
   make deps
   ```
5. **Create a new branch**:
   ```sh
   git checkout -b feature-or-bugfix-branch
   ```

### Code Guidelines

- Follow the existing **coding style** and project structure.
- Write **clear and maintainable** code.
- **Comment where necessary**, especially for complex logic.
- Include **tests** for new features or bug fixes.
- Ensure your code **passes all tests** before submitting a PR.

### Submitting a Pull Request

1. **Push your changes** to your fork:
   ```sh
   git push origin feature-or-bugfix-name
   ```
2. **Open a Pull Request** against the `main` branch.
3. **Provide a detailed description** of the changes and link to related issues.
4. **Ensure your PR follows the template** and passes all checks.

A maintainer will review your PR and provide feedback or merge it if everything looks good.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md), which promotes a welcoming and inclusive environment for all contributors.

## Thank You

We appreciate your contributions and support in making Tanukeys a secure and federated key management system!

