# LazyLife - Future Roadmap

LazyLife is a terminal-based productivity app for journaling, task management, habit tracking, and mood logging. This document outlines the future plans and enhancements for LazyLife, based on user feedback and potential improvements.

## Vision

LazyLife aims to be a personal command center for productivity, combining journaling, task management, habit tracking, and mood logging. It's designed to be lightweight, file-based, and deeply integrated into your workflow, with a focus on speed, simplicity, and hackability.

## Current Features

- **Journaling**: Basic file-based journaling with NeoVim/Vim integration
- **Task Management**: To-do list with check/uncheck functionality
- **Habit Tracker**: Simple habit logging
- **Mood Tracker**: Mood logging with basic charts
- **TUI Interface**: Ratatui-based terminal UI

## Future Enhancements

### 1. Core Improvements

- **Config File**: Add `~/.lazylife/config.yaml` for settings like editor preference, storage locations, and themes.
- **Keyboard Shortcuts**: Enhance shortcuts (e.g., `a` to add, `d` to delete, `e` to edit) for faster workflows.
- **Multiple Journals**: Support categories like "Work Journal", "Personal Journal", etc.

### 2. Habit & Mood Tracker Ideas

- **Habit Frequency**: Implement a streak system with visual indicators (e.g., `■■■□□` for 3/5 days).
- **Mood Trend**: Add weekly summaries with averages and comparisons.

### 3. Task Management Ideas

- **Priorities & Deadlines**: Add task priorities (`P1`, `P2`) and due dates.
- **Recurring Tasks**: Support syntax like `task: water plants [daily]`.
- **Filters**: View tasks by `today`, `this week`, or `completed`.

### 4. Journal Ideas

- **Daily Templates**: Auto-fill journals with prompts like "Morning Thoughts", "Work Progress", "Evening Reflection".
- **Search**: Implement keyword search across journal files.

### **Tasks**

- [ ] Add, edit, delete tasks
- [ ] Mark tasks as done/undone
- [ ] Organize tasks by tags/projects
- [ ] Task deadlines + priorities

### **Habit & Mood Tracking**

- [ ] Define habits (e.g., "Workout", "Meditate")
- [ ] Daily habit check-ins
- [ ] Mood tracking scale (e.g., 1-5 or emojis)
- [ ] Weekly habit & mood summary view

---

### **Command List (UI + keyboard accessible)**

```
General:
  :quit / :q             → Quit app
  :config                → Open config file
  :sync                  → Manual sync with GitHub
  :help                  → Show commands/help

Journaling:
  :journal new           → New journal entry
  :journal list          → List all journals
  :journal edit ID       → Edit entry
  :journal delete ID     → Delete entry

Tasks:
  :task add "title"      → Add task
  :task list             → List tasks
  :task done ID          → Mark as done
  :task edit ID          → Edit task
  :task delete ID        → Delete task

Habits & Mood:
  :habit add "name"      → Add new habit
  :habit check ID        → Mark habit as done
  :habit list            → View habits progress
  :mood log 3            → Log mood (1-5 scale)
  :mood summary          → Weekly mood summary
```

---

## **Agenda 3: Tech Stack**

- [x] **Language:** Rust
- [x] **TUI Framework:** [Ratatui](https://github.com/ratatui/ratatui) (active & maintained)
- [x] **Data Storage:**

  - Journals → Markdown files
  - Tasks, Habits, Moods → JSON/YAML files

- [ ] **Git Integration:** Native Git CLI commands for commit/push/pull
- [x] **UI Inspiration:** Lazygit-like panels & command palette

---

## **Agenda 4: Project Phases**

### **Phase 1: Core Features**

- [ ] Journaling panel + commands
- [ ] Tasks panel + commands
- [ ] Habits & Mood panel + commands
- [ ] Local file storage structure
- [ ] Basic UI layout with keyboard shortcuts

### **Phase 2: Git Integration**

- [ ] Auto-commit every X mins
- [ ] Manual push/pull sync commands
- [ ] Basic conflict resolution UI

### **Phase 3: Time Blocking & Calendar**

- [ ] Add daily/weekly time blocks
- [ ] Calendar-style TUI panel
- [ ] Schedule view

### **Phase 4: UI/UX Enhancements**

- [ ] Command palette at bottom
- [ ] Configurable themes, shortcuts
- [ ] Status bar for sync status, commit timer

### **Phase 5: Advanced Features**

- [ ] Search across journals, tasks, moods
- [ ] Weekly/monthly analytics → habits & mood graphs
- [ ] Optional plugin API in future

---

## **Agenda 5: Future Enhancements**

- [ ] Multi-repo support for different projects
- [ ] Export journals/tasks to PDF/HTML
- [ ] Mobile-optimized layouts for small screens
- [ ] Collaboration mode via shared repo

## **Phase-Wise Roadmap (Timeline)**

### **Phase 1: Core Features (Weeks 1–4)**

**Goal:** Journaling, Tasks, Habits & Mood tracking with basic UI.

**Week 1:** Project Setup

- [x] Rust project setup with [Ratatui](https://github.com/ratatui/ratatui)
- [x] Config system (`config.yaml`) for settings (repo URL, commit interval)
- [x] Local file structure for journals, tasks, habits, moods

**Week 2:** Journaling Feature

- [x] UI panel for journals (list view + create new)
- [x] `:journal new`, `:journal list`, `:journal edit` commands
- [x] Markdown file creation & reading

**Week 3:** Tasks Feature

- [x] UI panel for tasks (list + details)
- [x] `:task add`, `:task list`, `:task done`, `:task edit` commands
- [x] JSON storage for tasks

**Week 4:** Habits & Mood Feature

- [x] Habits tracking panel (`:habit add`, `:habit check`, `:habit list`)
- [ ] Mood tracking (`:mood log`, `:mood summary`)
- [ ] Weekly summary view for habits & moods

---

### **Phase 2: Git Integration (Weeks 5–6)**

**Goal:** Auto/manual Git sync with commit & push.

**Week 5:**

- [ ] Manual Git commands → `:sync` for commit & push
- [ ] Basic status bar for repo & commit info

**Week 6:**

- [ ] Auto-commit timer (every X mins)
- [ ] Auto-push after commit if online

---

### **Phase 3: UI & UX Enhancements (Weeks 7–8)**

**Goal:** Smooth, intuitive interface like Lazygit.

**Week 7:**

- [ ] Command palette for all commands
- [ ] Configurable themes & keyboard shortcuts

**Week 8:**

- [ ] Optimized layouts for small screens
- [ ] Persistent UI state across sessions

---

### **Phase 5: Advanced Features (Weeks 9–10)**

**Goal:** Polish & extras for power users.

**Week 9:**

- [ ] Search across journals, tasks, habits, moods
- [ ] Analytics → Habit streaks, mood trends

**Week 10:**

- [ ] Export to PDF/HTML
- [ ] Multi-repo support for separate projects

---

### **Phase 6: Future Enhancements (Optional)**

- [ ] Plugin API for community extensions
- [ ] Collaboration mode via shared repo
- [ ] Mobile-specific UI improvements
