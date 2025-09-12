#!/bin/bash
# BunkerVerse GitHub Branch Protection Rules Setup Script
# This script configures branch protection rules for develop and main branches
# Requires GitHub CLI (gh) and appropriate repository permissions

set -e

echo "🔒 BunkerVerse Branch Protection Setup"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ $1${NC}"
    else
        echo -e "${RED}❌ $1${NC}"
        exit 1
    fi
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ GitHub CLI (gh) is not installed${NC}"
    echo "Please install it from: https://cli.github.com/"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ Not authenticated with GitHub${NC}"
    echo "Please run: gh auth login"
    exit 1
fi

# Get repository info
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
if [ -z "$REPO" ]; then
    echo -e "${RED}❌ Could not determine repository${NC}"
    exit 1
fi

print_info "Repository: $REPO"

# Define required status checks for each workflow
BACKEND_CHECKS=(
    "All Checks Complete"
    "Lint and Format Check"
    "Build and Test"
    "Security Scans"
    "Docker Build and Vulnerability Scan"
)

CLIENT_CHECKS=(
    "All Client Checks Complete"
    "Lint and Format (Rust)"
    "Lint C++ Code"
    "Build NAR Library"
    "Build Client App Logic"
    "Security Scans (Rust)"
    "Security Scans (C++)"
    "Unit Tests"
)

GAMESERVER_CHECKS=(
    "All Game Server Stub Checks Complete"
    "C++ Static Analysis"
    "Build Game Server Stubs"
    "Security Scan C++ Dependencies"
)

PROTOBUF_CHECKS=(
    "validate-protobuf"
)

# Combine all checks
ALL_CHECKS=("${BACKEND_CHECKS[@]}" "${CLIENT_CHECKS[@]}" "${GAMESERVER_CHECKS[@]}" "${PROTOBUF_CHECKS[@]}")

echo ""
echo "🔧 Configuring Branch Protection Rules"
echo "-------------------------------------"

# Function to setup branch protection
setup_branch_protection() {
    local BRANCH=$1
    shift
    local CHECKS=("$@")
    
    print_info "Setting up protection for branch: $BRANCH"
    
    # Create the branch protection rule using GitHub API
    gh api \
        --method PUT \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        "/repos/$REPO/branches/$BRANCH/protection" \
        -f "required_status_checks[strict]=true" \
        -f "required_status_checks[contexts][]=$(printf '%s' "${CHECKS[@]}" | jq -R . | jq -s .)" \
        -f "enforce_admins=false" \
        -f "required_pull_request_reviews[required_approving_review_count]=1" \
        -f "required_pull_request_reviews[dismiss_stale_reviews]=true" \
        -f "required_pull_request_reviews[require_code_owner_reviews]=false" \
        -f "required_pull_request_reviews[require_last_push_approval]=false" \
        -f "restrictions=null" \
        -f "allow_force_pushes=false" \
        -f "allow_deletions=false" \
        -f "block_creations=false" \
        -f "required_conversation_resolution=true" \
        -f "lock_branch=false" \
        -f "allow_fork_syncing=false" || {
        
        # Alternative approach using gh CLI commands
        print_warning "Direct API call failed, trying gh ruleset approach..."
        
        # Create ruleset configuration file
        cat > branch-protection-ruleset.json <<EOF
{
  "name": "Protection for $BRANCH",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/$BRANCH"],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "pull_request",
      "parameters": {
        "required_approving_review_count": 1,
        "dismiss_stale_reviews_on_push": true,
        "require_code_owner_review": false,
        "require_last_push_approval": false,
        "required_review_thread_resolution": true
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "strict_required_status_checks_policy": true,
        "required_status_checks": [
$(printf '          {"context": "%s"}\n' "${CHECKS[@]}" | sed '$ s/,$//')
        ]
      }
    },
    {
      "type": "non_fast_forward",
      "parameters": {}
    },
    {
      "type": "deletion",
      "parameters": {}
    }
  ]
}
EOF
        
        gh api \
            --method POST \
            -H "Accept: application/vnd.github+json" \
            -H "X-GitHub-Api-Version: 2022-11-28" \
            "/repos/$REPO/rulesets" \
            --input branch-protection-ruleset.json
        
        rm -f branch-protection-ruleset.json
    }
    
    print_status "Branch protection configured for $BRANCH"
}

# Setup protection for develop branch
setup_branch_protection "develop" "${ALL_CHECKS[@]}"

# Setup protection for main branch (stricter)
setup_branch_protection "main" "${ALL_CHECKS[@]}"

echo ""
echo "📋 Branch Protection Summary"
echo "---------------------------"
echo "Protected branches: develop, main"
echo "Required status checks: ${#ALL_CHECKS[@]} checks configured"
echo "Required reviews: 1 approval required"
echo "Dismiss stale reviews: Enabled"
echo "Require conversation resolution: Enabled"
echo "Force pushes: Disabled"
echo "Branch deletion: Disabled"

echo ""
echo "🎉 Branch Protection Setup Complete!"
echo "===================================="
echo -e "${GREEN}✅ Branch protection rules have been configured successfully${NC}"
echo ""
echo "You can verify the settings at:"
echo "https://github.com/$REPO/settings/branches"