# GitHub Commit Message Guidance

## Commit Message Format

All commits should follow this standardized format:

```
Task X.Y.Z: [Action] [Component/Feature] [Brief Description]
```

## Task Numbering Convention

- `Task X.0`: Initial/primary task implementation
- `Task X.Y.Z`: Incremental fixes, improvements, or additions
- `Task X.Y.FINAL`: Final consolidation commit (REQUIRED)

**MANDATORY**: Every task must end with a `.FINAL` commit that represents the complete, finished implementation.

## Author Guidelines

### ❌ NEVER Use AI Co-authoring
```
Co-Authored-By: Claude <noreply@anthropic.com>
🤖 Generated with [Claude Code](https://claude.ai/code)
```

### ✅ Always Use Human Author Only
```
Author: Emilian Cristea <emilian@bunkercorpo.com>
```

## Quality Standards

- Task number follows X.Y.Z convention
- Action verb is appropriate and clear
- Component/feature is accurately described
- No AI co-authoring attribution
- Commit message is professional and concise
- Summary line: 50-72 characters max
- Body lines: 72 characters max per line
- **CRITICAL**: Every task implementation must conclude with a `.FINAL` commit