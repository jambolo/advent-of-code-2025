# Transition to the Next Day

Follow these steps in order to create a new day's crate and update the feature branch. Enter your SSH password in the terminal when prompted.

## Variables

- `<LAST>`: previous day number (e.g., 8)
- `<NEXT>`: next day number (e.g., 9)
- `<LAST2>`: previous day, two digits (e.g., 08)
- `<NEXT2>`: next day, two digits (e.g., 09)

## Steps

1. Pull the latest `master` from origin.
2. Checkout `feature/day<LAST2>`.
3. Rebase `feature/day<LAST2>` onto `master`.
4. Rename `feature/day<LAST2>` to `feature/day<NEXT2>`.
5. Confirm you are on `feature/day<NEXT2>`.
6. In root `Cargo.toml`, add `day<NEXT2>` after `day<LAST2>` in the `[workspace.members]` list. Ensure `day<LAST2>` ends with a comma.
7. Copy folder `day<LAST2>` to `day<NEXT2>`.
8. In `day<NEXT2>`:
   - Rename all input files from `day<LAST2>` to `day<NEXT2>`, and clear their contents.
   - In `main.rs`:
     - Keep only the banner print in `main()`. Remove all other code and functions except the initial comment.
     - Update any `<LAST>`/`<LAST2>` references to `<NEXT>`/`<NEXT2>`.
   - In `Cargo.toml`:
     - Update any `<LAST>`/`<LAST2>` references to `<NEXT>`/`<NEXT2>`.
     - Remove `default = ["part2"]` line.
