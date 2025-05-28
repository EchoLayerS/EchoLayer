#!/bin/bash

# EchoLayer Code Quality Check Script
# Description: Comprehensive code quality analysis and formatting
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
QUALITY_LOG="$PROJECT_ROOT/logs/code-quality.log"
REPORT_FILE="$PROJECT_ROOT/code-quality-report.md"

# Default options
FIX_ISSUES=false
VERBOSE=false
CHECK_ONLY=false

# Create logs directory
mkdir -p "$PROJECT_ROOT/logs"

# Logging function
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$QUALITY_LOG"
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

# Initialize quality report
init_report() {
    cat > "$REPORT_FILE" << EOF
# EchoLayer Code Quality Report

**Generated:** $(date)
**Version:** 1.0.0

## Overview

This report contains the results of comprehensive code quality analysis for the EchoLayer platform.

---

## Quality Metrics

EOF
}

# ====================================
# FRONTEND CODE QUALITY
# ====================================
check_frontend_quality() {
    print_header "FRONTEND CODE QUALITY ANALYSIS"
    
    local issues_found=0
    
    if [ ! -d "$PROJECT_ROOT/frontend" ]; then
        print_warning "Frontend directory not found"
        return 0
    fi
    
    cd "$PROJECT_ROOT/frontend"
    
    # Check if package.json exists
    if [ ! -f "package.json" ]; then
        print_warning "Frontend package.json not found"
        cd "$PROJECT_ROOT"
        return 1
    fi
    
    # Install dependencies if node_modules doesn't exist
    if [ ! -d "node_modules" ]; then
        print_info "Installing frontend dependencies..."
        npm install
    fi
    
    # TypeScript compilation check
    print_info "Checking TypeScript compilation..."
    if command_exists tsc; then
        if tsc --noEmit > "$PROJECT_ROOT/logs/frontend-tsc.log" 2>&1; then
            print_success "TypeScript compilation successful"
        else
            print_error "TypeScript compilation failed"
            issues_found=$((issues_found + 1))
            
            if [ "$VERBOSE" = true ]; then
                cat "$PROJECT_ROOT/logs/frontend-tsc.log"
            fi
        fi
    else
        print_warning "TypeScript compiler not found"
    fi
    
    # ESLint check
    print_info "Running ESLint analysis..."
    if command_exists eslint || [ -f "node_modules/.bin/eslint" ]; then
        local eslint_cmd="npx eslint"
        local eslint_args="src --ext .ts,.tsx,.js,.jsx"
        
        if [ "$FIX_ISSUES" = true ]; then
            eslint_args="$eslint_args --fix"
            print_info "Auto-fixing ESLint issues..."
        fi
        
        if $eslint_cmd $eslint_args --format json > "$PROJECT_ROOT/logs/frontend-eslint.json" 2>&1; then
            print_success "ESLint analysis passed"
        else
            print_warning "ESLint found issues"
            issues_found=$((issues_found + 1))
            
            # Parse ESLint results
            if command_exists jq && [ -f "$PROJECT_ROOT/logs/frontend-eslint.json" ]; then
                local error_count=$(jq '[.[].messages[] | select(.severity == 2)] | length' "$PROJECT_ROOT/logs/frontend-eslint.json" 2>/dev/null || echo "0")
                local warning_count=$(jq '[.[].messages[] | select(.severity == 1)] | length' "$PROJECT_ROOT/logs/frontend-eslint.json" 2>/dev/null || echo "0")
                
                print_info "ESLint results: $error_count errors, $warning_count warnings"
            fi
        fi
    else
        print_warning "ESLint not found"
    fi
    
    # Prettier formatting check
    print_info "Checking Prettier formatting..."
    if command_exists prettier || [ -f "node_modules/.bin/prettier" ]; then
        local prettier_cmd="npx prettier"
        local prettier_args="src --check"
        
        if [ "$FIX_ISSUES" = true ]; then
            prettier_args="src --write"
            print_info "Auto-formatting with Prettier..."
        fi
        
        if $prettier_cmd $prettier_args > "$PROJECT_ROOT/logs/frontend-prettier.log" 2>&1; then
            print_success "Code formatting is consistent"
        else
            print_warning "Code formatting issues found"
            issues_found=$((issues_found + 1))
        fi
    else
        print_warning "Prettier not found"
    fi
    
    # Bundle size analysis
    print_info "Analyzing bundle size..."
    if [ -f "next.config.js" ] && command_exists npm; then
        if ANALYZE=true npm run build > "$PROJECT_ROOT/logs/frontend-bundle.log" 2>&1; then
            print_success "Bundle analysis completed"
        else
            print_warning "Bundle analysis failed"
        fi
    fi
    
    # Jest tests
    print_info "Running Jest tests..."
    if command_exists jest || [ -f "node_modules/.bin/jest" ]; then
        if npm test -- --coverage --passWithNoTests > "$PROJECT_ROOT/logs/frontend-tests.log" 2>&1; then
            print_success "All tests passed"
        else
            print_warning "Some tests failed"
            issues_found=$((issues_found + 1))
        fi
    else
        print_warning "Jest not found"
    fi
    
    cd "$PROJECT_ROOT"
    return $issues_found
}

# ====================================
# BACKEND CODE QUALITY
# ====================================
check_backend_quality() {
    print_header "BACKEND CODE QUALITY ANALYSIS"
    
    local issues_found=0
    
    if [ ! -d "$PROJECT_ROOT/backend" ]; then
        print_warning "Backend directory not found"
        return 0
    fi
    
    cd "$PROJECT_ROOT/backend"
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_warning "Backend Cargo.toml not found"
        cd "$PROJECT_ROOT"
        return 1
    fi
    
    # Rust compilation check
    print_info "Checking Rust compilation..."
    if command_exists cargo; then
        if cargo check > "$PROJECT_ROOT/logs/backend-check.log" 2>&1; then
            print_success "Rust compilation successful"
        else
            print_error "Rust compilation failed"
            issues_found=$((issues_found + 1))
            
            if [ "$VERBOSE" = true ]; then
                cat "$PROJECT_ROOT/logs/backend-check.log"
            fi
        fi
    else
        print_error "Cargo not found"
        cd "$PROJECT_ROOT"
        return 1
    fi
    
    # Clippy linting
    print_info "Running Clippy analysis..."
    if cargo clippy --all-targets --all-features -- -D warnings > "$PROJECT_ROOT/logs/backend-clippy.log" 2>&1; then
        print_success "Clippy analysis passed"
    else
        print_warning "Clippy found issues"
        issues_found=$((issues_found + 1))
        
        if [ "$VERBOSE" = true ]; then
            cat "$PROJECT_ROOT/logs/backend-clippy.log"
        fi
    fi
    
    # Rustfmt formatting check
    print_info "Checking Rust formatting..."
    if [ "$FIX_ISSUES" = true ]; then
        print_info "Auto-formatting Rust code..."
        if cargo fmt > "$PROJECT_ROOT/logs/backend-fmt.log" 2>&1; then
            print_success "Code formatted successfully"
        else
            print_warning "Formatting failed"
        fi
    else
        if cargo fmt -- --check > "$PROJECT_ROOT/logs/backend-fmt.log" 2>&1; then
            print_success "Code formatting is consistent"
        else
            print_warning "Code formatting issues found"
            issues_found=$((issues_found + 1))
        fi
    fi
    
    # Cargo tests
    print_info "Running Cargo tests..."
    if cargo test > "$PROJECT_ROOT/logs/backend-tests.log" 2>&1; then
        print_success "All tests passed"
    else
        print_warning "Some tests failed"
        issues_found=$((issues_found + 1))
        
        if [ "$VERBOSE" = true ]; then
            tail -20 "$PROJECT_ROOT/logs/backend-tests.log"
        fi
    fi
    
    # Cargo bench (if available)
    print_info "Running benchmarks..."
    if cargo bench --no-run > /dev/null 2>&1; then
        cargo bench > "$PROJECT_ROOT/logs/backend-bench.log" 2>&1 || true
        print_success "Benchmarks completed"
    else
        print_info "No benchmarks found"
    fi
    
    # Code coverage (if tarpaulin is available)
    if command_exists cargo-tarpaulin; then
        print_info "Generating code coverage..."
        if cargo tarpaulin --out Html --output-dir "$PROJECT_ROOT/logs" > "$PROJECT_ROOT/logs/backend-coverage.log" 2>&1; then
            print_success "Code coverage generated"
        else
            print_warning "Code coverage generation failed"
        fi
    else
        print_info "cargo-tarpaulin not found, skipping coverage"
    fi
    
    cd "$PROJECT_ROOT"
    return $issues_found
}

# ====================================
# SMART CONTRACT QUALITY
# ====================================
check_smart_contract_quality() {
    print_header "SMART CONTRACT CODE QUALITY"
    
    local issues_found=0
    
    if [ ! -d "$PROJECT_ROOT/smart-contracts" ]; then
        print_warning "Smart contracts directory not found"
        return 0
    fi
    
    cd "$PROJECT_ROOT/smart-contracts"
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        print_warning "Smart contracts Cargo.toml not found"
        cd "$PROJECT_ROOT"
        return 0
    fi
    
    # Rust compilation check
    print_info "Checking smart contract compilation..."
    if command_exists cargo; then
        if cargo check > "$PROJECT_ROOT/logs/contracts-check.log" 2>&1; then
            print_success "Smart contract compilation successful"
        else
            print_error "Smart contract compilation failed"
            issues_found=$((issues_found + 1))
        fi
    fi
    
    # Clippy for smart contracts (with specific rules)
    print_info "Running Clippy for smart contracts..."
    if cargo clippy --all-targets --all-features -- -D warnings -A clippy::result_large_err > "$PROJECT_ROOT/logs/contracts-clippy.log" 2>&1; then
        print_success "Smart contract linting passed"
    else
        print_warning "Smart contract linting found issues"
        issues_found=$((issues_found + 1))
    fi
    
    # Smart contract specific security checks
    print_info "Running smart contract security checks..."
    
    # Check for common Solana vulnerabilities
    declare -a solana_patterns=(
        "\.unwrap\(\)" 
        "panic!" 
        "expect\(\"\"\)"
        "unsafe"
    )
    
    for pattern in "${solana_patterns[@]}"; do
        if grep -r "$pattern" src/ > /dev/null 2>&1; then
            print_warning "Potentially unsafe pattern found: $pattern"
            issues_found=$((issues_found + 1))
        fi
    done
    
    # Anchor specific checks (if using Anchor framework)
    if grep -q "anchor-lang" Cargo.toml; then
        print_info "Detected Anchor framework, running Anchor-specific checks..."
        
        # Check for proper error handling
        if ! grep -r "AnchorError\|ErrorCode" src/ > /dev/null 2>&1; then
            print_warning "Consider using Anchor error handling patterns"
        fi
        
        # Check for account validation
        if ! grep -r "#\[account" src/ > /dev/null 2>&1; then
            print_warning "Ensure proper account validation with #[account] constraints"
        fi
    fi
    
    # Test smart contracts
    print_info "Running smart contract tests..."
    if cargo test > "$PROJECT_ROOT/logs/contracts-tests.log" 2>&1; then
        print_success "Smart contract tests passed"
    else
        print_warning "Smart contract tests failed"
        issues_found=$((issues_found + 1))
    fi
    
    cd "$PROJECT_ROOT"
    return $issues_found
}

# ====================================
# DOCKER QUALITY CHECKS
# ====================================
check_docker_quality() {
    print_header "DOCKER CONFIGURATION QUALITY"
    
    local issues_found=0
    
    if [ ! -d "$PROJECT_ROOT/docker" ]; then
        print_warning "Docker directory not found"
        return 0
    fi
    
    # Dockerfile linting with hadolint
    if command_exists hadolint; then
        print_info "Running Hadolint on Dockerfiles..."
        
        find "$PROJECT_ROOT/docker" -name "Dockerfile*" | while read dockerfile; do
            if hadolint "$dockerfile" > "$PROJECT_ROOT/logs/docker-$(basename $dockerfile).log" 2>&1; then
                print_success "Dockerfile linting passed: $(basename $dockerfile)"
            else
                print_warning "Dockerfile issues found: $(basename $dockerfile)"
                issues_found=$((issues_found + 1))
            fi
        done
    else
        print_warning "Hadolint not found, skipping Dockerfile linting"
    fi
    
    # Docker-compose validation
    if command_exists docker-compose; then
        find "$PROJECT_ROOT/docker" -name "docker-compose*.yml" | while read compose_file; do
            print_info "Validating docker-compose file: $(basename $compose_file)"
            
            if docker-compose -f "$compose_file" config > /dev/null 2>&1; then
                print_success "Docker-compose validation passed: $(basename $compose_file)"
            else
                print_warning "Docker-compose validation failed: $(basename $compose_file)"
                issues_found=$((issues_found + 1))
            fi
        done
    fi
    
    return $issues_found
}

# ====================================
# DOCUMENTATION QUALITY
# ====================================
check_documentation_quality() {
    print_header "DOCUMENTATION QUALITY CHECK"
    
    local issues_found=0
    
    # Check for required documentation files
    declare -a required_docs=(
        "README.md"
        "docs/ARCHITECTURE.md"
        "docs/DEPLOYMENT.md"
        "docs/API_DOCUMENTATION.md"
    )
    
    for doc in "${required_docs[@]}"; do
        if [ -f "$PROJECT_ROOT/$doc" ]; then
            print_success "Documentation found: $doc"
            
            # Check if file is not empty
            if [ ! -s "$PROJECT_ROOT/$doc" ]; then
                print_warning "Documentation file is empty: $doc"
                issues_found=$((issues_found + 1))
            fi
        else
            print_warning "Missing documentation: $doc"
            issues_found=$((issues_found + 1))
        fi
    done
    
    # Check for code documentation
    print_info "Checking code documentation coverage..."
    
    # Rust documentation
    if [ -d "$PROJECT_ROOT/backend" ]; then
        cd "$PROJECT_ROOT/backend"
        if command_exists cargo; then
            if cargo doc --no-deps > "$PROJECT_ROOT/logs/backend-docs.log" 2>&1; then
                print_success "Rust documentation generated successfully"
            else
                print_warning "Rust documentation generation failed"
                issues_found=$((issues_found + 1))
            fi
        fi
        cd "$PROJECT_ROOT"
    fi
    
    # TypeScript documentation (if typedoc is available)
    if [ -d "$PROJECT_ROOT/frontend" ]; then
        cd "$PROJECT_ROOT/frontend"
        if command_exists typedoc || [ -f "node_modules/.bin/typedoc" ]; then
            if npx typedoc src --out "$PROJECT_ROOT/logs/frontend-docs" > "$PROJECT_ROOT/logs/frontend-docs.log" 2>&1; then
                print_success "TypeScript documentation generated successfully"
            else
                print_warning "TypeScript documentation generation failed"
                issues_found=$((issues_found + 1))
            fi
        fi
        cd "$PROJECT_ROOT"
    fi
    
    return $issues_found
}

# ====================================
# GENERATE QUALITY REPORT
# ====================================
generate_report() {
    print_header "GENERATING CODE QUALITY REPORT"
    
    local total_issues=$1
    local frontend_issues=$2
    local backend_issues=$3
    local contract_issues=$4
    local docker_issues=$5
    local doc_issues=$6
    
    cat >> "$REPORT_FILE" << EOF

## Summary

- **Total Issues Found:** $total_issues
- **Report Date:** $(date)
- **Status:** $([ $total_issues -eq 0 ] && echo "✅ EXCELLENT" || echo "⚠️ NEEDS IMPROVEMENT")

## Component Analysis

### Frontend Quality
- **Issues Found:** $frontend_issues
- **Status:** $([ $frontend_issues -eq 0 ] && echo "✅ GOOD" || echo "⚠️ NEEDS WORK")
- **Logs:** frontend-*.log

### Backend Quality  
- **Issues Found:** $backend_issues
- **Status:** $([ $backend_issues -eq 0 ] && echo "✅ GOOD" || echo "⚠️ NEEDS WORK")
- **Logs:** backend-*.log

### Smart Contract Quality
- **Issues Found:** $contract_issues  
- **Status:** $([ $contract_issues -eq 0 ] && echo "✅ GOOD" || echo "⚠️ NEEDS WORK")
- **Logs:** contracts-*.log

### Docker Quality
- **Issues Found:** $docker_issues
- **Status:** $([ $docker_issues -eq 0 ] && echo "✅ GOOD" || echo "⚠️ NEEDS WORK")
- **Logs:** docker-*.log

### Documentation Quality
- **Issues Found:** $doc_issues
- **Status:** $([ $doc_issues -eq 0 ] && echo "✅ GOOD" || echo "⚠️ NEEDS WORK")

## Recommendations

$([ $total_issues -gt 0 ] && cat << 'RECS'
### High Priority
1. Fix compilation errors and test failures
2. Address security-related linting issues
3. Ensure all critical documentation exists

### Medium Priority  
1. Fix formatting and style issues
2. Improve code coverage
3. Add missing documentation

### Low Priority
1. Optimize bundle sizes
2. Improve code organization
3. Add more comprehensive tests
RECS
)

$([ $total_issues -eq 0 ] && cat << 'GOOD'
### Excellent Code Quality! 🎉
- All checks passed successfully
- Code follows best practices
- Documentation is comprehensive
- Consider running regular quality checks
GOOD
)

## Automated Fixes

To automatically fix common issues, run:
\`\`\`bash
./scripts/code-quality-check.sh --fix
\`\`\`

---

**Generated by EchoLayer Code Quality Checker v1.0.0**
EOF
    
    print_success "Code quality report generated: $REPORT_FILE"
}

# ====================================
# MAIN EXECUTION
# ====================================
main() {
    print_header "ECHOLAYER CODE QUALITY CHECK"
    log "Starting comprehensive code quality analysis"
    
    # Initialize report
    init_report
    
    # Run all quality checks
    local total_issues=0
    local frontend_issues=0
    local backend_issues=0
    local contract_issues=0
    local docker_issues=0
    local doc_issues=0
    
    print_info "Running frontend quality checks..."
    check_frontend_quality || frontend_issues=$?
    total_issues=$((total_issues + frontend_issues))
    
    print_info "Running backend quality checks..."
    check_backend_quality || backend_issues=$?
    total_issues=$((total_issues + backend_issues))
    
    print_info "Running smart contract quality checks..."
    check_smart_contract_quality || contract_issues=$?
    total_issues=$((total_issues + contract_issues))
    
    print_info "Running Docker quality checks..."
    check_docker_quality || docker_issues=$?
    total_issues=$((total_issues + docker_issues))
    
    print_info "Running documentation quality checks..."
    check_documentation_quality || doc_issues=$?
    total_issues=$((total_issues + doc_issues))
    
    # Generate final report
    generate_report $total_issues $frontend_issues $backend_issues $contract_issues $docker_issues $doc_issues
    
    print_header "CODE QUALITY CHECK COMPLETE"
    
    if [ $total_issues -eq 0 ]; then
        print_success "🎉 Excellent code quality! All checks passed."
        exit 0
    else
        print_warning "⚠️  Code quality check completed with $total_issues issues found."
        print_info "Please review the report at: $REPORT_FILE"
        print_info "Check detailed logs in: $PROJECT_ROOT/logs/"
        
        if [ "$CHECK_ONLY" = false ]; then
            print_info "Run with --fix to automatically fix common issues"
        fi
        
        exit 1
    fi
}

# Print usage help
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help      Show this help message"
    echo "  -f, --fix       Automatically fix issues where possible"
    echo "  -v, --verbose   Enable verbose output"
    echo "  -c, --check     Check only, don't suggest fixes"
    echo ""
    echo "Examples:"
    echo "  $0              Run complete quality check"
    echo "  $0 --fix        Run check and auto-fix issues"
    echo "  $0 --verbose    Run with detailed output"
    echo ""
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -f|--fix)
            FIX_ISSUES=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            set -x
            shift
            ;;
        -c|--check)
            CHECK_ONLY=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Run main function
main "$@" 