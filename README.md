# Merge My GPX

## Futures (possible) development

- Komoot cannot import files that contains too many points. We may add a command to mitigate this issue.
    - Split into several tracks?
    - Keep 1 out of 2 points in the tracks?
- When calling the `merge-all` command, the presence of `merged.gpx` in the directory is probably an issue.
    - It probably means that the command has been called several times on the same directory.
    - Merging again will result in a wrong file (the track will be made twice).
    - Solution?
        - Exclude file and overwrite
        - Abort
        - Ask user
- If `merge` is called with a single file, we may use a more precise name than just `merged.gpx`
- Add `--smart` option to `merge` and `merge-all` to try to guess the order of the files when merging.
