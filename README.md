# ASCII Sorting Visualizer

A simple TUI application that visualizes common sorting algorithms using ASCII bars in the terminal.

![demo](./demo.gif)

## Quick Start

```bash
cargo run
```

## 📊 Implemented Algorithms

The following algorithms were chosen for their step-by-step nature, which makes them great for visual learning:

- Bubble Sort
- Selection Sort
- Insertion Sort

All of them have a time complexity of **O(n²)**.  
More algorithms are planned for future releases!

## 🎮 Controls

| Key        | Action            |
| ---------- | ----------------- |
| `Space`    | Start / Pause     |
| `r`        | Randomize array   |
| `a`        | Change algorithm  |
| `q`, `Esc` | Quit              |

## 🧱 Dependencies

- [ratatui](https://crates.io/crates/ratatui) — TUI library
- [tokio](https://crates.io/crates/tokio) — Async runtime

## ⚙️ How It Works

1. Generates a random array
2. Runs the selected sorting algorithm
3. Visualizes each step using animated ASCII bars

## 🎯 Why?

~~why not?~~

This project was created to:

1. Visualize sorting algorithms for educational purposes
2. Have fun learning Rust and async programming
3. Experiment with building terminal-based UI applications

## 📌 TODO

- [ ] Add more sorting algorithms (Merge Sort, Quick Sort, Heap Sort, etc.)
- [ ] Add sounds 🎵
- [ ] Display performance statistics

---

Made with ❤️ by [fagundescodes](https://github.com/fagundescodes)
