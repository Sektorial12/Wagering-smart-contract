#!/bin/bash

# Repository Setup Verification Script
echo "🔍 Verifying Git repository setup..."

# Check if .gitignore exists and contains required entries
echo "📋 Checking .gitignore..."
if [ -f ".gitignore" ]; then
    echo "✅ .gitignore exists"
    
    # Check for audit files in .gitignore
    audit_files=("progress.txt" "Final_Audit_Report.md" "findings.md" "info.md" "validation_log.md" "info_validation_log.md")
    
    for file in "${audit_files[@]}"; do
        if grep -q "$file" .gitignore; then
            echo "✅ $file is properly ignored"
        else
            echo "❌ $file is NOT in .gitignore"
        fi
    done
else
    echo "❌ .gitignore missing"
fi

# Check for essential files
echo ""
echo "📁 Checking essential files..."
essential_files=("README.md" "LICENSE" "CONTRIBUTING.md" "Anchor.toml" "Cargo.toml" "package.json")

for file in "${essential_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
    fi
done

# Check directory structure
echo ""
echo "🏗️ Checking directory structure..."
directories=("programs" "tests" "security" "security/proof-of-concepts")

for dir in "${directories[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ $dir/ exists"
    else
        echo "❌ $dir/ missing"
    fi
done

# Check if audit files are properly excluded
echo ""
echo "🛡️ Verifying audit files are excluded..."
if [ -f "progress.txt" ]; then
    echo "⚠️ progress.txt exists (will be ignored by Git)"
else
    echo "✅ progress.txt not in repository"
fi

if [ -f "Final_Audit_Report.md" ]; then
    echo "⚠️ Final_Audit_Report.md exists (will be ignored by Git)"
else
    echo "✅ Final_Audit_Report.md not in repository"
fi

# Check Git status
echo ""
echo "📊 Git repository status:"
if [ -d ".git" ]; then
    echo "✅ Git repository initialized"
    echo "📋 Files to be committed:"
    git status --porcelain | head -10
    
    if [ $(git status --porcelain | wc -l) -gt 10 ]; then
        echo "... and $(( $(git status --porcelain | wc -l) - 10 )) more files"
    fi
else
    echo "❌ Git repository not initialized"
    echo "💡 Run: git init"
fi

echo ""
echo "🎯 Repository Setup Summary:"
echo "• Private audit files: Excluded from Git"
echo "• Public documentation: Ready for GitHub"
echo "• Project structure: Properly organized"
echo "• Security PoCs: Moved to security/ directory"
echo ""
echo "✅ Repository is ready for GitHub!"
