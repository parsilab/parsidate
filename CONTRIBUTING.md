# Contributing to parsidate

🎉 First off — thanks for considering contributing to **parsidate**!  
We welcome bug reports, feature suggestions, code improvements, documentation fixes, and more.

Let’s build a stronger Rust ecosystem for Persian software together.

---

## 🛠️ Getting Started

1. **Fork the repository** and clone it locally.
2. Install Rust via [rustup](https://rustup.rs).
3. Run the test suite to verify setup:
   ```bash
   cargo test
   ```

---

## 🌱 Making a Contribution

- Pick an existing [issue](https://github.com/parsicore/parsidate/issues) or open a new one to discuss your idea.
- Use a **feature branch** with a clear name:
  ```
  feature/short-description
  bugfix/short-description
  chore/short-description
  ```
  Example: `feature/add-city-filter`

- Follow [Conventional Commits](https://www.conventionalcommits.org/):
  - `feat: Add city filtering by province`
  - `fix: Handle UTF-8 encoding issue`
  - `docs: Improve README examples`
  - `refactor: Simplify data loading logic`

---

## ✅ Before Submitting a Pull Request

- Format code:
  ```bash
  cargo fmt
  ```
- Lint your code:
  ```bash
  cargo clippy
  ```
- Run tests:
  ```bash
  cargo test
  ```
- Write or update tests when necessary.
- Ensure your PR targets the `main` branch.
- Keep your PR focused and well-scoped.
- Use a descriptive title and fill out the PR description.

---

## 📚 Contributing to Documentation

- Found a typo or unclear sentence? Please fix it!
- Improving docs is just as valuable as writing code.
- Docs are found in:
  - Inline Rust doc comments (`///`) in `src/`
  - Or markdown pages in `docs/` (if available)

---

## 💬 Need Help?

- Open a GitHub [discussion](https://github.com/parsicore/parsidate/discussions) for general questions or brainstorming.
- For specific issues, file a ticket with the appropriate label:
  - `bug`, `feature`, `question`, etc.

---

## ❤️ Code of Conduct

We follow the [parsicore Code of Conduct](https://github.com/parsicore/.github/blob/main/CODE_OF_CONDUCT.md).  
Please be respectful in all interactions.

---

Thanks again!  
Your contributions help make **parsidate** and the entire **parsicore** ecosystem better. 🚀
