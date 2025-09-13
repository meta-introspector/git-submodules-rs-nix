# Git Diff for Beginners: Understanding Your Changes

Welcome to the world of Git! One of the most fundamental and powerful commands you'll use is `git diff`. It helps you see exactly what changes you (or others) have made to your code. Think of it as a "spot the difference" game for your files.

## What is `git diff`?

`git diff` shows you the difference between two points in your Git repository. These "points" can be:

*   Your current working files and what's in your staging area.
*   Your staging area and your last committed version.
*   Two different commits.
*   Two different branches.

Understanding `git diff` is crucial for:

*   **Reviewing your own work**: Catching mistakes before you commit.
*   **Collaborating**: Understanding changes made by teammates.
*   **Debugging**: Pinpointing when a bug was introduced.

## Basic Usage: Seeing Unstaged Changes

The most common way to use `git diff` is to see changes you've made in your working directory that you haven't yet added to the staging area (with `git add`).

**Command:**
```bash
git diff
```

**What it shows:** Any modifications, additions, or deletions in files that are currently in your project folder but haven't been `git add`ed.

**Example Scenario:** You've just edited `main.rs` and `lib.rs`. Running `git diff` will show you all the changes across both files.

## Seeing Staged Changes

Once you've used `git add` to stage some changes, `git diff` alone won't show them anymore. To see what's in your staging area *compared to your last commit*, you use the `--staged` (or `--cached`) flag.

**Command:**
```bash
git diff --staged
# or
git diff --cached
```

**What it shows:** Changes that you've `git add`ed and are ready to be committed, compared to the version in your last commit.

**Example Scenario:** You edited `main.rs`, then ran `git add main.rs`. Now, `git diff` will show nothing (because `main.rs` is staged), but `git diff --staged` will show the changes you made to `main.rs`.

## Understanding the Output (Unified Diff Format)

`git diff` output might look a bit intimidating at first, but it follows a consistent pattern:

```diff
diff --git a/src/main.rs b/src/main.rs
index 1234567..abcdef0 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 fn main() {
-    println!("Hello, world!");
+    println!("Hello, Git diff world!");
+    println!("This is a new line.");
 }
```

Let's break it down:

*   `diff --git a/src/main.rs b/src/main.rs`: Indicates that `git diff` is comparing two versions of `src/main.rs`.
*   `--- a/src/main.rs`: The original (old) version of the file.
*   `+++ b/src/main.rs`: The new version of the file.
*   `@@ -1,5 +1,6 @@`: This is a "hunk header".
    *   `-1,5`: Means the old file had 5 lines starting from line 1.
    *   `+1,6`: Means the new file has 6 lines starting from line 1.
*   Lines starting with `-`: These lines were **deleted** from the old file.
*   Lines starting with `+`: These lines were **added** to the new file.
*   Lines starting with a space: These lines are **unchanged** context lines, shown to help you understand where the changes occurred.

In the example above, `println!("Hello, world!");` was removed, and two new `println!` statements were added.

## Comparing Commits and Branches

`git diff` can also compare different commits or branches.

### Comparing Two Commits

**Command:**
```bash
git diff <commit_hash_1> <commit_hash_2>
```

**Example:** `git diff HEAD~1 HEAD` will show you the changes introduced in your very last commit (by comparing it to the commit before it).

### Comparing Two Branches

**Command:**
```bash
git diff <branch_name_1> <branch_name_2>
```

**Example:** `git diff main feature/my-feature` will show you all the differences between your `main` branch and your `feature/my-feature` branch. This is super useful before merging!

## Pro Tip: Diff a Specific File

If you only want to see changes for a single file, you can specify it:

**Command:**
```bash
git diff <file_path>
# or for staged changes of a file
git diff --staged <file_path>
```

**Example:** `git diff src/lib.rs`

## Practice Makes Perfect!

The best way to learn `git diff` is to use it. Make some changes in your project, stage some, commit some, and then experiment with the commands above. You'll quickly become a `git diff` master!