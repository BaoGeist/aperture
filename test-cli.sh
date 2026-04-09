#!/bin/bash
# Test script for aperture CLI fixes

echo "Testing aperture CLI..."
echo ""

# Test 1: Launch from Finance-Master directory
echo "Test 1: Opening Finance-Master directory"
cd /home/bungeist/projects/Finance-Master
/home/bungeist/projects/aperture/aperture .
sleep 2

echo ""
echo "Test 2: Opening specific file"
cd /home/bungeist/projects
/home/bungeist/projects/aperture/aperture aperture/README.md
sleep 2

echo ""
echo "✓ Tests complete!"
echo ""
echo "Check your screen for aperture windows."
echo "The terminal should be free (not blocked)."
echo ""
echo "To use from anywhere, run: source ~/.zshrc"
echo "Then: aperture <file or directory>"
