# Contributing to Wagering Smart Contract

Thank you for your interest in contributing to the Wagering Smart Contract project! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have:
- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) v1.16+
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) v0.28+
- [Node.js](https://nodejs.org/) v16+
- [Git](https://git-scm.com/)

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/wagering-smart-contract.git
   cd wagering-smart-contract
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build the project**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

## 📋 Development Guidelines

### Code Style

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use meaningful variable and function names
- Add comprehensive documentation for public APIs
- Include inline comments for complex logic

### Testing Requirements

- All new features must include comprehensive tests
- Maintain or improve test coverage
- Test both success and failure scenarios
- Include integration tests for complex workflows

### Security Considerations

This is a financial smart contract. All contributions must:
- Validate all user inputs
- Handle edge cases properly
- Follow secure coding practices
- Consider potential attack vectors

## 🔧 Pull Request Process

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write clean, well-documented code
   - Add appropriate tests
   - Update documentation if needed

3. **Test thoroughly**
   ```bash
   anchor test
   cargo test
   ```

4. **Commit with clear messages**
   ```bash
   git commit -m "feat: add new game session validation"
   ```

5. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

### Commit Message Format

Use conventional commits:
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/modifications
- `refactor:` - Code refactoring
- `security:` - Security improvements

## 🐛 Bug Reports

When reporting bugs, please include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Relevant code snippets or logs

## 💡 Feature Requests

For new features:
- Describe the use case clearly
- Explain the expected behavior
- Consider security implications
- Provide implementation suggestions if possible

## 📚 Documentation

- Update README.md for user-facing changes
- Add inline documentation for new functions
- Update API documentation
- Include examples for new features

## ⚖️ Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Maintain professionalism in all interactions

## 🔍 Review Process

All contributions go through:
1. Automated testing
2. Code review by maintainers
3. Security assessment (for critical changes)
4. Integration testing

## 📞 Getting Help

- **Issues**: Use GitHub Issues for bugs and features
- **Discussions**: Use GitHub Discussions for questions
- **Security**: Email security issues privately

Thank you for contributing! 🎉
