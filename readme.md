# ssh-known-hosts

[![Crates.io](https://img.shields.io/crates/v/ssh-known-hosts)](https://crates.io/crates/ssh-known-hosts)

Connect to a host from a defined pool of hosts by an easy to remember name.

## Usage

```bash
$ ssh-known-hosts connect uwm-ale

$ ssh-known-hosts list

Local              Real Host
----------------------------
uwm-ale            <hidden>
minecraft-linode   <hidden>
...

```

## Configuration

The configuration file uses a basic yaml structure:

```yaml
hosts:
  # hosts is a map from local-names to host specifications
  google:
    user_name: root
    host: google.com
    port: 22
  # You also have the option to leave the port field off for a
  # default value of 22
  facebook:
    user_name: root
    host: 192.168.1.1
```

The utility looks for `~/.ssh_known_hosts.yml` by default,
but this can be override when calling. run `ssh-known-hosts help` for more information.

## Installation

You need cargo to install `ssh-known-hosts` go to [rustup.rs](https://rustup.rs/)
to see about downloading cargo.

```bash
cargo install ssh-known-hosts
```

--------

### Features
- [x] Keep a configurable list of hosts that can be ssh'd into
- [ ] Use scp to download a file from host
- [ ] Use scp to uplead a file to a host

Maintained by oldwomanjosiah (jhilden13@gmail.com)
