#!/bin/bash

# EchoLayer Development Environment Startup Script

set -e

echo "🚀 Starting EchoLayer Development Environment..."

# Check if required tools are installed
check_prerequisites() {
    echo "📋 Checking prerequisites..."
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        echo "❌ Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/"
        exit 1
    fi
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        echo "❌ npm is not installed. Please install npm."
        exit 1
    fi
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    echo "✅ All prerequisites are installed."
}

# Install dependencies
install_dependencies() {
    echo "📦 Installing dependencies..."
    
    # Install frontend dependencies
    echo "🎨 Installing frontend dependencies..."
    cd frontend
    npm install
    cd ..
    
    # Install shared dependencies
    echo "🔗 Installing shared dependencies..."
    cd shared
    npm install
    cd ..
    
    echo "✅ Dependencies installed successfully."
}

# Setup environment variables
setup_environment() {
    echo "🔧 Setting up environment..."
    
    # Create .env files if they don't exist
    if [ ! -f "frontend/.env.local" ]; then
        echo "Creating frontend/.env.local..."
        cat > frontend/.env.local << EOF
# Frontend Environment Variables
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_BLOCKCHAIN_NETWORK=devnet
NEXT_PUBLIC_APP_ENV=development
EOF
    fi
    
    if [ ! -f "backend/.env" ]; then
        echo "Creating backend/.env..."
        cat > backend/.env << EOF
# Backend Environment Variables
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgresql://echolayer:password@localhost/echolayer_dev
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-here
SOLANA_RPC_URL=https://api.devnet.solana.com
ECHO_INDEX_CALCULATION_INTERVAL=300
LOG_LEVEL=info
EOF
    fi
    
    echo "✅ Environment setup complete."
}

# Build shared types
build_shared() {
    echo "🔨 Building shared types..."
    cd shared
    npm run build || echo "⚠️  Shared build failed, continuing..."
    cd ..
    echo "✅ Shared types build complete."
}

# Start services
start_services() {
    echo "🌟 Starting services..."
    
    # Function to start a service in background
    start_service() {
        local service_name=$1
        local command=$2
        local dir=$3
        
        echo "🚀 Starting $service_name..."
        cd $dir
        $command &
        local pid=$!
        echo $pid > "../.${service_name}.pid"
        echo "✅ $service_name started (PID: $pid)"
        cd ..
    }
    
    # Start backend
    echo "🔧 Starting backend server..."
    cd backend
    cargo run &
    BACKEND_PID=$!
    echo $BACKEND_PID > ../.backend.pid
    echo "✅ Backend started (PID: $BACKEND_PID)"
    cd ..
    
    # Start frontend
    echo "🎨 Starting frontend development server..."
    cd frontend
    npm run dev &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > ../.frontend.pid
    echo "✅ Frontend started (PID: $FRONTEND_PID)"
    cd ..
    
    echo ""
    echo "🎉 EchoLayer Development Environment is ready!"
    echo ""
    echo "📱 Frontend: http://localhost:3000"
    echo "🔧 Backend API: http://localhost:8080"
    echo "📊 Health Check: http://localhost:8080/api/v1/health"
    echo ""
    echo "💡 Tip: Use 'scripts/stop-dev.sh' to stop all services"
    echo ""
    echo "📝 Logs will appear below. Press Ctrl+C to stop all services."
    echo ""
}

# Cleanup function
cleanup() {
    echo ""
    echo "🛑 Stopping services..."
    
    if [ -f ".backend.pid" ]; then
        BACKEND_PID=$(cat .backend.pid)
        kill $BACKEND_PID 2>/dev/null || true
        rm .backend.pid
        echo "✅ Backend stopped"
    fi
    
    if [ -f ".frontend.pid" ]; then
        FRONTEND_PID=$(cat .frontend.pid)
        kill $FRONTEND_PID 2>/dev/null || true
        rm .frontend.pid
        echo "✅ Frontend stopped"
    fi
    
    echo "👋 Development environment stopped."
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Main execution
main() {
    check_prerequisites
    install_dependencies
    setup_environment
    build_shared
    start_services
    
    # Wait for services
    wait
}

# Run main function
main "$@" 