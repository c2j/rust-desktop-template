#!/bin/bash
# Post-generation hook for rust-desktop-template

set -e

echo "🎉 Rust desktop application generated successfully!"
echo ""
echo "📁 Project Structure:"
echo "   $project_name/"
echo "   ├── src/"
echo "   ├── tests/"
echo "   ├── docs/"
echo "   └── examples/"
echo ""
echo "🚀 Next Steps:"
echo "   cd $project_name"
echo "   cargo run"
echo ""
echo "📖 Documentation:"
echo "   docs/en/quickstart.md - English quick start guide"
echo "   docs/zh/quickstart.md - Chinese quick start guide"
echo ""
echo "🔧 Customization:"
echo "   1. Edit src/app.rs to customize application logic"
echo "   2. Add modules in src/modules/"
echo "   3. Customize themes in src/themes/"
echo "   4. Update project.toml for configuration"
echo ""
echo "📚 Learn More:"
echo "   cargo doc --open"
echo ""
echo "🎨 Theme: $theme theme will be loaded by default"
if [[ "$include_examples" == "true" ]]; then
    echo "📋 Examples: File browser, text editor, and system monitor modules included"
fi

echo ""
echo "Happy coding! 🦀"