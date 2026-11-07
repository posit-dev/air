---
name: advance-biome
description: Moves Air forward to a newer version of Biome, one problem at a time. Finds the next Biome change that Air can't handle as-is, points Air at that change, fixes Air to match on a fresh feature/advance-biome branch (without committing), and explains the fix in plain language. Invoked only by the /advance-biome command.
disable-model-invocation: true
---

# Advance Biome

## What this is for

Air uses code from another project called Biome. `Cargo.toml` names one exact Biome commit, and Air uses Biome as it was at that moment. We call that commit "the pin".

Air is thousands of Biome changes behind. Catching up all at once would be too big to review, so each run takes one small step:

1. Move the pin forward until we hit a Biome change Air can't handle. We call it "the breaking change".
2. Fix Air so it works with the breaking change.
3. Explain the fix in plain words, so the user can review it and open a pull request.

The user runs this again after each pull request is merged.

## Rules

- Don't commit or push. The user does that.
- Only fix the breaking change, and catch up the copied code in Step 5. Don't clean up anything else.
- When copying Biome code, keep Biome's names, comments, and layout. Only swap in Air's language name, and only differ where Air's style rules or newer Rust require it.
- Never accept a change in how Air formats R code just to make tests pass. Stop and show the user a before-and-after example.
- If the right fix is unclear or large, stop and ask. Don't guess.
- Talk to the user the way Step 6 says.

## Step 1: Get ready

Run these from the Air folder. `origin` is Air on GitHub (posit-dev/air).

1. `git status --short --untracked-files=no` should print nothing. If it does, stop and ask the user what to do. Untracked files are fine.
2. `git fetch origin`
3. If `git branch --list feature/advance-biome` prints anything, it's left over from last time. Stop and ask whether to delete it.
4. Make the branch. `--no-track` stops a later push from going to `main`:
   ```
   git switch --no-track -c feature/advance-biome origin/main
   ```
5. Find the pin. This should print exactly one line. If not, stop and ask:
   ```
   grep -o 'biomejs/biome", rev = "[0-9a-f]*"' Cargo.toml | sort -u
   ```

## Step 2: Get Biome's history

Below, `<temp>` is your scratch folder. This download skips file contents, so it takes seconds:

```
git clone --quiet --filter=blob:none --no-checkout https://github.com/biomejs/biome.git <temp>/biome
```

List every change on Biome's main line since the pin, oldest first:

```
git -C <temp>/biome log --first-parent --reverse --format='%H %cs %s' <pin>..origin/main > <temp>/candidates.txt
```

`--first-parent` treats a big batch of changes (a line like "Merge pull request #1234 from biomejs/next") as one line.

If the file is empty, Air is caught up. Tell the user and stop.

## Step 3: Find the breaking change

[scripts/check-air.sh](scripts/check-air.sh) does one try. It points Air at a Biome commit, runs Air's checks, and ends with one line:

```
bash <this skill's folder>/scripts/check-air.sh --pin <full commit>
```

| Last line | Meaning |
|---|---|
| `RESULT: WORKS` | Air is fine with this commit. |
| `RESULT: NEEDS AIR CHANGES` | Something failed. The log it names says what. |
| `RESULT: COULD NOT RUN` | The script broke before checking anything. Fix that and try again. |

A try can take many minutes, the first one most of all. Give it at least 30 minutes.

Biome often needs a newer version of some shared library (like `cfg-if`), and Cargo won't bump it in `Cargo.lock` on its own. That isn't a breaking change. The script bumps just those libraries and prints a "Bumping ..." line for each. Don't ask the user about it, but list the bumps in "What I changed in Air". If the script still says COULD NOT RUN, find out why before going on.

Before you trust a "NEEDS AIR CHANGES", check the log to make sure Biome caused it: a build error mentioning a `biome_` package, or a test whose expected output changed. A dropped network connection or a flaky test doesn't count. If in doubt, run the same try again.

How to search:

1. Try the newest commit (last line of `candidates.txt`) first. If it works, keep that pin, skip Step 4, and tell the user Air has caught up.
2. Otherwise, halve. Try the commit halfway between the newest one that worked and the oldest one that failed. Keep halving until you find a commit that works right before one that fails. About 3,000 lines takes about 12 tries. This works because once Biome breaks something, every later commit stays broken.
3. Show the user a table of every try (line number, commit, result) as you go, so an interrupted search can pick up where it left off.

The first commit that fails is the breaking change, and Air must end up pinned to it. If your last try was a different commit, run `--pin <breaking change>` once more. Expect NEEDS AIR CHANGES.

## Step 4: Understand and fix

Find out what the breaking change did and why:

- What it says and which files it touched: `git -C <temp>/biome show --stat <commit>`
- What it changed in the parts Air uses (the `biome_` packages in Air's `Cargo.lock`): `git -C <temp>/biome show <commit> -- crates/biome_rowan crates/biome_formatter ...`
- Why: the description usually ends with `(#1234)`. Read https://github.com/biomejs/biome/pull/1234.
- If it's a big batch from `biomejs/next`, find the change inside it that matters: `git -C <temp>/biome log --oneline <commit>^1..<commit>^2 -- crates/biome_rowan ...`. Keep the pin on the batch, but tell the user which change caused the problem.

Common kinds of breaking changes:

| What Biome did | What Air usually needs |
|---|---|
| Renamed, moved, or removed something Air uses | Switch Air to the new name or new way. |
| Needs a newer Rust | Raise `channel` in `rust-toolchain.toml` and `rust-version` in `Cargo.toml`. |
| Prints messages or errors differently | Update Air's saved test output, once the new output looks right. |
| Makes formatting decisions differently | Air may format R code differently. See the rules at the top. |

The files in `crates/air_r_syntax/src/generated/` are written by a program in `xtask/codegen/`. Don't edit them. Change the program and rerun `just gen-grammar`.

Many tests compare against saved output. To update it: `just test-insta` then `cargo insta accept` for the formatter, `just test-snapshot-accept` for the parser. Look at what changed before keeping it.

Then check Air as it is now, without `--pin`:

```
bash <this skill's folder>/scripts/check-air.sh
```

Repeat until `RESULT: WORKS`. If you can't get there, stop and tell the user what's still failing.

Before moving on, confirm:

- [ ] You're on `feature/advance-biome`
- [ ] Every Biome line in `Cargo.toml` names the breaking change
- [ ] `check-air.sh` without `--pin` says `RESULT: WORKS`
- [ ] Nothing is committed

## Step 5: Catch up code Air copied from Biome

Some Air code is a copy of Biome code, not something Air gets from Biome's packages. When Biome fixes a bug in its version, Air still builds, so Step 3 never notices. This step keeps the copies from falling behind.

Don't mark copies with comments in Air's files. The list lives here:

| Air file | Biome file |
|---|---|
| `crates/air_r_formatter/src/verbatim.rs` | `crates/biome_json_formatter/src/verbatim.rs` |
| `FormatNodeRule` and `FormatBogusNodeRule` in `crates/air_r_formatter/src/lib.rs` | The same traits in `crates/biome_json_formatter/src/lib.rs` |
| `crates/air_r_formatter/src/joiner_ext.rs` (in part) | `crates/biome_formatter/src/comments/builder.rs` |
| `crates/air_formatter_test/src/snapshot_builder.rs`, `spec.rs`, `check_reformat.rs` (intentional difference: Air's `with_input()` doesn't add a newline after the input, so the Input section matches the file byte for byte, like the Output section does) | The same files in `crates/biome_formatter_test/src/` |
| `crates/aether_lsp_utils/src/proto.rs`, `proto/from_proto.rs`, `proto/to_proto.rs` | `crates/biome_lsp_converters/src/lib.rs`, `from_proto.rs`, `to_proto.rs` |
| `text_non_whitespace_range` in `crates/lsp/src/handlers_format.rs` | The same function in `crates/biome_formatter/src/lib.rs` |
| `crates/biome_ungrammar/` (Air keeps its own copy, so the pin never moves it) | `crates/biome_ungrammar/` |
| `xtask/codegen/src/` (in part; `r_kinds_src.rs` is Air's own) | `xtask/codegen/src/` |

After catching up `xtask/codegen/` or `crates/biome_ungrammar/`, rerun `just gen-grammar`. If the files in `generated/` change, mention it in the summary.

If Biome moves or renames one of these files, follow it (`git -C <temp>/biome log --follow`) and update the table. If you find a copy that isn't listed, or the fix creates a new one, add a row and tell the user.

For each copy, see what Biome changed between the old pin (Step 1) and the new pin:

```
git -C <temp>/biome log --format='%h %cs %s' <old pin>..<new pin> -- <biome file>
git -C <temp>/biome diff <old pin> <new pin> -- <biome file>
```

If the breaking change made Air copy a new file, copy it as it was at the new pin. Then run the same `log` with `<new pin>..origin/main` to see what Biome changed in it later.

Sort each change:

| Kind of change | What to do |
|---|---|
| Bug fix or behavior change that works at the current pin | Bring it over. |
| Only comments, typos, or names | Bring it over. |
| Needs something the current pin doesn't have yet | Skip it. A later run will catch it as a breaking change. |
| Changes how Air formats R code | Stop and show the user a before-and-after example. |
| Doesn't fit Air (like JSON-only syntax) | Skip it, and say why in the summary. |

Then compare each copy with Biome's version. The only differences left should be the language name, Air's import style, and changes you skipped on purpose. Run `check-air.sh` (without `--pin`) again and make sure it still says `RESULT: WORKS`.

## Step 6: Tell the user

Write like you're explaining to a smart friend who has never seen Air's or Biome's code. Use short sentences and everyday words. If you have to use a technical term, explain it in a few words right there. About 15 lines or fewer.

Use these headings, in this order:

- **Where we are:** The Biome commit Air uses now, its date, and a link like https://github.com/biomejs/biome/commit/<commit>. How many lines of `candidates.txt` we moved past, and how many are left (`git -C <temp>/biome rev-list --count --first-parent <breaking change>..origin/main`).
- **What Biome changed:** One to three sentences, with a link to the pull request.
- **Why Air cared:** One or two sentences.
- **What I changed in Air:** Each changed file, with one plain sentence about why. Include copies caught up in Step 5, and Biome changes you skipped on purpose.
- **How I checked:** Usually "All of Air's checks pass."
- **Worth a closer look:** Anything you're unsure about, or anything Air users might notice. Otherwise "Nothing".
- **Suggested pull request title:** One line.

A made-up example:

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
