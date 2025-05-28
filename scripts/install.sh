#!/bin/bash

# EchoLayer Project Installation Script
# This script sets up the development environment for EchoLayer

set -e

echo "🚀 Installing EchoLayer Development Environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Node.js is installed
check_nodejs() {
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        print_success "Node.js is installed: $NODE_VERSION"
    else
        print_error "Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/"
        exit 1
    fi
}

# Check if Rust is installed
check_rust() {
    if command -v cargo &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        print_success "Rust is installed: $RUST_VERSION"
    else
        print_warning "Rust is not installed. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        print_success "Rust installed successfully"
    fi
}

# Check if Solana CLI is installed
check_solana() {
    if command -v solana &> /dev/null; then
        SOLANA_VERSION=$(solana --version)
        print_success "Solana CLI is installed: $SOLANA_VERSION"
    else
        print_warning "Solana CLI is not installed. Installing Solana CLI..."
        sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
        export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
        print_success "Solana CLI installed successfully"
    fi
}

# Check if Anchor is installed
check_anchor() {
    if command -v anchor &> /dev/null; then
        ANCHOR_VERSION=$(anchor --version)
        print_success "Anchor is installed: $ANCHOR_VERSION"
    else
        print_warning "Anchor is not installed. Installing Anchor..."
        cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
        avm install latest
        avm use latest
        print_success "Anchor installed successfully"
    fi
}

# Install frontend dependencies
install_frontend() {
    print_status "Installing frontend dependencies..."
    cd frontend
    npm install
    print_success "Frontend dependencies installed"
    cd ..
}

# Install backend dependencies
install_backend() {
    print_status "Installing backend dependencies..."
    cd backend
    cargo build
    print_success "Backend dependencies installed"
    cd ..
}

# Build smart contracts
build_contracts() {
    print_status "Building smart contracts..."
    cd smart-contracts
    if command -v anchor &> /dev/null; then
        anchor build
        print_success "Smart contracts built successfully"
    else
        print_warning "Skipping smart contracts build (Anchor not available)"
    fi
    cd ..
}

# Setup environment files
setup_env() {
    print_status "Setting up environment files..."
    
    # Frontend environment
    if [ ! -f "frontend/.env.local" ]; then
        cp frontend/.env.example frontend/.env.local 2>/dev/null || cat > frontend/.env.local << EOF
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
NEXT_PUBLIC_SOLANA_NETWORK=devnet
EOF
        print_success "Frontend environment file created"
    fi
    
    # Backend environment
    if [ ! -f "backend/.env" ]; then
        cat > backend/.env << EOF
DATABASE_URL=postgresql://user:password@localhost:5432/echolayer
REDIS_URL=redis://localhost:6379
SOLANA_RPC_URL=https://api.devnet.solana.com
JWT_SECRET=your-super-secret-jwt-key-here
RUST_LOG=info
EOF
        print_success "Backend environment file created"
    fi
}

# Setup database (if PostgreSQL is available)
setup_database() {
    if command -v psql &> /dev/null; then
        print_status "Setting up database..."
        # This is a basic setup - users should configure their own database
        print_warning "Please configure your PostgreSQL database manually"
        print_warning "Update the DATABASE_URL in backend/.env with your credentials"
    else
        print_warning "PostgreSQL not found. Please install PostgreSQL for backend functionality"
    fi
}

# Main installation function
main() {
    echo "🌊 EchoLayer Development Environment Setup"
    echo "=========================================="
    
    print_status "Checking system requirements..."
    check_nodejs
    check_rust
    check_solana
    check_anchor
    
    print_status "Installing project dependencies..."
    install_frontend
    install_backend
    build_contracts
    
    print_status "Setting up configuration..."
    setup_env
    setup_database
    
    echo ""
    echo "🎉 Installation Complete!"
    echo "========================"
    echo ""
    echo "Next steps:"
    echo "1. Configure your database connection in backend/.env"
    echo "2. Start the development servers:"
    echo "   - Frontend: cd frontend && npm run dev"
    echo "   - Backend:  cd backend && cargo run"
    echo "3. Access the application at http://localhost:3000"
    echo ""
    echo "For more information, see the README.md file."
    echo ""
    print_success "Happy coding! 🚀"
}

# Run main function
main 