# rbw

This is an Fork of the unofficial command line client [rbw](https://github.com/doy/rbw) for
[Bitwarden](https://bitwarden.com/). This Fork add self signed ca certificate support.

## Installation

Uninstall rbw (rbw-agent). Then

```bash
git clone [THIS_REPO_URL]
cd rbw
cargo install rbw --path .
```

Finally add `$HOME/.cargo/bin` to your `PATH`. Make sure you have [`pinentry`](https://www.gnupg.org/related_software/pinentry/index.en.html) program installed.

## Convert certificate.crt to certificate.der

```bash
openssl x509 -outform der -in self-signed-ca-cert.crt -out self-signed-ca-cert.der
```

## Configuration

Example configuration located in `$HOME/.config/rbw/config.json`:

```json
{
  "email": "bitwarden@local",
  "base_url": "https://bitwarden.local:8080",
  "identity_url": null,
  "lock_timeout": 3600,
  "pinentry": "pinentry",
  "root_certificate": "/home/arch/.certs/self-signed-ca-cert.der"
}
```

Configuration options can be set using the `rbw config set [KEY] [VALUE]` command. Available configuration options:

- `email`: The email address to use as the account name when logging into the Bitwarden server. Required.
- `base_url`: The URL of the Bitwarden server to use. Defaults to the official server at `https://api.bitwarden.com/` if unset.
- `identity_url`: The URL of the Bitwarden identity server to use. If unset, will use the `/identity` path on the configured `base_url`, or
  `https://identity.bitwarden.com/` if no `base_url` is set. - `lock_timeout`: The number of seconds to keep the master keys in memory for
  before requiring the password to be entered again. Defaults to `3600` (one hour).
- `pinentry`: The [pinentry](https://www.gnupg.org/related_software/pinentry/index.html)
  executable to use. Defaults to `pinentry`.
- `root_certificate`: absolute path to the self signed ca certificate in DER format.

## Usage

See [rbw](https://github.com/doy/rbw) Repository.
