# Security policy

## Supported versions

Bloom is pre-1.0 and fixes land on the latest release only. If you are running
an older build, update before reporting.

## Reporting a vulnerability

Report privately through GitHub, using **Report a vulnerability** on the
[Security tab](https://github.com/nnmarcoo/bloom/security/advisories/new). That
keeps the report between you and the maintainers until a fix ships.

Please do not open a public issue for a vulnerability.

Include the version (`bloom --version`), the platform, and the steps or file
that trigger it. A sample file that reproduces the problem is the most useful
thing you can attach.

## Scope

Bloom decodes untrusted files: images, camera RAW, and with the `av` feature,
video and audio. Bugs in that decoding are the ones worth reporting, especially
memory corruption reachable from a crafted file.

Most decoding is done by third-party crates. A vulnerability in one of those is
better reported upstream, through the [RustSec advisory
database](https://rustsec.org) or the crate's own tracker. `cargo audit` runs
here daily against the dependency tree.

Bloom makes no network requests and sends no telemetry, so there is no server
side to this project.
