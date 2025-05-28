# EchoLayer Development Environment Startup Script for Windows

Write-Host "🚀 Starting EchoLayer Development Environment..." -ForegroundColor Green

# Check if required tools are installed
function Check-Prerequisites {
    Write-Host "📋 Checking prerequisites..." -ForegroundColor Yellow
    
    # Check Node.js
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Node.js is not installed. Please install Node.js 18+ from https://nodejs.org/" -ForegroundColor Red
        exit 1
    }
    
    # Check npm
    if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
        Write-Host "❌ npm is not installed. Please install npm." -ForegroundColor Red
        exit 1
    }
    
    # Check Rust
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "❌ Rust is not installed. Please install Rust from https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ All prerequisites are installed." -ForegroundColor Green
}

# Install dependencies
function Install-Dependencies {
    Write-Host "📦 Installing dependencies..." -ForegroundColor Yellow
    
    # Install frontend dependencies
    Write-Host "🎨 Installing frontend dependencies..." -ForegroundColor Cyan
    Set-Location frontend
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Frontend dependency installation failed" -ForegroundColor Red
        exit 1
    }
    Set-Location ..
    
    # Install shared dependencies
    Write-Host "🔗 Installing shared dependencies..." -ForegroundColor Cyan
    Set-Location shared
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Shared dependency installation failed" -ForegroundColor Red
        exit 1
    }
    Set-Location ..
    
    Write-Host "✅ Dependencies installed successfully." -ForegroundColor Green
}

# Setup environment variables
function Setup-Environment {
    Write-Host "🔧 Setting up environment..." -ForegroundColor Yellow
    
    # Create .env files if they don't exist
    if (-not (Test-Path "frontend\.env.local")) {
        Write-Host "Creating frontend\.env.local..." -ForegroundColor Cyan
        @"
# Frontend Environment Variables
NEXT_PUBLIC_API_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_BLOCKCHAIN_NETWORK=devnet
NEXT_PUBLIC_APP_ENV=development
"@ | Out-File -FilePath "frontend\.env.local" -Encoding UTF8
    }
    
    if (-not (Test-Path "backend\.env")) {
        Write-Host "Creating backend\.env..." -ForegroundColor Cyan
        @"
# Backend Environment Variables
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgresql://echolayer:password@localhost/echolayer_dev
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key-here
SOLANA_RPC_URL=https://api.devnet.solana.com
ECHO_INDEX_CALCULATION_INTERVAL=300
LOG_LEVEL=info
"@ | Out-File -FilePath "backend\.env" -Encoding UTF8
    }
    
    Write-Host "✅ Environment setup complete." -ForegroundColor Green
}

# Build shared types
function Build-Shared {
    Write-Host "🔨 Building shared types..." -ForegroundColor Yellow
    Set-Location shared
    try {
        npm run build
        Write-Host "✅ Shared types build complete." -ForegroundColor Green
    } catch {
        Write-Host "⚠️  Shared build failed, continuing..." -ForegroundColor Yellow
    }
    Set-Location ..
}

# Start services
function Start-Services {
    Write-Host "🌟 Starting services..." -ForegroundColor Yellow
    
    # Start backend
    Write-Host "🔧 Starting backend server..." -ForegroundColor Cyan
    Set-Location backend
    $BackendJob = Start-Job -ScriptBlock { 
        Set-Location $using:PWD
        cargo run 
    }
    $BackendJob.Id | Out-File -FilePath "../.backend.pid" -Encoding UTF8
    Write-Host "✅ Backend started (Job ID: $($BackendJob.Id))" -ForegroundColor Green
    Set-Location ..
    
    # Start frontend
    Write-Host "🎨 Starting frontend development server..." -ForegroundColor Cyan
    Set-Location frontend
    $FrontendJob = Start-Job -ScriptBlock { 
        Set-Location $using:PWD
        npm run dev 
    }
    $FrontendJob.Id | Out-File -FilePath "../.frontend.pid" -Encoding UTF8
    Write-Host "✅ Frontend started (Job ID: $($FrontendJob.Id))" -ForegroundColor Green
    Set-Location ..
    
    Write-Host ""
    Write-Host "🎉 EchoLayer Development Environment is ready!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📱 Frontend: http://localhost:3000" -ForegroundColor Cyan
    Write-Host "🔧 Backend API: http://localhost:8080" -ForegroundColor Cyan
    Write-Host "📊 Health Check: http://localhost:8080/api/v1/health" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "💡 Tip: Use 'scripts/stop-dev.ps1' to stop all services" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "📝 Press Ctrl+C to stop all services" -ForegroundColor Yellow
    Write-Host ""
    
    # Keep the script running and monitor jobs
    try {
        while ($true) {
            Start-Sleep -Seconds 5
            
            # Check if jobs are still running
            if (Get-Job -Id $BackendJob.Id -ErrorAction SilentlyContinue) {
                if ((Get-Job -Id $BackendJob.Id).State -eq "Failed") {
                    Write-Host "❌ Backend job failed" -ForegroundColor Red
                    break
                }
            }
            
            if (Get-Job -Id $FrontendJob.Id -ErrorAction SilentlyContinue) {
                if ((Get-Job -Id $FrontendJob.Id).State -eq "Failed") {
                    Write-Host "❌ Frontend job failed" -ForegroundColor Red
                    break
                }
            }
        }
    } catch {
        Write-Host "🛑 Stopping services..." -ForegroundColor Yellow
        
        # Stop jobs
        if (Test-Path ".backend.pid") {
            $BackendJobId = Get-Content ".backend.pid"
            Stop-Job -Id $BackendJobId -ErrorAction SilentlyContinue
            Remove-Job -Id $BackendJobId -ErrorAction SilentlyContinue
            Remove-Item ".backend.pid" -ErrorAction SilentlyContinue
            Write-Host "✅ Backend stopped" -ForegroundColor Green
        }
        
        if (Test-Path ".frontend.pid") {
            $FrontendJobId = Get-Content ".frontend.pid"
            Stop-Job -Id $FrontendJobId -ErrorAction SilentlyContinue
            Remove-Job -Id $FrontendJobId -ErrorAction SilentlyContinue
            Remove-Item ".frontend.pid" -ErrorAction SilentlyContinue
            Write-Host "✅ Frontend stopped" -ForegroundColor Green
        }
        
        Write-Host "👋 Development environment stopped." -ForegroundColor Green
    }
}

# Main execution
function Main {
    Check-Prerequisites
    Install-Dependencies
    Setup-Environment
    Build-Shared
    Start-Services
}

# Handle Ctrl+C
$null = Register-EngineEvent PowerShell.Exiting -Action {
    Write-Host "🛑 Stopping services..." -ForegroundColor Yellow
    
    if (Test-Path ".backend.pid") {
        $BackendJobId = Get-Content ".backend.pid"
        Stop-Job -Id $BackendJobId -ErrorAction SilentlyContinue
        Remove-Job -Id $BackendJobId -ErrorAction SilentlyContinue
        Remove-Item ".backend.pid" -ErrorAction SilentlyContinue
        Write-Host "✅ Backend stopped" -ForegroundColor Green
    }
    
    if (Test-Path ".frontend.pid") {
        $FrontendJobId = Get-Content ".frontend.pid"
        Stop-Job -Id $FrontendJobId -ErrorAction SilentlyContinue
        Remove-Job -Id $FrontendJobId -ErrorAction SilentlyContinue
        Remove-Item ".frontend.pid" -ErrorAction SilentlyContinue
        Write-Host "✅ Frontend stopped" -ForegroundColor Green
    }
}

# Run main function
Main 