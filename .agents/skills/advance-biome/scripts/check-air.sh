#!/usr/bin/env bash
#
# Runs the same checks Air's CI runs, and optionally points Air at a different
# Biome commit first.
#
# Usage, from anywhere inside the Air folder:
#
#   check-air.sh --pin <biome commit>   Point Air at that Biome commit, then check.
#   check-air.sh                        Check Air exactly as it is right now.
#
# The last line printed always starts with "RESULT:". Exit codes:
#
#   0  Every check passed.
#   1  A check failed. The log folder printed above has the full details.
#   2  The script couldn't run at all (bad input, unexpected changes, ...).
#
# With --pin, the files git tracks must have no changes other than Cargo.toml
# and Cargo.lock. New files git doesn't track yet are fine. Cargo.lock is reset
# to the committed version first, then only the libraries Cargo reports as
# conflicting with the new Biome commit are bumped. When the script
# finishes, it puts back the tracked files the checks touched and removes only
# the new files the checks created, so the next try starts clean. Cargo.toml
# and Cargo.lock keep their changes, so Air stays pointed at the commit you
# tried.
#
# Without --pin, nothing is put back. Files the checks regenerate are left in
# place so you can look at them.

set -uo pipefail

could_not_run() {
  echo "RESULT: COULD NOT RUN ($1)"
  exit 2
}

pin=""
while [ $# -gt 0 ]; do
  case "$1" in
    --pin)
      [ $# -ge 2 ] || could_not_run "--pin needs a Biome commit after it"
      pin="$2"
      shift 2
      ;;
    -h | --help)
      awk 'NR > 2 && /^#/ { sub(/^# ?/, ""); print; next } NR > 2 { exit }' "$0"
      exit 0
      ;;
    *)
      could_not_run "unknown argument: $1"
      ;;
  esac
done

repo=$(git rev-parse --show-toplevel 2>/dev/null) || could_not_run "not inside the Air folder"
cd "$repo" || could_not_run "can't enter $repo"

grep -q 'git = "https://github.com/biomejs/biome", rev = "' Cargo.toml 2>/dev/null ||
  could_not_run "Cargo.toml has no Biome lines in the expected shape"

command -v perl >/dev/null || could_not_run "perl is needed to edit Cargo.toml"
cargo nextest --version >/dev/null 2>&1 || could_not_run "cargo-nextest is not installed"

keep=(':(exclude)Cargo.toml' ':(exclude)Cargo.lock')

if [ -n "$pin" ]; then
  [[ "$pin" =~ ^[0-9a-f]{40}$ ]] || could_not_run "give the full 40-character commit, not '$pin'"

  if [ -n "$(git status --porcelain --untracked-files=no -- . "${keep[@]}")" ]; then
    git status --short --untracked-files=no -- . "${keep[@]}"
    could_not_run "there are changes besides Cargo.toml and Cargo.lock"
  fi

  untracked_before=$(git ls-files --others --exclude-standard | sort)

  # Start every try from the committed lock file, so libraries bumped for an
  # earlier (possibly newer) try don't leak into this one
  git checkout -q HEAD -- Cargo.lock

  perl -pi -e 's{(git = "https://github.com/biomejs/biome", rev = ")[0-9a-f]+(")}{${1}'"$pin"'${2}}g' Cargo.toml

  # Make sure every Biome line got the new commit. If one is written in a
  # different shape, the replacement above would silently miss it.
  biome_lines=$(grep -c 'github.com/biomejs/biome' Cargo.toml)
  pinned_lines=$(grep -c "github.com/biomejs/biome\", rev = \"$pin\"" Cargo.toml)
  [ "$biome_lines" = "$pinned_lines" ] ||
    could_not_run "only $pinned_lines of $biome_lines Biome lines in Cargo.toml were updated"

  # Put back anything the checks wrote, except the pin itself. Only remove new
  # files that weren't there when we started.
  restore() {
    git checkout -q -- . "${keep[@]}"
    comm -13 <(printf '%s\n' "$untracked_before") <(git ls-files --others --exclude-standard | sort) |
      while IFS= read -r file; do
        [ -n "$file" ] && rm -f -- "$file"
      done
  }
  trap restore EXIT

  # Biome often raises the minimum version of a shared crates.io library, and
  # Cargo won't move a locked library on its own. That isn't something Air needs
  # to fix, so bump only the libraries Cargo names as conflicting. A bare
  # `cargo update` would update everything, so it's never run without `-p`.
  bumps=()
  resolved=""
  for _ in $(seq 1 50); do
    if [ ${#bumps[@]} -eq 0 ]; then
      out=$(cargo metadata --format-version 1 2>&1 >/dev/null) && { resolved=yes; break; }
    else
      out=$(cargo update "${bumps[@]}" 2>&1) && { resolved=yes; break; }
    fi
    crate=$(printf '%s\n' "$out" |
      grep -o 'previously selected package `[^`]*`' | head -n 1 |
      sed -E 's/.*`([^ ]+) v([^`]+)`/\1@\2/')
    if [ -z "$crate" ] || [[ " ${bumps[*]} " == *" $crate "* ]]; then
      printf '%s\n' "$out" | tail -n 20
      could_not_run "Cargo.lock can't be resolved for this Biome commit"
    fi
    echo "Bumping $crate in Cargo.lock, because Biome needs a newer version"
    bumps+=(-p "$crate")
  done
  [ -n "$resolved" ] || could_not_run "too many Cargo.lock conflicts to bump"

  # Don't write `.snap.new` files while searching, nobody will look at them
  export INSTA_UPDATE=no

  echo "Air now points at Biome commit $pin"
fi

logs=$(mktemp -d "${TMPDIR:-/tmp}/air-check.XXXXXX")
echo "Logs: $logs"

# Fingerprint of every change in the folder, used to notice regenerated files
fingerprint() {
  { git diff HEAD; git ls-files --others --exclude-standard; } | git hash-object --stdin
}

failed() {
  echo "  FAILED. Last lines of $logs/$1.log:"
  tail -n 40 "$logs/$1.log" | sed 's/^/    /'
  echo "RESULT: NEEDS AIR CHANGES (failed step: $1, full log: $logs/$1.log)"
  exit 1
}

step() {
  local name="$1"
  shift
  echo "== $name: $*"
  "$@" >"$logs/$name.log" 2>&1 || failed "$name"
  echo "  passed"
}

step format cargo fmt --all --check
step clippy cargo clippy --workspace --all-targets -- -D warnings
step tests cargo nextest run --workspace --no-fail-fast

before=$(fingerprint)
step grammar cargo run -q -p xtask_codegen -- grammar r
step parser-snapshots cargo run -q -p xtask_accept
after=$(fingerprint)

if [ "$before" != "$after" ]; then
  git status --short >"$logs/regenerated.log"
  git diff >>"$logs/regenerated.log"
  echo "== regenerated files changed:"
  git status --short | sed 's/^/    /'
  failed regenerated
fi

echo "RESULT: WORKS"
