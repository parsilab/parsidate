# GitHub Workflow Guide for parsidate Project

## Overview
This document describes the standard GitHub workflow to maintain a clean, manageable, and professional codebase for the parsidate project. Following this workflow ensures efficient collaboration, clear history, and smooth releases.

---

## 1. Branching Strategy

- **main branch**  
  - Contains stable, production-ready code only.  
  - No direct commits allowed on this branch.

- **Feature branches**  
  - Create a new branch for every new feature, bug fix, or documentation update.  
  - Branch names should be descriptive and follow this convention:  
    ```
    feature/<descriptive-name>
    bugfix/<descriptive-name>
    docs/<descriptive-name>
    ```
  - Example branch names:  
    ```
    feature/add-persian-date-converter
    bugfix/fix-date-offset-bug
    docs/update-readme
    ```

---

## 2. Commit Guidelines

- Write **clear and meaningful commit messages** following [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).  
- Commit message format:  
  `<type>: <short description>`

- Common types:  
  - `feat`: new feature  
  - `fix`: bug fix  
  - `docs`: documentation changes  
  - `refactor`: code changes that neither fix a bug nor add a feature  
  - `test`: adding or fixing tests

- Examples:  
  - feat: add support for Persian date conversion  
  - fix: correct off-by-one error in date calculation  
  - docs: update usage examples in README

---

## 3. Development Workflow

- Start by creating a new **feature branch** off `main`.  
- Make incremental commits with clear messages as you develop.  
- Push your branch to GitHub frequently to back up work and enable collaboration.  
- Keep your feature branch up to date by regularly rebasing it onto the latest `main` branch instead of merging, to maintain a clean history.

---

## 4. Pull Requests (PRs)

- Open a PR to merge your feature branch into `main` when development is complete.  
- In the PR description, clearly explain the purpose and scope of your changes.  
- Request at least one review from team members or yourself for code quality and correctness.  
- Use **Squash and Merge** to keep the commit history clean and linear.  
  This compresses all your feature branch commits into a single commit on `main`, making history easy to follow.

---

## 5. Issue Tracking

- Every new feature or bug fix should be associated with a GitHub Issue.  
- Create Issues to track work and assign Milestones and Labels accordingly.  
- Assign Issues to team members responsible for implementation.  
- Close Issues promptly when resolved.

---

## 6. Releases

- When a Milestone's work is complete and all associated Issues are closed, prepare a release.  
- Tag the release version in GitHub (e.g., `v1.8.0`).  
- Update release notes with significant changes, bug fixes, and new features.

---

## 7. Best Practices

- Avoid committing directly to `main`. Always use feature branches and PRs.  
- Keep commit messages meaningful and concise.  
- Regularly update documentation as the code evolves.  
- Use Labels and Milestones consistently to track progress and priorities.  
- Coordinate with the team to ensure smooth collaboration.

---

## Summary

| Step            | Action                                            |
|-----------------|--------------------------------------------------|
| Branching       | Create feature/bugfix/docs branch from `main`    |
| Development     | Commit frequently with clear messages             |
| Pull Request    | Open PR, request review, squash & merge to `main`|
| Issue Management| Track work with Issues, assign Milestones/Labels  |
| Release         | Tag version, write release notes                   |

---

Keep this guide handy and update it as your workflow evolves!

---

**End of Document**
