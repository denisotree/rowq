# Scoop bucket

The bucket is the repository **`denisotree/scoop-rowq`**; it holds `bucket/rowq.json`, a copy
of `rowq.json` from this directory. It was made from `ScoopInstaller/BucketTemplate` with the
Excavator workflow removed: the manifest is pushed by hand at release time, as the Homebrew
formula is, so the copy here and the one in the bucket never disagree.

## User installation

```powershell
scoop bucket add rowq https://github.com/denisotree/scoop-rowq
scoop install rowq
```

Installs `rowq` and `rowq-mcp` (the MCP server alone, which opens no console window).

## Updating the manifest for a release

`/release` sets `version`, `url` and `extract_dir` and resets `hash`; once the GitHub
Release is built, the hash of `rowq-vX.Y.Z-x86_64-pc-windows-msvc.zip` comes from its
`SHA256SUMS`. Copy the manifest to `bucket/rowq.json` in the bucket and push to `main`.

## Testing the manifest locally

On Windows, from a clone of the bucket:

```powershell
scoop install .\bucket\rowq.json
rowq --version
```
