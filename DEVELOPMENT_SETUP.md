# EchoLayer Development Setup Guide

## 🚀 Quick Start

This guide will help you set up the EchoLayer development environment on your local machine.

## 📋 Prerequisites

### Required Tools
- **Node.js** (v18.0.0 or higher)
- **npm** or **yarn** package manager
- **Rust** (latest stable version)
- **Git** for version control

### Required Databases
- **PostgreSQL** (v14 or higher)
- **Redis** (v6 or higher)
- **Neo4j** (v4.4 or higher) - for graph data

### Optional Tools
- **Docker** and **Docker Compose** (recommended for database setup)
- **VS Code** with Rust and TypeScript extensions

## 🔧 Installation Steps

### 1. Install Node.js and npm
```bash
# Download and install from https://nodejs.org/
# Verify installation
node --version  # Should be v18.0.0+
npm --version
```

### 2. Install Rust
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell and verify
source ~/.cargo/env
rustc --version
cargo --version
```

### 3. Clone the Repository
```bash
git clone https://github.com/YourUsername/EchoLayer.git
cd EchoLayer
```

### 4. Install Dependencies

#### Frontend Dependencies
```bash
cd frontend
npm install
```

#### Shared Types
```bash
cd ../shared
npm install
npm run build
```

#### Backend Dependencies
```bash
cd ../backend
cargo build
```

## 🗄️ Database Setup

### Option 1: Docker (Recommended)
```bash
# In project root directory
docker-compose up -d

# This will start:
# - PostgreSQL on port 5432
# - Redis on port 6379
# - Neo4j on port 7474 (web) and 7687 (bolt)
```

### Option 2: Manual Installation

#### PostgreSQL
```bash
# Install PostgreSQL
# Create database
createdb echolayer_dev

# Create user
psql -c "CREATE USER echolayer WITH PASSWORD 'password';"
psql -c "GRANT ALL PRIVILEGES ON DATABASE echolayer_dev TO echolayer;"
```

#### Redis
```bash
# Install and start Redis
redis-server
```

#### Neo4j
```bash
# Download and install Neo4j Community Edition
# Start Neo4j service
# Access web interface at http://localhost:7474
```

## ⚙️ Environment Configuration

### Frontend Environment
```bash
cd frontend
cp .env.example .env.local

# Edit .env.local with your settings
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
```

### Backend Environment
```bash
cd backend
cp .env.example .env

# Edit .env with your database URLs
DATABASE_URL=postgresql://echolayer:password@localhost:5432/echolayer_dev
REDIS_URL=redis://localhost:6379
NEO4J_URL=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password
```

## 🚀 Running the Application

### 1. Start Database Services
```bash
# If using Docker
docker-compose up -d

# If using manual installation, ensure all services are running
```

### 2. Run Database Migrations
```bash
cd backend
cargo run --bin migrate
```

### 3. Start Backend Server
```bash
cd backend
cargo run
# Backend will start on http://localhost:8080
```

### 4. Start Frontend Development Server
```bash
cd frontend
npm run dev
# Frontend will start on http://localhost:3000
```

## 🧪 Development Workflow

### Running Tests
```bash
# Frontend tests
cd frontend
npm test

# Backend tests
cd backend
cargo test

# Type checking
cd frontend
npm run type-check
```

### Code Formatting
```bash
# Frontend (Prettier)
cd frontend
npm run format

# Backend (rustfmt)
cd backend
cargo fmt
```

### Linting
```bash
# Frontend (ESLint)
cd frontend
npm run lint

# Backend (Clippy)
cd backend
cargo clippy
```

## 📁 Project Structure Understanding

```
EchoLayer/
├── frontend/           # Next.js React frontend
│   ├── src/app/       # App Router pages
│   ├── src/components/# React components
│   ├── src/hooks/     # Custom React hooks
│   ├── src/store/     # Zustand state management
│   └── src/types/     # Frontend-specific TypeScript types
├── backend/           # Rust Actix-web backend
│   ├── src/handlers/  # HTTP route handlers
│   ├── src/models/    # Database models
│   ├── src/services/  # Business logic services
│   └── migrations/    # Database migrations
├── shared/            # Shared TypeScript types
└── assets/           # Static assets (logos, images)
```

## 🔍 Development Tools

### VS Code Extensions
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "ms-vscode.vscode-typescript-next",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "ms-vscode.vscode-json"
  ]
}
```

### Browser Developer Tools
- **React Developer Tools**
- **Redux DevTools** (for Zustand debugging)

## 🐛 Troubleshooting

### Common Issues

#### Frontend Issues
```bash
# Clear Next.js cache
cd frontend
rm -rf .next
npm run dev

# TypeScript errors
npm run type-check
```

#### Backend Issues
```bash
# Clean and rebuild
cd backend
cargo clean
cargo build

# Database connection issues
# Check .env file and ensure databases are running
```

#### Shared Types Issues
```bash
# Rebuild shared types
cd shared
npm run build

# Clear node_modules if needed
rm -rf node_modules
npm install
```

### Performance Tips
1. Use `npm run dev -- --turbo` for faster frontend builds
2. Use `cargo check` instead of `cargo build` for quick validation
3. Run databases in Docker for consistent environment

## 📚 Additional Resources

### Documentation
- [Next.js Documentation](https://nextjs.org/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix Web Guide](https://actix.rs/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)

### Project-Specific Guides
- `PROJECT_STATUS_REPORT.md` - Current project status
- `assets/LOGO_SETUP.md` - Logo setup instructions
- `CONTRIBUTING.md` - Contribution guidelines

## 🤝 Getting Help

1. Check existing issues in the GitHub repository
2. Read the troubleshooting section above
3. Review the project documentation
4. Create a new issue with detailed problem description

## 🔄 Development Cycle

1. **Pull latest changes**: `git pull origin main`
2. **Install/update dependencies**: `npm install` (frontend), `cargo build` (backend)
3. **Make changes**: Follow coding standards and conventions
4. **Test changes**: Run tests and type checking
5. **Commit changes**: Use conventional commit messages
6. **Push and create PR**: Submit for review

---

*Happy coding! 🚀* 