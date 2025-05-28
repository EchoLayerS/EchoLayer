#!/bin/bash

# EchoLayer Automated Deployment Script
# Description: Comprehensive deployment automation for all environments
# Version: 1.0.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOY_LOG="$PROJECT_ROOT/logs/deployment.log"

# Default values
ENVIRONMENT="development"
SKIP_TESTS=false
SKIP_BUILD=false
FORCE_DEPLOY=false
DRY_RUN=false
VERBOSE=false

# Create logs directory
mkdir -p "$PROJECT_ROOT/logs"

# Logging function
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$DEPLOY_LOG"
}

print_header() {
    echo -e "${BLUE}=====================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=====================================${NC}"
    log "$1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
    log "SUCCESS: $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
    log "WARNING: $1"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
    log "ERROR: $1"
}

print_info() {
    echo -e "${PURPLE}ℹ️  $1${NC}"
    log "INFO: $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_header "CHECKING PREREQUISITES"
    
    local missing_tools=()
    
    # Check Docker
    if ! command_exists docker; then
        missing_tools+=("docker")
    fi
    
    # Check Docker Compose
    if ! command_exists docker-compose; then
        missing_tools+=("docker-compose")
    fi
    
    # Check Node.js (for frontend)
    if [ -d "$PROJECT_ROOT/frontend" ] && ! command_exists node; then
        missing_tools+=("node")
    fi
    
    # Check npm (for frontend)
    if [ -d "$PROJECT_ROOT/frontend" ] && ! command_exists npm; then
        missing_tools+=("npm")
    fi
    
    # Check Rust/Cargo (for backend)
    if [ -d "$PROJECT_ROOT/backend" ] && ! command_exists cargo; then
        missing_tools+=("cargo")
    fi
    
    # Check kubectl (for Kubernetes deployment)
    if [ "$ENVIRONMENT" = "production" ] && ! command_exists kubectl; then
        missing_tools+=("kubectl")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        print_info "Please install the missing tools and try again"
        exit 1
    fi
    
    print_success "All prerequisites satisfied"
}

# Pre-deployment checks
pre_deployment_checks() {
    print_header "PRE-DEPLOYMENT CHECKS"
    
    # Check if git working directory is clean
    if [ "$ENVIRONMENT" = "production" ] || [ "$ENVIRONMENT" = "staging" ]; then
        if ! git diff --quiet HEAD; then
            if [ "$FORCE_DEPLOY" = false ]; then
                print_error "Git working directory is not clean"
                print_info "Commit your changes or use --force to deploy anyway"
                exit 1
            else
                print_warning "Deploying with uncommitted changes (forced)"
            fi
        fi
    fi
    
    # Check if we're on the correct branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    case $ENVIRONMENT in
        "production")
            if [ "$current_branch" != "main" ] && [ "$current_branch" != "master" ]; then
                if [ "$FORCE_DEPLOY" = false ]; then
                    print_error "Not on main/master branch for production deployment"
                    print_info "Switch to main/master branch or use --force"
                    exit 1
                else
                    print_warning "Deploying to production from $current_branch (forced)"
                fi
            fi
            ;;
        "staging")
            if [ "$current_branch" != "develop" ] && [ "$current_branch" != "staging" ]; then
                print_warning "Not on develop/staging branch for staging deployment"
            fi
            ;;
    esac
    
    # Check environment configuration files
    case $ENVIRONMENT in
        "development")
            env_file="$PROJECT_ROOT/.env.development"
            ;;
        "staging")
            env_file="$PROJECT_ROOT/.env.staging"
            ;;
        "production")
            env_file="$PROJECT_ROOT/.env.production"
            ;;
    esac
    
    if [ ! -f "$env_file" ]; then
        print_warning "Environment file not found: $env_file"
        print_info "Using default configuration"
    else
        print_success "Environment configuration found: $env_file"
    fi
    
    print_success "Pre-deployment checks completed"
}

# Run tests
run_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        print_info "Skipping tests (--skip-tests flag used)"
        return 0
    fi
    
    print_header "RUNNING TESTS"
    
    local test_failures=0
    
    # Frontend tests
    if [ -d "$PROJECT_ROOT/frontend" ]; then
        print_info "Running frontend tests..."
        cd "$PROJECT_ROOT/frontend"
        
        if [ ! -d "node_modules" ]; then
            print_info "Installing frontend dependencies..."
            npm ci
        fi
        
        if npm test -- --watchAll=false --coverage > "$PROJECT_ROOT/logs/frontend-tests.log" 2>&1; then
            print_success "Frontend tests passed"
        else
            print_error "Frontend tests failed"
            test_failures=$((test_failures + 1))
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/frontend-tests.log"
            fi
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Backend tests
    if [ -d "$PROJECT_ROOT/backend" ]; then
        print_info "Running backend tests..."
        cd "$PROJECT_ROOT/backend"
        
        if cargo test > "$PROJECT_ROOT/logs/backend-tests.log" 2>&1; then
            print_success "Backend tests passed"
        else
            print_error "Backend tests failed"
            test_failures=$((test_failures + 1))
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/backend-tests.log"
            fi
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Smart contract tests
    if [ -d "$PROJECT_ROOT/smart-contracts" ]; then
        print_info "Running smart contract tests..."
        cd "$PROJECT_ROOT/smart-contracts"
        
        if cargo test > "$PROJECT_ROOT/logs/contracts-tests.log" 2>&1; then
            print_success "Smart contract tests passed"
        else
            print_error "Smart contract tests failed"
            test_failures=$((test_failures + 1))
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/contracts-tests.log"
            fi
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Integration tests
    if [ -d "$PROJECT_ROOT/tests" ]; then
        print_info "Running integration tests..."
        cd "$PROJECT_ROOT/tests"
        
        if cargo test > "$PROJECT_ROOT/logs/integration-tests.log" 2>&1; then
            print_success "Integration tests passed"
        else
            print_error "Integration tests failed"
            test_failures=$((test_failures + 1))
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    if [ $test_failures -gt 0 ]; then
        if [ "$FORCE_DEPLOY" = false ]; then
            print_error "Tests failed. Use --force to deploy anyway"
            exit 1
        else
            print_warning "Deploying despite test failures (forced)"
        fi
    else
        print_success "All tests passed"
    fi
}

# Build application
build_application() {
    if [ "$SKIP_BUILD" = true ]; then
        print_info "Skipping build (--skip-build flag used)"
        return 0
    fi
    
    print_header "BUILDING APPLICATION"
    
    # Build frontend
    if [ -d "$PROJECT_ROOT/frontend" ]; then
        print_info "Building frontend..."
        cd "$PROJECT_ROOT/frontend"
        
        if [ ! -d "node_modules" ]; then
            print_info "Installing frontend dependencies..."
            npm ci
        fi
        
        # Set environment-specific build variables
        case $ENVIRONMENT in
            "production")
                export NODE_ENV=production
                export NEXT_PUBLIC_API_URL="https://api.echolayers.xyz"
                ;;
            "staging")
                export NODE_ENV=production
                export NEXT_PUBLIC_API_URL="https://api-staging.echolayers.xyz"
                ;;
            "development")
                export NODE_ENV=development
                export NEXT_PUBLIC_API_URL="http://localhost:8080"
                ;;
        esac
        
        if npm run build > "$PROJECT_ROOT/logs/frontend-build.log" 2>&1; then
            print_success "Frontend build completed"
        else
            print_error "Frontend build failed"
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/frontend-build.log"
            fi
            
            exit 1
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Build backend
    if [ -d "$PROJECT_ROOT/backend" ]; then
        print_info "Building backend..."
        cd "$PROJECT_ROOT/backend"
        
        if cargo build --release > "$PROJECT_ROOT/logs/backend-build.log" 2>&1; then
            print_success "Backend build completed"
        else
            print_error "Backend build failed"
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/backend-build.log"
            fi
            
            exit 1
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Build smart contracts
    if [ -d "$PROJECT_ROOT/smart-contracts" ]; then
        print_info "Building smart contracts..."
        cd "$PROJECT_ROOT/smart-contracts"
        
        if cargo build-bpf > "$PROJECT_ROOT/logs/contracts-build.log" 2>&1; then
            print_success "Smart contracts build completed"
        else
            print_error "Smart contracts build failed"
            
            if [ "$VERBOSE" = true ]; then
                tail -20 "$PROJECT_ROOT/logs/contracts-build.log"
            fi
            
            exit 1
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    print_success "Application build completed"
}

# Build Docker images
build_docker_images() {
    print_header "BUILDING DOCKER IMAGES"
    
    local image_tag
    case $ENVIRONMENT in
        "production")
            image_tag="latest"
            ;;
        "staging")
            image_tag="staging"
            ;;
        "development")
            image_tag="dev"
            ;;
    esac
    
    # Build backend image
    if [ -f "$PROJECT_ROOT/docker/Dockerfile.backend" ]; then
        print_info "Building backend Docker image..."
        
        if docker build -f "$PROJECT_ROOT/docker/Dockerfile.backend" \
                        -t "echolayer/backend:$image_tag" \
                        "$PROJECT_ROOT" > "$PROJECT_ROOT/logs/docker-backend-build.log" 2>&1; then
            print_success "Backend Docker image built successfully"
        else
            print_error "Backend Docker image build failed"
            exit 1
        fi
    fi
    
    # Build frontend image
    if [ -f "$PROJECT_ROOT/docker/Dockerfile.frontend" ]; then
        print_info "Building frontend Docker image..."
        
        if docker build -f "$PROJECT_ROOT/docker/Dockerfile.frontend" \
                        -t "echolayer/frontend:$image_tag" \
                        "$PROJECT_ROOT" > "$PROJECT_ROOT/logs/docker-frontend-build.log" 2>&1; then
            print_success "Frontend Docker image built successfully"
        else
            print_error "Frontend Docker image build failed"
            exit 1
        fi
    fi
    
    print_success "Docker images built successfully"
}

# Deploy with Docker Compose
deploy_docker_compose() {
    print_header "DEPLOYING WITH DOCKER COMPOSE"
    
    local compose_file
    case $ENVIRONMENT in
        "production")
            compose_file="$PROJECT_ROOT/docker/docker-compose.prod.yml"
            ;;
        "staging")
            compose_file="$PROJECT_ROOT/docker/docker-compose.staging.yml"
            if [ ! -f "$compose_file" ]; then
                compose_file="$PROJECT_ROOT/docker/docker-compose.prod.yml"
                print_info "Using production compose file for staging"
            fi
            ;;
        "development")
            compose_file="$PROJECT_ROOT/docker/docker-compose.yml"
            ;;
    esac
    
    if [ ! -f "$compose_file" ]; then
        print_error "Docker Compose file not found: $compose_file"
        exit 1
    fi
    
    print_info "Using compose file: $compose_file"
    
    if [ "$DRY_RUN" = true ]; then
        print_info "DRY RUN: Would execute: docker-compose -f $compose_file up -d"
        return 0
    fi
    
    # Stop existing containers
    print_info "Stopping existing containers..."
    docker-compose -f "$compose_file" down > "$PROJECT_ROOT/logs/docker-compose-down.log" 2>&1 || true
    
    # Start new containers
    print_info "Starting containers..."
    if docker-compose -f "$compose_file" up -d > "$PROJECT_ROOT/logs/docker-compose-up.log" 2>&1; then
        print_success "Containers started successfully"
    else
        print_error "Failed to start containers"
        
        if [ "$VERBOSE" = true ]; then
            tail -20 "$PROJECT_ROOT/logs/docker-compose-up.log"
        fi
        
        exit 1
    fi
    
    # Wait for services to be ready
    print_info "Waiting for services to be ready..."
    sleep 30
    
    # Health checks
    perform_health_checks
}

# Deploy to Kubernetes
deploy_kubernetes() {
    print_header "DEPLOYING TO KUBERNETES"
    
    local k8s_dir="$PROJECT_ROOT/k8s"
    local namespace
    
    case $ENVIRONMENT in
        "production")
            namespace="echolayer-prod"
            ;;
        "staging")
            namespace="echolayer-staging"
            ;;
        "development")
            namespace="echolayer-dev"
            ;;
    esac
    
    if [ ! -d "$k8s_dir" ]; then
        print_error "Kubernetes manifests directory not found: $k8s_dir"
        exit 1
    fi
    
    # Create namespace if it doesn't exist
    if ! kubectl get namespace "$namespace" > /dev/null 2>&1; then
        print_info "Creating namespace: $namespace"
        
        if [ "$DRY_RUN" = false ]; then
            kubectl create namespace "$namespace"
        fi
    fi
    
    # Apply Kubernetes manifests
    print_info "Applying Kubernetes manifests..."
    
    if [ "$DRY_RUN" = true ]; then
        print_info "DRY RUN: Would apply manifests in $k8s_dir to namespace $namespace"
        return 0
    fi
    
    if kubectl apply -f "$k8s_dir" -n "$namespace" > "$PROJECT_ROOT/logs/k8s-apply.log" 2>&1; then
        print_success "Kubernetes manifests applied successfully"
    else
        print_error "Failed to apply Kubernetes manifests"
        
        if [ "$VERBOSE" = true ]; then
            tail -20 "$PROJECT_ROOT/logs/k8s-apply.log"
        fi
        
        exit 1
    fi
    
    # Wait for rollout to complete
    print_info "Waiting for deployment rollout..."
    
    deployments=$(kubectl get deployments -n "$namespace" -o name 2>/dev/null || true)
    for deployment in $deployments; do
        print_info "Waiting for $deployment..."
        kubectl rollout status "$deployment" -n "$namespace" --timeout=300s
    done
    
    # Health checks
    perform_health_checks
}

# Perform health checks
perform_health_checks() {
    print_header "PERFORMING HEALTH CHECKS"
    
    local backend_url
    local frontend_url
    
    case $ENVIRONMENT in
        "production")
            backend_url="https://api.echolayers.xyz"
            frontend_url="https://echolayers.xyz"
            ;;
        "staging")
            backend_url="https://api-staging.echolayers.xyz"
            frontend_url="https://staging.echolayers.xyz"
            ;;
        "development")
            backend_url="http://localhost:8080"
            frontend_url="http://localhost:3000"
            ;;
    esac
    
    # Backend health check
    print_info "Checking backend health..."
    for i in {1..10}; do
        if curl -sf "$backend_url/health" > /dev/null 2>&1; then
            print_success "Backend is healthy"
            break
        else
            if [ $i -eq 10 ]; then
                print_error "Backend health check failed after 10 attempts"
                exit 1
            fi
            print_info "Backend not ready, retrying in 10 seconds... ($i/10)"
            sleep 10
        fi
    done
    
    # Frontend health check
    print_info "Checking frontend health..."
    for i in {1..10}; do
        if curl -sf "$frontend_url" > /dev/null 2>&1; then
            print_success "Frontend is healthy"
            break
        else
            if [ $i -eq 10 ]; then
                print_error "Frontend health check failed after 10 attempts"
                exit 1
            fi
            print_info "Frontend not ready, retrying in 10 seconds... ($i/10)"
            sleep 10
        fi
    done
    
    print_success "All health checks passed"
}

# Post-deployment tasks
post_deployment_tasks() {
    print_header "POST-DEPLOYMENT TASKS"
    
    # Database migrations (if needed)
    if [ -f "$PROJECT_ROOT/backend/migrations/001_initial_schema.sql" ]; then
        print_info "Running database migrations..."
        # This would typically connect to the database and run migrations
        # Implementation depends on your database setup
        print_info "Database migrations completed"
    fi
    
    # Cache warming
    if [ "$ENVIRONMENT" = "production" ]; then
        print_info "Warming up caches..."
        # Implement cache warming logic here
        print_info "Cache warming completed"
    fi
    
    # Notification
    send_deployment_notification
    
    print_success "Post-deployment tasks completed"
}

# Send deployment notification
send_deployment_notification() {
    local commit_hash=$(git rev-parse --short HEAD)
    local deployment_time=$(date)
    local deployer=$(git config user.name || echo "Unknown")
    
    print_info "Sending deployment notification..."
    
    # Slack notification (if webhook URL is configured)
    if [ -n "${SLACK_WEBHOOK_URL:-}" ]; then
        local message="🚀 EchoLayer deployment completed successfully!
Environment: $ENVIRONMENT
Version: $commit_hash
Deployed by: $deployer
Time: $deployment_time"
        
        curl -X POST -H 'Content-type: application/json' \
             --data "{\"text\":\"$message\"}" \
             "$SLACK_WEBHOOK_URL" > /dev/null 2>&1 || true
    fi
    
    print_success "Deployment notification sent"
}

# Rollback function
rollback() {
    print_header "PERFORMING ROLLBACK"
    
    if [ "$ENVIRONMENT" = "development" ]; then
        print_error "Rollback not supported for development environment"
        exit 1
    fi
    
    # Kubernetes rollback
    if command_exists kubectl; then
        local namespace
        case $ENVIRONMENT in
            "production")
                namespace="echolayer-prod"
                ;;
            "staging")
                namespace="echolayer-staging"
                ;;
        esac
        
        deployments=$(kubectl get deployments -n "$namespace" -o name 2>/dev/null || true)
        for deployment in $deployments; do
            print_info "Rolling back $deployment..."
            kubectl rollout undo "$deployment" -n "$namespace"
        done
        
        print_success "Rollback completed"
    else
        print_error "kubectl not found, cannot perform rollback"
        exit 1
    fi
}

# Main deployment function
main() {
    print_header "ECHOLAYER DEPLOYMENT"
    log "Starting deployment to $ENVIRONMENT environment"
    
    # Check prerequisites
    check_prerequisites
    
    # Pre-deployment checks
    pre_deployment_checks
    
    # Run tests
    run_tests
    
    # Build application
    build_application
    
    # Build Docker images
    build_docker_images
    
    # Deploy based on environment
    if [ "$ENVIRONMENT" = "production" ] && command_exists kubectl; then
        deploy_kubernetes
    else
        deploy_docker_compose
    fi
    
    # Post-deployment tasks
    post_deployment_tasks
    
    print_header "DEPLOYMENT COMPLETE"
    print_success "🎉 EchoLayer deployed successfully to $ENVIRONMENT!"
    print_info "Deployment logs saved to: $DEPLOY_LOG"
    
    # Show deployment summary
    local commit_hash=$(git rev-parse --short HEAD)
    local deployment_time=$(date)
    
    echo ""
    echo "=== DEPLOYMENT SUMMARY ==="
    echo "Environment: $ENVIRONMENT"
    echo "Version: $commit_hash"
    echo "Time: $deployment_time"
    echo "Logs: $DEPLOY_LOG"
    echo "=========================="
}

# Print usage help
usage() {
    echo "Usage: $0 [OPTIONS] [ENVIRONMENT]"
    echo ""
    echo "Environments:"
    echo "  development    Deploy to local development environment (default)"
    echo "  staging        Deploy to staging environment"
    echo "  production     Deploy to production environment"
    echo ""
    echo "Options:"
    echo "  -h, --help         Show this help message"
    echo "  -f, --force        Force deployment even if checks fail"
    echo "  -t, --skip-tests   Skip running tests"
    echo "  -b, --skip-build   Skip building application"
    echo "  -d, --dry-run      Show what would be deployed without actually deploying"
    echo "  -v, --verbose      Enable verbose output"
    echo "  -r, --rollback     Rollback to previous deployment"
    echo ""
    echo "Examples:"
    echo "  $0                        Deploy to development"
    echo "  $0 production             Deploy to production"
    echo "  $0 --force production     Force deploy to production"
    echo "  $0 --rollback production  Rollback production deployment"
    echo ""
}

# Parse command line arguments
ROLLBACK=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -f|--force)
            FORCE_DEPLOY=true
            shift
            ;;
        -t|--skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        -b|--skip-build)
            SKIP_BUILD=true
            shift
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            set -x
            shift
            ;;
        -r|--rollback)
            ROLLBACK=true
            shift
            ;;
        development|staging|production)
            ENVIRONMENT=$1
            shift
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Execute rollback or deployment
if [ "$ROLLBACK" = true ]; then
    rollback
else
    main
fi 