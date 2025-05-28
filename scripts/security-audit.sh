#!/bin/bash

# EchoLayer Security Audit Script
# Description: Comprehensive security audit for EchoLayer platform
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
AUDIT_LOG="$PROJECT_ROOT/logs/security-audit.log"
REPORT_FILE="$PROJECT_ROOT/security-audit-report.md"

# Create logs directory
mkdir -p "$PROJECT_ROOT/logs"

# Logging function
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$AUDIT_LOG"
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

# Initialize audit report
init_report() {
    cat > "$REPORT_FILE" << 'EOF'
# EchoLayer Security Audit Report

**Generated:** $(date)
**Version:** 1.0.0

## Executive Summary

This report contains the results of a comprehensive security audit of the EchoLayer platform.

---

## Audit Results

EOF
}

# ====================================
# DEPENDENCY VULNERABILITY SCANNING
# ====================================
audit_dependencies() {
    print_header "DEPENDENCY VULNERABILITY SCANNING"
    
    local issues_found=0
    
    # Frontend dependencies audit
    if [ -f "$PROJECT_ROOT/frontend/package.json" ]; then
        print_info "Auditing frontend dependencies..."
        cd "$PROJECT_ROOT/frontend"
        
        if command_exists npm; then
            if npm audit --audit-level moderate > "$PROJECT_ROOT/logs/frontend-audit.log" 2>&1; then
                print_success "Frontend dependencies audit passed"
            else
                print_warning "Frontend dependencies have vulnerabilities"
                issues_found=$((issues_found + 1))
                
                # Generate audit fix suggestions
                npm audit fix --dry-run > "$PROJECT_ROOT/logs/frontend-audit-fix.log" 2>&1 || true
            fi
            
            # Check for outdated packages
            npm outdated > "$PROJECT_ROOT/logs/frontend-outdated.log" 2>&1 || true
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Backend dependencies audit (Rust)
    if [ -f "$PROJECT_ROOT/backend/Cargo.toml" ]; then
        print_info "Auditing Rust dependencies..."
        cd "$PROJECT_ROOT/backend"
        
        if command_exists cargo; then
            # Install cargo-audit if not available
            if ! command_exists cargo-audit; then
                print_info "Installing cargo-audit..."
                cargo install cargo-audit
            fi
            
            if cargo audit > "$PROJECT_ROOT/logs/backend-audit.log" 2>&1; then
                print_success "Backend dependencies audit passed"
            else
                print_warning "Backend dependencies have vulnerabilities"
                issues_found=$((issues_found + 1))
            fi
            
            # Check for outdated dependencies
            if command_exists cargo-outdated; then
                cargo outdated > "$PROJECT_ROOT/logs/backend-outdated.log" 2>&1 || true
            else
                print_info "Installing cargo-outdated..."
                cargo install cargo-outdated
                cargo outdated > "$PROJECT_ROOT/logs/backend-outdated.log" 2>&1 || true
            fi
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Smart contract dependencies audit
    if [ -f "$PROJECT_ROOT/smart-contracts/Cargo.toml" ]; then
        print_info "Auditing smart contract dependencies..."
        cd "$PROJECT_ROOT/smart-contracts"
        
        if command_exists cargo; then
            if cargo audit > "$PROJECT_ROOT/logs/contracts-audit.log" 2>&1; then
                print_success "Smart contract dependencies audit passed"
            else
                print_warning "Smart contract dependencies have vulnerabilities"
                issues_found=$((issues_found + 1))
            fi
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Docker image vulnerability scanning
    if command_exists trivy; then
        print_info "Scanning Docker images for vulnerabilities..."
        
        # Scan backend image
        if [ -f "$PROJECT_ROOT/docker/Dockerfile.backend" ]; then
            trivy image --exit-code 1 --severity HIGH,CRITICAL echolayer/backend:latest > "$PROJECT_ROOT/logs/docker-backend-scan.log" 2>&1 || {
                print_warning "Backend Docker image has high/critical vulnerabilities"
                issues_found=$((issues_found + 1))
            }
        fi
        
        # Scan frontend image
        if [ -f "$PROJECT_ROOT/docker/Dockerfile.frontend" ]; then
            trivy image --exit-code 1 --severity HIGH,CRITICAL echolayer/frontend:latest > "$PROJECT_ROOT/logs/docker-frontend-scan.log" 2>&1 || {
                print_warning "Frontend Docker image has high/critical vulnerabilities"
                issues_found=$((issues_found + 1))
            }
        fi
    else
        print_warning "Trivy not installed. Docker vulnerability scanning skipped."
    fi
    
    return $issues_found
}

# ====================================
# SECRET SCANNING
# ====================================
scan_secrets() {
    print_header "SECRET AND CREDENTIAL SCANNING"
    
    local secrets_found=0
    
    # Define patterns for common secrets
    declare -a secret_patterns=(
        "password\s*=\s*['\"][^'\"]+['\"]"
        "api[_-]?key\s*=\s*['\"][^'\"]+['\"]"
        "secret[_-]?key\s*=\s*['\"][^'\"]+['\"]"
        "private[_-]?key\s*=\s*['\"][^'\"]+['\"]"
        "token\s*=\s*['\"][^'\"]+['\"]"
        "access[_-]?token\s*=\s*['\"][^'\"]+['\"]"
        "database[_-]?url\s*=\s*['\"][^'\"]+['\"]"
        "mongodb[_-]?uri\s*=\s*['\"][^'\"]+['\"]"
        "redis[_-]?url\s*=\s*['\"][^'\"]+['\"]"
        "aws[_-]?access[_-]?key"
        "aws[_-]?secret[_-]?key"
        "-----BEGIN\s+(RSA\s+)?PRIVATE\s+KEY-----"
        "sk_live_[0-9a-zA-Z]+"
        "pk_live_[0-9a-zA-Z]+"
    )
    
    print_info "Scanning for hardcoded secrets..."
    
    # Scan source code files
    for pattern in "${secret_patterns[@]}"; do
        if grep -r -i -E "$pattern" \
            --include="*.js" --include="*.jsx" --include="*.ts" --include="*.tsx" \
            --include="*.rs" --include="*.toml" --include="*.json" --include="*.yaml" --include="*.yml" \
            --include="*.env" --include="*.config" \
            --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.git \
            "$PROJECT_ROOT" > "$PROJECT_ROOT/logs/secrets-scan.log" 2>&1; then
            print_warning "Potential secrets found matching pattern: $pattern"
            secrets_found=$((secrets_found + 1))
        fi
    done
    
    # Check for common secret files that shouldn't be committed
    declare -a secret_files=(
        ".env"
        ".env.local"
        ".env.production"
        "secrets.json"
        "private.key"
        "id_rsa"
        "id_dsa"
        "id_ecdsa"
        "id_ed25519"
    )
    
    for file in "${secret_files[@]}"; do
        if find "$PROJECT_ROOT" -name "$file" -not -path "*/node_modules/*" -not -path "*/.git/*" | head -1 | read; then
            print_warning "Sensitive file found: $file"
            secrets_found=$((secrets_found + 1))
        fi
    done
    
    # Check .gitignore for proper secret exclusions
    if [ -f "$PROJECT_ROOT/.gitignore" ]; then
        declare -a required_ignores=(
            "*.env"
            "*.env.local"
            "*.env.production"
            "secrets.*"
            "private.key"
            "*.pem"
            "id_rsa*"
        )
        
        for ignore_pattern in "${required_ignores[@]}"; do
            if ! grep -q "$ignore_pattern" "$PROJECT_ROOT/.gitignore"; then
                print_warning ".gitignore missing pattern: $ignore_pattern"
                secrets_found=$((secrets_found + 1))
            fi
        done
    else
        print_warning ".gitignore file not found"
        secrets_found=$((secrets_found + 1))
    fi
    
    if [ $secrets_found -eq 0 ]; then
        print_success "No secrets found in codebase"
    fi
    
    return $secrets_found
}

# ====================================
# CODE SECURITY ANALYSIS
# ====================================
analyze_code_security() {
    print_header "CODE SECURITY ANALYSIS"
    
    local security_issues=0
    
    # Frontend security checks
    print_info "Analyzing frontend security..."
    
    # Check for dangerous JavaScript patterns
    declare -a js_security_patterns=(
        "eval\s*\("
        "innerHTML\s*="
        "document\.write\s*\("
        "setTimeout\s*\(\s*['\"][^'\"]*['\"]"
        "setInterval\s*\(\s*['\"][^'\"]*['\"]"
        "dangerouslySetInnerHTML"
        "\.html\s*\(\s*.*\)"
        "window\[.*\]\s*\("
    )
    
    for pattern in "${js_security_patterns[@]}"; do
        if grep -r -E "$pattern" \
            --include="*.js" --include="*.jsx" --include="*.ts" --include="*.tsx" \
            --exclude-dir=node_modules --exclude-dir=.next \
            "$PROJECT_ROOT/frontend" > /dev/null 2>&1; then
            print_warning "Potentially unsafe JavaScript pattern found: $pattern"
            security_issues=$((security_issues + 1))
        fi
    done
    
    # Backend security checks
    print_info "Analyzing backend security..."
    
    # Check for Rust security anti-patterns
    declare -a rust_security_patterns=(
        "unsafe\s*{"
        "transmute"
        "from_raw"
        "as_ptr\(\)\.offset"
        "unwrap\(\)\s*;"
        "expect\(\s*\"\s*\"\s*\)"
        "panic!\(\s*\)"
    )
    
    for pattern in "${rust_security_patterns[@]}"; do
        if grep -r -E "$pattern" \
            --include="*.rs" \
            --exclude-dir=target \
            "$PROJECT_ROOT/backend" > /dev/null 2>&1; then
            print_warning "Potentially unsafe Rust pattern found: $pattern"
            security_issues=$((security_issues + 1))
        fi
    done
    
    # Check for SQL injection patterns
    if grep -r -E "(format!|write!|writeln!)\s*\(\s*\".*\{.*\}.*\".*user.*input" \
        --include="*.rs" \
        --exclude-dir=target \
        "$PROJECT_ROOT/backend" > /dev/null 2>&1; then
        print_warning "Potential SQL injection vulnerability found"
        security_issues=$((security_issues + 1))
    fi
    
    # Smart contract security checks
    if [ -d "$PROJECT_ROOT/smart-contracts" ]; then
        print_info "Analyzing smart contract security..."
        
        # Check for common Solana program vulnerabilities
        declare -a solana_security_patterns=(
            "unchecked.*arithmetic"
            "\.unwrap\(\)\s*;"
            "from.*without.*validation"
            "direct.*transfer.*without.*checks"
        )
        
        for pattern in "${solana_security_patterns[@]}"; do
            if grep -r -E "$pattern" \
                --include="*.rs" \
                --exclude-dir=target \
                "$PROJECT_ROOT/smart-contracts" > /dev/null 2>&1; then
                print_warning "Potential smart contract vulnerability: $pattern"
                security_issues=$((security_issues + 1))
            fi
        done
    fi
    
    if [ $security_issues -eq 0 ]; then
        print_success "No obvious security issues found in code analysis"
    fi
    
    return $security_issues
}

# ====================================
# CONFIGURATION SECURITY AUDIT
# ====================================
audit_configuration() {
    print_header "CONFIGURATION SECURITY AUDIT"
    
    local config_issues=0
    
    # Docker security checks
    if [ -d "$PROJECT_ROOT/docker" ]; then
        print_info "Auditing Docker configurations..."
        
        # Check for non-root user in Dockerfiles
        find "$PROJECT_ROOT/docker" -name "Dockerfile*" | while read dockerfile; do
            if ! grep -q "USER" "$dockerfile"; then
                print_warning "Dockerfile missing USER directive: $(basename $dockerfile)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for COPY --chown usage
            if grep -q "COPY.*--chown" "$dockerfile"; then
                print_success "Proper file ownership in: $(basename $dockerfile)"
            else
                print_warning "Consider using COPY --chown in: $(basename $dockerfile)"
            fi
        done
        
        # Check docker-compose security
        find "$PROJECT_ROOT/docker" -name "docker-compose*.yml" | while read compose_file; do
            # Check for privileged mode
            if grep -q "privileged.*true" "$compose_file"; then
                print_warning "Privileged mode detected in: $(basename $compose_file)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for host network mode
            if grep -q "network_mode.*host" "$compose_file"; then
                print_warning "Host network mode detected in: $(basename $compose_file)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for volume mounts
            if grep -q "/:/.*" "$compose_file"; then
                print_warning "Root filesystem mount detected in: $(basename $compose_file)"
                config_issues=$((config_issues + 1))
            fi
        done
    fi
    
    # Kubernetes security checks
    if [ -d "$PROJECT_ROOT/k8s" ]; then
        print_info "Auditing Kubernetes configurations..."
        
        find "$PROJECT_ROOT/k8s" -name "*.yaml" -o -name "*.yml" | while read k8s_file; do
            # Check for privileged containers
            if grep -q "privileged.*true" "$k8s_file"; then
                print_warning "Privileged container in: $(basename $k8s_file)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for hostNetwork
            if grep -q "hostNetwork.*true" "$k8s_file"; then
                print_warning "Host network in: $(basename $k8s_file)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for runAsRoot
            if grep -q "runAsUser.*0" "$k8s_file"; then
                print_warning "Running as root in: $(basename $k8s_file)"
                config_issues=$((config_issues + 1))
            fi
            
            # Check for security context
            if ! grep -q "securityContext" "$k8s_file"; then
                print_warning "Missing security context in: $(basename $k8s_file)"
                config_issues=$((config_issues + 1))
            fi
        done
    fi
    
    # Environment configuration checks
    print_info "Checking environment configurations..."
    
    # Check for example environment files
    find "$PROJECT_ROOT" -name ".env.example" -o -name ".env.template" | while read env_file; do
        if grep -q -E "(password|secret|key).*=.*[^=]" "$env_file"; then
            print_warning "Example environment file contains actual values: $(basename $env_file)"
            config_issues=$((config_issues + 1))
        fi
    done
    
    if [ $config_issues -eq 0 ]; then
        print_success "Configuration security audit passed"
    fi
    
    return $config_issues
}

# ====================================
# NETWORK SECURITY CHECKS
# ====================================
check_network_security() {
    print_header "NETWORK SECURITY ANALYSIS"
    
    local network_issues=0
    
    # Check for exposed ports in configurations
    print_info "Checking for exposed services..."
    
    # Docker port exposures
    if find "$PROJECT_ROOT" -name "docker-compose*.yml" | head -1 | read; then
        if grep -r "ports:" "$PROJECT_ROOT/docker/" | grep -E ":[0-9]+:[0-9]+" > "$PROJECT_ROOT/logs/exposed-ports.log"; then
            print_warning "Services exposing ports to host. Review exposed-ports.log"
            network_issues=$((network_issues + 1))
        fi
    fi
    
    # Check for CORS configurations
    print_info "Checking CORS configurations..."
    
    if grep -r -i "cors" --include="*.js" --include="*.ts" --include="*.rs" "$PROJECT_ROOT" | grep -i "origin.*\*" > /dev/null; then
        print_warning "Wildcard CORS origin detected - may be unsafe for production"
        network_issues=$((network_issues + 1))
    fi
    
    # Check for HTTPS enforcement
    print_info "Checking HTTPS configurations..."
    
    if ! grep -r -i "https" --include="*.js" --include="*.ts" --include="*.rs" --include="*.yml" "$PROJECT_ROOT" > /dev/null; then
        print_warning "No HTTPS configurations found"
        network_issues=$((network_issues + 1))
    fi
    
    if [ $network_issues -eq 0 ]; then
        print_success "Network security checks passed"
    fi
    
    return $network_issues
}

# ====================================
# GENERATE SECURITY REPORT
# ====================================
generate_report() {
    print_header "GENERATING SECURITY AUDIT REPORT"
    
    local total_issues=$1
    
    cat >> "$REPORT_FILE" << EOF

## Summary

- **Total Issues Found:** $total_issues
- **Audit Date:** $(date)
- **Status:** $([ $total_issues -eq 0 ] && echo "✅ PASSED" || echo "⚠️ ISSUES FOUND")

## Detailed Findings

### Dependency Vulnerabilities
$([ -f "$PROJECT_ROOT/logs/frontend-audit.log" ] && echo "- Frontend: See logs/frontend-audit.log" || echo "- Frontend: No issues")
$([ -f "$PROJECT_ROOT/logs/backend-audit.log" ] && echo "- Backend: See logs/backend-audit.log" || echo "- Backend: No issues")
$([ -f "$PROJECT_ROOT/logs/contracts-audit.log" ] && echo "- Smart Contracts: See logs/contracts-audit.log" || echo "- Smart Contracts: No issues")

### Secret Scanning
$([ -f "$PROJECT_ROOT/logs/secrets-scan.log" ] && echo "- Potential secrets: See logs/secrets-scan.log" || echo "- No secrets detected")

### Code Security
$([ $total_issues -gt 0 ] && echo "- Security patterns detected - review recommendations" || echo "- No security anti-patterns found")

### Configuration Security
$([ $total_issues -gt 0 ] && echo "- Configuration issues found - see audit log" || echo "- Configurations are secure")

## Recommendations

1. **High Priority:**
   - Fix any HIGH or CRITICAL vulnerabilities
   - Remove hardcoded secrets
   - Enable HTTPS everywhere

2. **Medium Priority:**
   - Update outdated dependencies
   - Review CORS configurations
   - Implement security headers

3. **Low Priority:**
   - Add security contexts to Kubernetes configs
   - Review Docker security best practices
   - Implement additional monitoring

## Next Steps

1. Review detailed logs in the \`logs/\` directory
2. Fix high-priority issues first
3. Re-run security audit after fixes
4. Schedule regular security audits

---

**Generated by EchoLayer Security Audit v1.0.0**
EOF
    
    print_success "Security audit report generated: $REPORT_FILE"
}

# ====================================
# MAIN EXECUTION
# ====================================
main() {
    print_header "ECHOLAYER SECURITY AUDIT"
    log "Starting comprehensive security audit"
    
    # Initialize report
    init_report
    
    # Run all security checks
    local total_issues=0
    
    print_info "Running dependency vulnerability scanning..."
    audit_dependencies || total_issues=$((total_issues + $?))
    
    print_info "Running secret scanning..."
    scan_secrets || total_issues=$((total_issues + $?))
    
    print_info "Running code security analysis..."
    analyze_code_security || total_issues=$((total_issues + $?))
    
    print_info "Running configuration security audit..."
    audit_configuration || total_issues=$((total_issues + $?))
    
    print_info "Running network security checks..."
    check_network_security || total_issues=$((total_issues + $?))
    
    # Generate final report
    generate_report $total_issues
    
    print_header "SECURITY AUDIT COMPLETE"
    
    if [ $total_issues -eq 0 ]; then
        print_success "🎉 Security audit completed successfully! No issues found."
        exit 0
    else
        print_warning "⚠️  Security audit completed with $total_issues issues found."
        print_info "Please review the report at: $REPORT_FILE"
        print_info "Check detailed logs in: $PROJECT_ROOT/logs/"
        exit 1
    fi
}

# Print usage help
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help      Show this help message"
    echo "  -v, --verbose   Enable verbose output"
    echo "  -q, --quiet     Suppress output except errors"
    echo ""
    echo "Examples:"
    echo "  $0              Run complete security audit"
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
        -v|--verbose)
            set -x
            shift
            ;;
        -q|--quiet)
            exec > /dev/null 2>&1
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