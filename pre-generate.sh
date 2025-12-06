#!/bin/bash
# Pre-generation hook for rust-desktop-template

set -e

echo "🔧 Preparing to generate Rust desktop application..."

# Validate project name
if [[ ! "$project_name" =~ ^[a-zA-Z][a-zA-Z0-9_-]*$ ]]; then
    echo "❌ Error: Project name must start with a letter and contain only letters, numbers, hyphens, and underscores"
    exit 1
fi

# Check if project name is a valid Rust identifier
if [[ "$project_name" =~ ^[0-9] ]]; then
    echo "❌ Error: Project name cannot start with a number"
    exit 1
fi

# Convert project name to valid Rust crate name
export CRATE_NAME="$(echo "$project_name" | tr '-' '_' | tr '[:upper:]' '[:lower:]')"

# Validate author name
if [[ -z "$author_name" ]]; then
    echo "❌ Error: Author name cannot be empty"
    exit 1
fi

# Validate email
if [[ ! "$author_email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
    echo "❌ Warning: Email format may be invalid"
fi

echo "✅ Validation passed!"
echo "📋 Project Configuration:"
echo "   Name: $project_name"
echo "   Crate: $CRATE_NAME"
echo "   Author: $author_name"
echo "   Email: $author_email"
echo "   Theme: $theme"
echo "   Include Examples: $include_examples"