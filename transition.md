# Transition to the Next Day

Follow these steps in order to create a new day's crate and update the feature branch. Enter your SSH password in the terminal when prompted.

## Variables

- `<CURRENT>`: current day number (e.g., 8)
- `<CURRENT2>`: current day, two digits (e.g., 08)
- `<NEW>`: new day number (e.g., 9)
- `<NEW2>`: new day, two digits (e.g., 09)

## Steps

1. Pull the latest `master` from origin.
2. Checkout `feature/day<CURRENT2>`.
3. Rebase `feature/day<CURRENT2>` onto `master`.
4. Rename `feature/day<CURRENT2>` to `feature/day<NEW2>`.
5. Confirm you are on `feature/day<NEW2>`.
6. In root `Cargo.toml`, add `day<NEW2>` after `day<CURRENT2>` in the `[workspace.members]` list. Ensure `day<CURRENT2>` ends with a comma.
7. Copy folder `day<CURRENT2>` to `day<NEW2>`.
8. In `day<NEW2>`:
   - Rename all input files from `day<CURRENT2>` to `day<NEW2>`, and clear their contents.
   - In `main.rs`:
     - Keep only the banner print in `main()`. Remove all other code and functions except the initial comment.
     - Update any `<CURRENT>`/`<CURRENT2>` references to `<NEW>`/`<NEW2>`.
   - In `Cargo.toml`:
     - Update any `<CURRENT>`/`<CURRENT2>` references to `<NEW>`/`<NEW2>`.
     - Remove `default = ["part2"]` line.
