#!/bin/bash

# ZedLite Mode Switcher
# Switch between Full IDE mode and Text-Only mode for novel writing

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

FULL_CARGO="Cargo.toml"
TEXT_CARGO="Cargo-text-only.toml"
BACKUP_FULL="Cargo-full.toml"
BACKUP_TEXT="Cargo-text-only.backup.toml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}  ZedLite Mode Switcher${NC}"
    echo -e "${BLUE}============================================${NC}"
}

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

get_current_mode() {
    if [[ -f "$FULL_CARGO" ]]; then
        # Check if it's the text-only version
        if grep -q "TEXT-ONLY BUILD: Novel Writing Tool" "$FULL_CARGO" 2>/dev/null; then
            echo "text-only"
        else
            echo "full-ide"
        fi
    else
        echo "unknown"
    fi
}

count_crates() {
    if [[ -f "$FULL_CARGO" ]]; then
        grep -c "\"crates/" "$FULL_CARGO" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

show_status() {
    print_header
    
    local current_mode=$(get_current_mode)
    local crate_count=$(count_crates)
    
    echo -e "Current Mode: ${BLUE}$current_mode${NC}"
    echo -e "Workspace Crates: ${BLUE}$crate_count${NC}"
    echo ""
    
    case $current_mode in
        "full-ide")
            echo -e "${GREEN}Full IDE Mode${NC} - Complete programming environment"
            echo "• All coding features enabled (LSP, Git, Debugger, etc.)"
            echo "• All ~150 crates included"
            echo "• Larger binary size and longer compile times"
            echo "• Complete novel writing functionality"
            ;;
        "text-only")
            echo -e "${GREEN}Text-Only Mode${NC} - Focused novel writing tool"
            echo "• Coding features removed (LSP, Git, Debugger, etc.)"
            echo "• Only ~60 essential crates included"
            echo "• Smaller binary size and faster compile times"
            echo "• Complete novel writing functionality"
            ;;
        "unknown")
            print_error "Cannot determine current mode (Cargo.toml missing?)"
            ;;
    esac
    
    echo ""
    echo "Available files:"
    [[ -f "$FULL_CARGO" ]] && echo "• $FULL_CARGO (active)"
    [[ -f "$TEXT_CARGO" ]] && echo "• $TEXT_CARGO"
    [[ -f "$BACKUP_FULL" ]] && echo "• $BACKUP_FULL"
    [[ -f "$BACKUP_TEXT" ]] && echo "• $BACKUP_TEXT"
}

switch_to_text_only() {
    print_header
    echo "Switching to Text-Only Mode..."
    echo ""
    
    # Verify text-only config exists
    if [[ ! -f "$TEXT_CARGO" ]]; then
        print_error "Text-only configuration not found: $TEXT_CARGO"
        echo "Please ensure Cargo-text-only.toml exists in the project root."
        exit 1
    fi
    
    local current_mode=$(get_current_mode)
    if [[ "$current_mode" == "text-only" ]]; then
        print_warning "Already in text-only mode"
        return 0
    fi
    
    # Backup current full version
    if [[ -f "$FULL_CARGO" ]] && [[ "$current_mode" == "full-ide" ]]; then
        print_status "Backing up full IDE configuration to $BACKUP_FULL"
        cp "$FULL_CARGO" "$BACKUP_FULL"
    fi
    
    # Switch to text-only
    print_status "Activating text-only configuration"
    cp "$TEXT_CARGO" "$FULL_CARGO"
    
    # Clean build artifacts
    print_status "Cleaning build artifacts"
    cargo clean --quiet
    
    print_status "Successfully switched to text-only mode"
    echo ""
    echo -e "${GREEN}Text-Only Mode Active${NC}"
    echo "• ~60 crates (reduced from ~150)"
    echo "• Faster compilation and smaller binary"
    echo "• All novel writing features preserved"
    echo "• Coding features disabled"
    echo ""
    echo "Build with: ${YELLOW}cargo build --release${NC}"
}

switch_to_full_ide() {
    print_header
    echo "Switching to Full IDE Mode..."
    echo ""
    
    local current_mode=$(get_current_mode)
    if [[ "$current_mode" == "full-ide" ]]; then
        print_warning "Already in full IDE mode"
        return 0
    fi
    
    # Backup current text-only version
    if [[ -f "$FULL_CARGO" ]] && [[ "$current_mode" == "text-only" ]]; then
        print_status "Backing up text-only configuration to $BACKUP_TEXT"
        cp "$FULL_CARGO" "$BACKUP_TEXT"
    fi
    
    # Restore full IDE version
    if [[ -f "$BACKUP_FULL" ]]; then
        print_status "Restoring full IDE configuration from backup"
        cp "$BACKUP_FULL" "$FULL_CARGO"
    else
        print_error "Full IDE backup not found: $BACKUP_FULL"
        echo "Cannot restore full IDE mode without backup."
        exit 1
    fi
    
    # Clean build artifacts
    print_status "Cleaning build artifacts"
    cargo clean --quiet
    
    print_status "Successfully switched to full IDE mode"
    echo ""
    echo -e "${GREEN}Full IDE Mode Active${NC}"
    echo "• ~150 crates (complete feature set)"
    echo "• All coding and novel writing features"
    echo "• Larger binary and longer compile times"
    echo ""
    echo "Build with: ${YELLOW}cargo build --release${NC}"
}

test_build() {
    local mode="$1"
    print_header
    echo "Testing $mode build..."
    echo ""
    
    print_status "Running cargo check"
    if cargo check --quiet; then
        print_status "✓ Cargo check passed"
    else
        print_error "✗ Cargo check failed"
        return 1
    fi
    
    print_status "Analyzing workspace"
    local crate_count=$(count_crates)
    echo "• Workspace contains $crate_count crates"
    
    if command -v jq &> /dev/null; then
        local metadata_count=$(cargo metadata --format-version=1 2>/dev/null | jq '.workspace_members | length' 2>/dev/null || echo "unknown")
        echo "• Cargo metadata reports $metadata_count members"
    fi
    
    print_status "$mode build test completed successfully"
}

show_help() {
    print_header
    cat << EOF
USAGE:
    $0 <command>

COMMANDS:
    text-only       Switch to text-only mode (novel writing focused)
    full-ide        Switch to full IDE mode (complete programming environment)
    status          Show current mode and workspace information
    test [mode]     Test build for current or specified mode
    help            Show this help message

EXAMPLES:
    $0 status                    # Check current mode
    $0 text-only                 # Switch to text-only mode
    $0 full-ide                  # Switch to full IDE mode
    $0 test                      # Test current mode build
    $0 test text-only            # Test text-only mode build

MODES:
    full-ide     Complete programming environment with all features
                 • All ~150 crates included
                 • LSP, Git, Debugger, Extensions, Terminal, etc.
                 • Novel writing + coding features
                 
    text-only    Focused novel writing tool with coding features removed
                 • Only ~60 essential crates
                 • AI agents, manuscript management, export, analytics
                 • No LSP, Git, Debugger, Extensions, Terminal, etc.

FILES:
    Cargo.toml              Active configuration
    Cargo-text-only.toml    Text-only mode template
    Cargo-full.toml         Full IDE mode backup
    
For more information, see: TEXT_ONLY_MODE.md
EOF
}

# Main command handling
case "${1:-}" in
    "text-only")
        switch_to_text_only
        ;;
    "full-ide")
        switch_to_full_ide
        ;;
    "status")
        show_status
        ;;
    "test")
        mode="${2:-$(get_current_mode)}"
        test_build "$mode"
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    "")
        show_status
        echo ""
        echo "Use '$0 help' for usage information"
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac

