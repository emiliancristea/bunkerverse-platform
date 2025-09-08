#!/bin/bash

# Setup script for Git hooks
echo "Setting up Git hooks for BUNKERVERSE Platform..."

# Make hooks executable
chmod +x .githooks/pre-commit

# Configure git to use our hooks directory
git config core.hooksPath .githooks

echo "✅ Git hooks configured successfully!"
echo "Pre-commit hooks will now run automatically on each commit."
echo ""
echo "To manually run pre-commit checks:"
echo "  ./.githooks/pre-commit"
echo ""
echo "To bypass pre-commit hooks (not recommended):"
echo "  git commit --no-verify"