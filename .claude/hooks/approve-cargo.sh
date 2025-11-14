#!/usr/bin/env bash
# Auto-approve cargo commands for Color Bang! project

# Read JSON input from stdin
input=$(cat)

# Extract tool name and command
tool_name=$(echo "$input" | grep -o '"tool_name":"[^"]*"' | cut -d'"' -f4)
command=$(echo "$input" | grep -o '"command":"[^"]*"' | cut -d'"' -f4)

# Auto-approve if it's a cargo command
if [[ "$tool_name" == "Bash" && "$command" =~ ^cargo[[:space:]] ]]; then
    cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "Cargo command auto-approved for project"
  }
}
EOF
fi

exit 0
