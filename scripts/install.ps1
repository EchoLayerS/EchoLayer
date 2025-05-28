# EchoLayer Project Installation Script for Windows
# This script sets up the development environment for EchoLayer on Windows

param(
    [switch]$SkipRust,
    [switch]$SkipSolana,
    [switch]$SkipAnchor
)

# Set error action preference
$ErrorActionPreference = "Stop"

Write-Host "🚀 Installing EchoLayer Development Environment..." -ForegroundColor Blue

# Function to print colored output
function Write-Status {
    param($Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param($Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param($Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param($Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if Node.js is installed
function Test-NodeJS {
    try {
        $nodeVersion = node --version
        Write-Success "Node.js is installed: $nodeVersion"
        return $true
    } catch {
        Write-Error "Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/"
        return $false
    }
}

# Check if Rust is installed
function Test-Rust {
    try {
        $rustVersion = rustc --version
        Write-Success "Rust is installed: $rustVersion"
        return $true
    } catch {
        if (-not $SkipRust) {
            Write-Warning "Rust is not installed. Please install Rust from https://rustup.rs/"
            Write-Warning "After installing Rust, restart PowerShell and run this script again."
        }
        return $false
    }
}

# Check if Solana CLI is installed
function Test-Solana {
    try {
        $solanaVersion = solana --version
        Write-Success "Solana CLI is installed: $solanaVersion"
        return $true
    } catch {
        if (-not $SkipSolana) {
            Write-Warning "Solana CLI is not installed."
            Write-Warning "Please install from: https://docs.solana.com/cli/install-solana-cli-tools"
        }
        return $false
    }
}

# Check if Anchor is installed
function Test-Anchor {
    try {
        $anchorVersion = anchor --version
        Write-Success "Anchor is installed: $anchorVersion"
        return $true
    } catch {
        if (-not $SkipAnchor) {
            Write-Warning "Anchor is not installed."
            Write-Warning "Please install from: https://www.anchor-lang.com/docs/installation"
        }
        return $false
    }
}

# Install frontend dependencies
function Install-Frontend {
    Write-Status "Installing frontend dependencies..."
    Push-Location -Path "frontend"
    try {
        npm install
        Write-Success "Frontend dependencies installed"
    } catch {
        Write-Error "Failed to install frontend dependencies: $_"
        throw
    } finally {
        Pop-Location
    }
}

# Install backend dependencies
function Install-Backend {
    if (Test-Rust) {
        Write-Status "Installing backend dependencies..."
        Push-Location -Path "backend"
        try {
            cargo build
            Write-Success "Backend dependencies installed"
        } catch {
            Write-Error "Failed to install backend dependencies: $_"
            throw
        } finally {
            Pop-Location
        }
    } else {
        Write-Warning "Skipping backend setup (Rust not available)"
    }
}

# Build smart contracts
function Build-Contracts {
    if (Test-Anchor) {
        Write-Status "Building smart contracts..."
        Push-Location -Path "smart-contracts"
        try {
            anchor build
            Write-Success "Smart contracts built successfully"
        } catch {
            Write-Warning "Failed to build smart contracts: $_"
        } finally {
            Pop-Location
        }
    } else {
        Write-Warning "Skipping smart contracts build (Anchor not available)"
    }
}

# Setup environment files
function Setup-Environment {
    Write-Status "Setting up environment files..."
    
    # Frontend environment
    $frontendEnv = "frontend\.env.local"
    if (-not (Test-Path $frontendEnv)) {
        $envContent = @"
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
NEXT_PUBLIC_SOLANA_NETWORK=devnet
"@
        $envContent | Out-File -FilePath $frontendEnv -Encoding UTF8
        Write-Success "Frontend environment file created"
    }
    
    # Backend environment
    $backendEnv = "backend\.env"
    if (-not (Test-Path $backendEnv)) {
        $envContent = @"
DATABASE_URL=postgresql://user:password@localhost:5432/echolayer
REDIS_URL=redis://localhost:6379
SOLANA_RPC_URL=https://api.devnet.solana.com
JWT_SECRET=your-super-secret-jwt-key-here
RUST_LOG=info
"@
        $envContent | Out-File -FilePath $backendEnv -Encoding UTF8
        Write-Success "Backend environment file created"
    }
}

# Setup database
function Setup-Database {
    Write-Status "Checking database requirements..."
    Write-Warning "Please ensure PostgreSQL is installed and configured"
    Write-Warning "Update the DATABASE_URL in backend\.env with your credentials"
}

# Main installation function
function Start-Installation {
    Write-Host "🌊 EchoLayer Development Environment Setup" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Status "Checking system requirements..."
    $nodeOk = Test-NodeJS
    $rustOk = Test-Rust
    $solanaOk = Test-Solana
    $anchorOk = Test-Anchor
    
    if (-not $nodeOk) {
        Write-Error "Node.js is required but not installed. Please install it first."
        exit 1
    }
    
    Write-Status "Installing project dependencies..."
    Install-Frontend
    Install-Backend
    Build-Contracts
    
    Write-Status "Setting up configuration..."
    Setup-Environment
    Setup-Database
    
    Write-Host ""
    Write-Host "🎉 Installation Complete!" -ForegroundColor Green
    Write-Host "========================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:"
    Write-Host "1. Configure your database connection in backend\.env"
    Write-Host "2. Start the development servers:"
    Write-Host "   - Frontend: cd frontend; npm run dev"
    Write-Host "   - Backend:  cd backend; cargo run"
    Write-Host "3. Access the application at http://localhost:3000"
    Write-Host ""
    Write-Host "For more information, see the README.md file."
    Write-Host ""
    Write-Success "Happy coding! 🚀"
}

# Check if running in the correct directory
if (-not (Test-Path "package.json" -PathType Leaf) -and -not (Test-Path "frontend" -PathType Container)) {
    Write-Error "Please run this script from the EchoLayer project root directory"
    exit 1
}

# Run main installation
try {
    Start-Installation
} catch {
    Write-Error "Installation failed: $_"
    exit 1
} 