---
name: advance-biome
description: Moves Air forward to a newer version of Biome, one problem at a time. Finds the next Biome change that Air can't handle as-is, points Air at that change, fixes Air to match on a fresh feature/advance-biome branch (without committing), and explains the fix in plain language. Invoked only by the /advance-biome command.
disable-model-invocation: true
---

# Advance Biome

## What this is for

Air borrows a lot of code from another project called Biome. Air doesn't follow Biome's latest version on its own. Instead, `Cargo.toml` names one exact Biome commit, and Air uses Biome exactly as it was at that moment. This skill calls that commit "the pin".

Biome keeps changing, and Air has fallen thousands of changes behind. Catching up all at once would be one huge change that nobody could review. So we catch up in small steps:

1. Move the pin forward as far as it can go, until we reach a Biome change that Air can't handle as-is. This skill calls that one "the breaking change".
2. Stop there, and fix Air so it works with the breaking change.
3. Explain the fix to the user in plain words, so they can review it and open a pull request.

The user runs this skill again after each pull request is merged, until Air has caught up.

## Rules

- Don't commit and don't push. The user reviews the work and commits it.
- Fix only the breaking change. Don't clean up anything else.
- Never accept a change in how Air formats R code just to make tests pass. If the breaking change makes Air format R code differently, stop and show the user a before-and-after example.
- If the right fix isn't clear, or would be large, stop and ask the user. Don't guess.
- Everything you tell the user follows "Step 5: Tell the user" below.

## Step 1: Get ready

Run these from the Air folder. `origin` is the copy of Air on GitHub at posit-dev/air.

1. Check for unsaved work. `git status --short --untracked-files=no` should print nothing. If it prints anything, stop and ask the user what to do with it. New files that git isn't tracking yet are fine to leave alone.
2. Get the latest Air: `git fetch origin`
3. Check for a leftover branch. If `git branch --list feature/advance-biome` prints anything, it's probably from last time. Stop and ask the user whether to delete it.
4. Make the new branch from the latest `main` on GitHub. `--no-track` means the branch isn't linked to `main`, so pushing it later makes a new branch on GitHub instead of going to `main`.
   ```
   git switch --no-track -c feature/advance-biome origin/main
   ```
5. Find the pin. Every Biome line in `Cargo.toml` should name the same commit, so this should print exactly one line:
   ```
   grep -o 'biomejs/biome", rev = "[0-9a-f]*"' Cargo.toml | sort -u
   ```
   If it prints more than one, stop and ask the user.

## Step 2: Get Biome's history

Download Biome's history into a temporary folder. Use your scratch folder if you have one. Below, `<temp>` means that folder. The download skips file contents until they're needed, so it takes seconds:

```
git clone --quiet --filter=blob:none --no-checkout https://github.com/biomejs/biome.git <temp>/biome
```

Then write down every change made on Biome's main line since the pin, oldest first, one per line:

```
git -C <temp>/biome log --first-parent --reverse --format='%H %cs %s' <pin>..origin/main > <temp>/candidates.txt
```

`--first-parent` keeps us on Biome's main line. Now and then Biome lands a big batch of changes at once, which shows up as a line like "Merge pull request #1234 from biomejs/next". That whole batch counts as one line here.

If the file is empty, Air is already caught up. Tell the user and stop.

## Step 3: Find the breaking change

[scripts/check-air.sh](scripts/check-air.sh) does one try. It points Air at a Biome commit, runs the same checks Air's GitHub checks run, and ends with one line saying how it went:

```
bash <this skill's folder>/scripts/check-air.sh --pin <full commit>
```

| Last line | Meaning |
|---|---|
| `RESULT: WORKS` | Air is fine with this commit. |
| `RESULT: NEEDS AIR CHANGES` | Something failed. The log it names says what. |
| `RESULT: COULD NOT RUN` | The script hit a problem before checking anything. Fix that and try again. |

Each try can take many minutes, the first one most of all, because Biome has to be built. Give it a time limit of at least 30 minutes.

Biome often starts needing a newer version of some shared library (for example `cfg-if` or `camino`), and Cargo won't raise the version in Air's `Cargo.lock` by itself. That doesn't count as a breaking change. With `--pin`, the script resets `Cargo.lock`, bumps only the libraries Cargo says conflict (it prints a "Bumping ..." line for each), and keeps those bumps with the pin. Don't ask the user about this. Do mention the bumped libraries in "What I changed in Air". If the lock file still can't be worked out, the script says COULD NOT RUN. Look at why before going on.

Before you believe a "NEEDS AIR CHANGES", read the log and make sure Biome caused it. A Biome cause looks like a build error that mentions a `biome_` package, or a test whose expected output changed. Things like a lost network connection or a test that fails at random are not Biome's fault. If in doubt, run the same try again.

How to search:

1. Try the newest commit first, the last line of `candidates.txt`. If it works, Air can jump straight to the newest Biome. Keep that pin, skip Step 4, and tell the user Air has caught up.
2. Otherwise, narrow it down by halving. The pin works and the newest commit doesn't, so the breaking change is somewhere in between. Try the commit halfway between the newest one that worked and the oldest one that failed. If it works, the breaking change is later. If it fails, the breaking change is that one or earlier. Keep halving until you find two neighbors in the list: one that works, and the very next one that fails. With about 3,000 lines, this takes about 12 tries.

   This works because once Biome changes something Air relies on, that change stays in every later version.
3. Keep a small table of every try (line number, commit, result) and show it to the user as you go. If something interrupts the search, the table lets it pick up where it left off.

The first commit that fails is the breaking change. Air has to end up pinned to it. If your last try was a different commit, run the script once more with `--pin <breaking change>`, and expect it to say NEEDS AIR CHANGES.

## Step 4: Understand and fix

Find out what the breaking change did and why:

- Its description and the files it touched: `git -C <temp>/biome show --stat <commit>`
- What it changed in the Biome parts Air uses. Air uses the `biome_` packages listed in Air's `Cargo.lock`, which live in Biome's `crates/` folder: `git -C <temp>/biome show <commit> -- crates/biome_rowan crates/biome_formatter`, and so on.
- The description usually ends with a pull request number like `(#1234)`. Read https://github.com/biomejs/biome/pull/1234 to learn why Biome made the change.
- If it's one of the big batches from `biomejs/next`, look inside the batch for the change that matters: `git -C <temp>/biome log --oneline <commit>^1..<commit>^2 -- crates/biome_rowan crates/biome_formatter`, and so on. Air stays pinned to the batch, but tell the user which change inside it caused the problem.

Common kinds of breaking changes:

| What Biome did | What Air usually needs |
|---|---|
| Renamed, moved, or removed something Air uses | Update Air to the new name or the new way of doing it. |
| Started needing a newer version of Rust | Raise `channel` in `rust-toolchain.toml` and `rust-version` in `Cargo.toml`. |
| Changed how messages or errors are printed | Update Air's saved test output, after checking the new output makes sense. |
| Changed how formatting decisions are made | This may change how Air formats R code. See the rules at the top. |

Some of Air's code is written by a program instead of by hand: the files in `crates/air_r_syntax/src/generated/`, which come from `xtask/codegen/`. Don't edit the written-out files. Change the program in `xtask/codegen/` and rerun it with `just gen-grammar`.

Many of Air's tests compare against saved output. When saved output needs updating, `just test-insta` followed by `cargo insta accept` updates the formatter's saved output, and `just test-snapshot-accept` updates the parser's. Look at what changed before you keep it.

After making the fix, check Air as it is now. Leave out `--pin` this time:

```
bash <this skill's folder>/scripts/check-air.sh
```

Repeat until it says `RESULT: WORKS`. If you can't get there, stop and tell the user what's still failing.

Before moving on, confirm:

- [ ] You're on `feature/advance-biome`
- [ ] Every Biome line in `Cargo.toml` names the breaking change
- [ ] `check-air.sh` without `--pin` says `RESULT: WORKS`
- [ ] Nothing is committed

## Step 5: Tell the user

Write the summary in plain language, as if explaining to a smart friend who has never seen Air's or Biome's code. Use short sentences. Avoid technical terms. If one can't be avoided, explain it in a few words right where it appears. Keep the whole summary short, about 15 lines or fewer.

Use these headings, in this order:

- **Where we are:** Which Biome commit Air uses now, its date, and a link like https://github.com/biomejs/biome/commit/<commit>. How many lines of `candidates.txt` we moved past, and how many are left. Count what's left with `git -C <temp>/biome rev-list --count --first-parent <breaking change>..origin/main`.
- **What Biome changed:** One to three sentences, with a link to Biome's pull request.
- **Why Air cared:** One or two sentences.
- **What I changed in Air:** Each file you changed, with one plain sentence about why.
- **How I checked:** Usually "All of Air's checks pass."
- **Worth a closer look:** Anything you're unsure about, or anything Air users might notice. Write "Nothing" if there isn't anything.
- **Suggested pull request title:** One line.

Here's a made-up example of the style to aim for:

> **Where we are:** Air now uses Biome from July 21, 2025 (link). That's 41 Biome changes further along than before, with 3,033 still to go.
>
> **What Biome changed:** Biome changed how it stores small pieces of text, to use less memory (link to the pull request). Because of that, a tool Air uses to read the text of a piece of code now gives the text back in a different form.
>
> **Why Air cared:** Air used that text in four places and expected the old form, so Air wouldn't build.
>
> **What I changed in Air:** `crates/air_r_formatter/src/string_literal.rs` now asks for the text in the new form. Nothing else changed.
>
> **How I checked:** All of Air's checks pass, and no test output changed.
>
> **Worth a closer look:** Nothing.
>
> **Suggested pull request title:** Advance Biome to July 21, 2025
