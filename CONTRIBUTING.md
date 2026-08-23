# Contributing to Linuxy

Thank you for your interest in contributing to Linuxy! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting Code](#submitting-code)
- [Coding Guidelines](#coding-guidelines)
  - [Frontend (TypeScript/React)](#frontend-typescriptreact)
  - [Backend (Rust)](#backend-rust)
  - [Styling (CSS)](#styling-css)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity, gender expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to creating a positive environment:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

Examples of unacceptable behavior:

- The use of sexualized language or imagery and unwelcome sexual attention
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- [ ] A GitHub account
- [ ] Git installed on your system
- [ ] Bun 1.1+ installed
- [ ] Rust 1.70+ installed
- [ ] Basic knowledge of React, TypeScript, and Rust

### First Contribution

Not sure where to start? Look for issues labeled:

- `good first issue` - Great for first-time contributors
- `help wanted` - Tasks that need community help
- `bug` - Bug fixes are always welcome

---

## Development Setup

### 1. Fork the Repository

```bash
# Click "Fork" on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/linuxy.git
cd linuxy
```

### 2. Set Up Development Environment

```bash
# Install Bun dependencies
bun install

# Install Rust dependencies (if needed)
cd src-tauri && cargo fetch
```

### 3. Install System Dependencies

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libxdo-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel \
    openssl-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libxdo-devel
```

### 4. Run in Development Mode

```bash
# Start the development server
bun run dev

# Or run Tauri in development mode
bun run tauri dev
```

### 5. Create a Branch

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-123
```

---

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find the problem has already been reported.

**When creating a bug report, include:**

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Screenshots** if applicable
- **System information**: OS, Linuxy version, etc.

**Example:**

```markdown
**Title:** App fails to launch when sandboxing is enabled

**Description:**
When I enable Firejail sandboxing for an AppImage, it fails to launch with an error.

**Steps to Reproduce:**
1. Install any AppImage (e.g., Firefox)
2. Click the shield icon to enable sandboxing
3. Click Launch
4. See error: "Failed to launch app with firejail"

**Expected:** App should launch in sandboxed mode
**Actual:** Error message appears

**Environment:**
- OS: Ubuntu 22.04
- Linuxy: 0.1.0
- Firejail: 0.9.68
```

### Suggesting Features

Feature suggestions are welcome! Please provide:

- **Use case**: Why is this feature needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches you've thought about

### Submitting Code

1. **Create a branch** from `main`
2. **Make your changes** following coding guidelines
3. **Test your changes** thoroughly
4. **Commit** with clear messages
5. **Push** to your fork
6. **Open a Pull Request**

---

## Coding Guidelines

### Frontend (TypeScript/React)

#### Component Structure

```typescript
import { useState, useEffect } from "react";
import { SomeIcon } from "lucide-react";
import "./ComponentName.css";

interface ComponentNameProps {
  prop1: string;
  prop2: boolean;
}

const ComponentName: React.FC<ComponentNameProps> = ({ prop1, prop2 }) => {
  // Hooks at the top
  const [state, setState] = useState<Type>(initialValue);

  // Event handlers
  const handleClick = () => {
    // Handler logic
  };

  // Effects
  useEffect(() => {
    // Effect logic
  }, [dependencies]);

  // Render
  return (
    <div className="component-name">
      {/* JSX content */}
    </div>
  );
};

export default ComponentName;
```

#### TypeScript Guidelines

- Use TypeScript for all new code
- Define interfaces for props and state
- Avoid `any` - use proper types
- Use union types for limited values:

```typescript
type Theme = "dark" | "light" | "system";
type View = "library" | "settings" | "discover";
```

#### React Best Practices

- Use functional components with hooks
- Keep components small and focused
- Extract reusable logic into custom hooks
- Use meaningful variable and function names

```typescript
// ✅ Good
const loadApps = async () => {
  const apps = await invoke<AppInfo[]>("get_installed_apps");
  setApps(apps);
};

// ❌ Avoid
const x = async () => {
  const a = await invoke("get_installed_apps");
  setApps(a);
};
```

### Backend (Rust)

#### Command Structure

```rust
#[tauri::command]
pub async fn command_name(param1: String, param2: bool) -> Result<(), String> {
    // Validate inputs
    if param1.is_empty() {
        return Err("Parameter cannot be empty".into());
    }

    // Main logic
    // ...

    Ok(())
}
```

#### Error Handling

- Use `Result<T, String>` for Tauri commands
- Provide descriptive error messages
- Handle errors gracefully

```rust
// ✅ Good
let file = fs::read_to_string(&path)
    .map_err(|e| format!("Failed to read file: {}", e))?;

// ❌ Avoid
let file = fs::read_to_string(&path).unwrap();
```

#### Async/Await

- Use async for I/O operations
- Keep async functions focused
- Avoid blocking operations in async context

```rust
pub async fn download_file(url: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await
        .map_err(|e| format!("Download failed: {}", e))?;
    
    Ok(response.text().await.unwrap())
}
```

### Styling (CSS)

#### CSS Custom Properties

Use CSS variables for theming:

```css
:root {
  --bg-main: #121212;
  --text-primary: #ffffff;
  --accent-color: #4caf50;
}

[data-theme="light"] {
  --bg-main: #f5f5f5;
  --text-primary: #121212;
}

.component {
  background: var(--bg-main);
  color: var(--text-primary);
}
```

#### Naming Conventions

Use kebab-case for class names:

```css
/* ✅ Good */
.app-card { }
.app-card__icon { }
.app-card__title { }

/* ❌ Avoid */
.appCard { }
.app_card_icon { }
```

#### Responsive Design

```css
.component {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
}

@media (max-width: 768px) {
  .component {
    grid-template-columns: 1fr;
  }
}
```

---

## Commit Guidelines

### Conventional Commits

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Build/config changes

### Examples

```bash
# Feature
git commit -m "feat: add automatic update checking"

# Bug fix
git commit -m "fix: resolve crash when launching sandboxed apps"

# Documentation
git commit -m "docs: update installation guide for Fedora"

# Breaking change (include BREAKING CHANGE in footer)
git commit -m "feat!: redesign app card layout"
```

### Commit Message Tips

- Use imperative mood ("add" not "added")
- Don't capitalize the first letter after type
- No period at the end
- Keep subject line under 72 characters

---

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Tests pass (if applicable)
- [ ] Documentation updated
- [ ] No new warnings

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested these changes

## Checklist
- [ ] Code follows project guidelines
- [ ] Self-reviewed code
- [ ] Commented complex code
- [ ] Updated documentation
- [ ] No new warnings
- [ ] Tests added/updated
```

### Review Process

1. **Submit PR** with clear description
2. **Automated checks** run (CI/CD)
3. **Maintainer review** - feedback may be provided
4. **Address feedback** - make requested changes
5. **Approval** - PR is merged

---

## Testing

### Frontend Testing

```bash
# Run tests (when implemented)
bun run test

# Run with coverage
bun run test:coverage
```

### Backend Testing

```bash
# Run Rust tests
cd src-tauri && cargo test

# Run with output
cd src-tauri && cargo test -- --nocapture
```

### Manual Testing

Test your changes:

1. **Build the app**: `bun run tauri build`
2. **Test affected features** thoroughly
3. **Check edge cases**
4. **Verify on different environments** if possible

---

## Documentation

### Code Comments

- Comment **why**, not **what**
- Explain complex logic
- Don't state the obvious

```rust
// ✅ Good - explains why
// Use firejail --appimage flag for better isolation
Command::new("firejail").arg("--appimage")

// ❌ Avoid - states the obvious
// Create a new firejail command
Command::new("firejail")
```

### Documentation Files

When adding features, update:

- `README.md` - Feature overview
- `docs/USAGE.md` - User documentation
- `docs/INSTALL.md` - Installation changes

### Inline Documentation

```typescript
/**
 * Installs an AppImage to the user's library.
 * 
 * @param path - Full path to the AppImage file
 * @returns Promise resolving to installation result
 * @throws Error if installation fails
 */
const installAppImage = async (path: string): Promise<void> => {
  // Implementation
};
```

---

## Additional Resources

- [Tauri Documentation](https://tauri.app/docs)
- [React Documentation](https://react.dev)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

## Questions?

Need help? Reach out via:

- **GitHub Discussions**: [Ask a question](https://github.com/swadhinbiswas/linuxy/discussions)
- **GitHub Issues**: [Report a problem](https://github.com/swadhinbiswas/linuxy/issues)

---

<div align="center">
  <p><strong>Thank you for contributing to Linuxy!</strong></p>
  <p>🐧 Built with ❤️ for the Linux community</p>
</div>
