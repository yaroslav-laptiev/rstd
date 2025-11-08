
<img width="1274" height="654" alt="Screenshot 2025-11-08 at 19 28 34" src="https://github.com/user-attachments/assets/1cad950c-f2cd-4f3b-9532-e1480c304a69" />

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
# Todo CLI - A Simple Kanban Board in Your Terminal 📋


A lightweight command-line todo app built with Rust for personal task management. Simple kanban board interface with SQLite persistence.

> **Note**: This is a personal learning project and hobby tool. The code might not follow all best practices - it's primarily built for fun, learning Rust, and my own productivity needs. Feel free to use it if you find it helpful!

## ✨ Features

- **Kanban Board**: 5 columns (Backlog, Today, In Progress, Done, Archived)
- **SQLite Storage**: Tasks persist between sessions
- **Keyboard Navigation**: All operations via keyboard
- **Task Management**: Create, delete, and move tasks
- **Deadlines**: Optional deadline support
- **Terminal UI**: Built with Ratatui

---

## 🚀 Installation

### Prerequisites
- Rust (1.70+) - [Install here](https://rustup.rs/)

### Build from Source
```bash
# Clone and build
git clone https://github.com/yaroslav-laptiev/rstd.git
cd rstd
cargo install --path .

# Run from anywhere
rstd
```

---

## 📖 Usage

### Keyboard Controls

#### Board Navigation
- **`q`** - Quit the application
- **`Tab`** - Switch between columns
- **`↑/↓`** - Navigate tasks within a column  
- **`←/→`** - Move selected task between columns
- **`n`** - Create a new task
- **`d`** - Delete the selected task

#### Task Creation Modal
- **`Ctrl + s`** - Save and create the task
- **`Tab`** - Switch between description and deadline fields
- **`Esc`** - Cancel and return to board

### Task Workflow
1. **Create a task**: Press `n` to open the task creation modal
2. **Enter description**: Type your task description
3. **Set deadline** (optional): Tab to deadline field and enter date
4. **Save**: Press `Ctrl + s` to create the task
5. **Organize**: Use arrow keys to move tasks through your workflow

### Database
Tasks are saved to a local SQLite database automatically.

---

## 🛣️ Maybe Coming Later

I might add these features when I have time:

### 🔄 1. Task Editing
- Edit existing tasks
- Modify deadlines

### 📊 2. Multiple Boards  
- Custom boards
- Different projects

### 🎨 3. UI Improvements
- Better colors
- Task tags
- Search

---

## 🤝 Feedback

If you find bugs or have suggestions:

- **Found an issue?** [Open an issue](https://github.com/yaroslav-laptiev/rstd/issues)
- **Have ideas?** I'd love to hear them!
- **Want to contribute?** Feel free to submit a PR!

### Contact
📧 **Email**: [laptievdev@gmail.com](mailto:laptievdev@gmail.com)

---

## ⭐ Support

If this helps you, consider:
- Starring the repo ⭐
- Sharing it with others
- Sending feedback

Thanks!

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Happy organizing!** 🎯
