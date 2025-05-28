# Contributing to EchoLayer

We appreciate your interest in contributing to EchoLayer! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/EchoLayer.git`
3. Install dependencies: `./scripts/install.sh` (or `.\scripts\install.ps1` on Windows)
4. Create a new branch: `git checkout -b feature/your-feature-name`
5. Make your changes and commit them
6. Push to your fork and submit a pull request

## 🛠️ Development Setup

### Prerequisites
- Node.js 18+
- Rust 1.70+
- PostgreSQL 14+ (optional for MVP)
- Redis 6+ (optional for MVP)

### Local Development
```bash
# Install dependencies
cd frontend && npm install

# Start frontend development server
npm run dev

# Start backend development server (in another terminal)
cd backend && cargo run
```

## 📝 Code Style

### Frontend (TypeScript/React)
- Use TypeScript for all new code
- Follow ESLint configuration
- Use functional components with hooks
- Prefer named exports over default exports
- Use Tailwind CSS for styling

### Backend (Rust)
- Follow Rust formatting standards (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Write comprehensive tests
- Document public APIs

### Smart Contracts (Solana/Anchor)
- Follow Anchor framework conventions
- Include comprehensive tests
- Document all public instructions

## 🧪 Testing

### Frontend
```bash
cd frontend
npm run test
npm run type-check
```

### Backend
```bash
cd backend
cargo test
cargo clippy
```

### Smart Contracts
```bash
cd smart-contracts
anchor test
```

## 📋 Pull Request Process

1. **Create an Issue**: For significant changes, create an issue first to discuss the approach
2. **Branch Naming**: Use descriptive branch names:
   - `feature/echo-index-optimization`
   - `fix/propagation-tracking-bug`
   - `docs/api-documentation-update`
3. **Commit Messages**: Use conventional commits:
   - `feat: add Echo Loop mechanism`
   - `fix: resolve wallet connection issue`
   - `docs: update API documentation`
4. **Testing**: Ensure all tests pass and add new tests for new features
5. **Documentation**: Update relevant documentation
6. **Code Review**: Address all review comments before merging

## 🎯 Areas for Contribution

### High Priority
- Echo Index calculation improvements
- Propagation tracking algorithms
- UI/UX enhancements
- Performance optimizations

### Medium Priority
- Additional platform integrations
- Advanced visualization features
- Smart contract optimizations
- API documentation

### Low Priority
- Code refactoring
- Additional tests
- Documentation improvements
- Tooling enhancements

## 📚 Learning Resources

### EchoLayer Concepts
- [Project Plan](./PROJECT_PLAN.md)
- [API Documentation](./docs/API.md)
- [Architecture Overview](./README.md#architecture)

### Technology Stack
- [Next.js Documentation](https://nextjs.org/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Framework](https://book.anchor-lang.com/)

## 🐛 Reporting Bugs

When reporting bugs, please include:
1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce the bug
3. **Expected Behavior**: What you expected to happen
4. **Actual Behavior**: What actually happened
5. **Environment**: OS, browser, versions, etc.
6. **Screenshots**: If applicable

## 💡 Feature Requests

For feature requests, please include:
1. **Use Case**: Why is this feature needed?
2. **Description**: Detailed description of the feature
3. **Implementation Ideas**: Any thoughts on implementation
4. **Alternatives**: Other solutions you've considered

## 🏗️ Architecture Guidelines

### Frontend Architecture
- Use Next.js App Router for routing
- Implement state management with Zustand
- Create reusable components in `/components`
- Use custom hooks for complex logic

### Backend Architecture
- Follow Clean Architecture principles
- Separate business logic from HTTP handlers
- Use proper error handling and logging
- Implement comprehensive input validation

### Smart Contract Architecture
- Keep contracts simple and focused
- Implement proper access controls
- Use events for important state changes
- Optimize for gas efficiency

## 🔒 Security Considerations

- Never commit sensitive information (keys, passwords, etc.)
- Follow security best practices for smart contracts
- Implement proper input validation
- Use secure coding practices

## �� Getting Help

- **GitHub Discussions**: Use for general questions
- **Issues**: Use for bug reports and feature requests

## 📄 License

By contributing to EchoLayer, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to EchoLayer! Together, we're building the future of decentralized attention tracking. 🚀 