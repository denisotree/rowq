# Arch Linux AUR packaging

Two AUR packages are built from this directory:

| AUR package | Files here | Builds |
|---|---|---|
| [`rowq`](https://aur.archlinux.org/packages/rowq) | `PKGBUILD`, `.SRCINFO` | from the tag tarball |
| [`rowq-bin`](https://aur.archlinux.org/packages/rowq-bin) | `PKGBUILD-bin`, `.SRCINFO-bin` | from the release archive |

AUR reads `.SRCINFO`, not `PKGBUILD`, so both are copied together. `-bin` files
go in as plain `PKGBUILD` and `.SRCINFO`.

## Next to tuitab

tuitab, which rowq continues, stays on the AUR at 0.11.2 (`tuitab`, `tuitab-bin`). The
binaries differ (`tuitab`, `ttab`, `ttb` against `rowq`), so the packages do not
conflict and install side by side.

## Publishing a release

```sh
git clone ssh://aur@aur.archlinux.org/rowq.git tmp/aur-rowq
cp packaging/aur/PKGBUILD packaging/aur/.SRCINFO tmp/aur-rowq/
cd tmp/aur-rowq && git add PKGBUILD .SRCINFO && git commit -m "Update to <version>" && git push
```

The source package's `sha256sums` is the hash of the GitHub tag tarball
(`archive/refs/tags/v<version>.tar.gz`); the `-bin` hashes come from the
release's `SHA256SUMS`.

## Checking the files

```sh
cd packaging/aur
makepkg --printsrcinfo | diff - .SRCINFO
makepkg -si   # build and install locally
```
