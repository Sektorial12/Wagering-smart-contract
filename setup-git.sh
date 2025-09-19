#!/bin/bash

# Git Repository Setup Script for Wagering Smart Contract
# This script initializes the repository and prepares it for GitHub

echo "🚀 Setting up Git repository for Wagering Smart Contract..."

# Initialize git repository if not already initialized
if [ ! -d ".git" ]; then
    echo "📁 Initializing Git repository..."
    git init
else
    echo "✅ Git repository already initialized"
fi

# Configure git settings (optional - user can modify)
echo "⚙️ Configuring Git settings..."
echo "Please enter your Git configuration:"
read -p "Your name: " git_name
read -p "Your email: " git_email

git config user.name "$git_name"
git config user.email "$git_email"

# Set up default branch as main
git branch -M main

# Add all files to staging
echo "📦 Adding files to Git..."
git add .

# Check git status
echo "📋 Current Git status:"
git status

# Create initial commit
echo "💾 Creating initial commit..."
git commit -m "feat: initial commit - wagering smart contract

- Add core smart contract functionality
- Include comprehensive test suite  
- Add security documentation
- Set up proper project structure
- Include development guidelines"

echo "✅ Git repository setup complete!"
echo ""
echo "🔗 Next steps to push to GitHub:"
echo "1. Create a new repository on GitHub"
echo "2. Copy the repository URL"
echo "3. Run: git remote add origin <repository-url>"
echo "4. Run: git push -u origin main"
echo ""
echo "📚 Additional commands:"
echo "• Check status: git status"
echo "• View commits: git log --oneline"
echo "• Create branch: git checkout -b feature/branch-name"
echo ""
echo "🛡️ Security Note:"
echo "Private audit files are excluded via .gitignore"
echo "Only public documentation and code will be pushed"
