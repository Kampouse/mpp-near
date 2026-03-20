#!/bin/bash
cd /Users/asil/.openclaw/workspace/mpp-near

echo "GIT PUSH STATUS"
echo "==============="

# Check if there are uncommitted changes
CHANGES=$(git status --porcelain 2>/dev/null | wc -l)
echo "Uncommitted changes: $CHANGES"

# Check last commit
echo ""
echo "Last commit:"
git log -1 --oneline 2>/dev/null

# Check remote
echo ""
echo "Remote:"
git remote -v 2>/dev/null | head -1

# Check if ahead of remote
AHEAD=$(git rev-list --count origin/main..HEAD 2>/dev/null || echo "unknown")
echo ""
echo "Commits ahead of origin: $AHEAD"

# Check if pushed
if [ "$AHEAD" = "0" ]; then
    echo ""
    echo "✅ PUSHED - Up to date with remote"
elif [ "$AHEAD" = "unknown" ]; then
    echo ""
    echo "⚠️ Cannot determine push status"
else
    echo ""
    echo "⚠️ NOT PUSHED - $AHEAD commits ahead of origin"
fi
