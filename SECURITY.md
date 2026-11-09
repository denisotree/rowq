# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| latest  | Yes       |

Only the latest release receives security fixes.

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Please report them privately via email to the maintainer listed in `Cargo.toml`,
or use [GitHub's private vulnerability reporting](https://github.com/denisotree/rowq/security/advisories/new).

Include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

You will receive a response within 7 days. After the fix is released, the vulnerability
will be disclosed publicly via a GitHub Security Advisory.

## Scope

rowq reads local files and renders them in the terminal. It does not make network
requests. With `--mcp` it also runs as an MCP server for an AI assistant: it talks over
stdin and stdout only and opens no network port. Writing to a database through it needs
`--mcp-write`, and every change is planned, shown and applied as one transaction.

The primary security concern is malicious file content that could trigger unexpected
behaviour (e.g. path traversal in file loaders, integer overflow in rendering, or
arbitrary code execution via malformed data), and, for the MCP server, a request that
reads or writes outside what the user asked for.

## Dependencies

Security advisories for dependencies are tracked automatically via
[`cargo audit`](https://github.com/rustsec/rustsec) in CI. You can run it locally:

```sh
cargo install cargo-audit
cargo audit
```
