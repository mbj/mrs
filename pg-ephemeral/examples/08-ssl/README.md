# 08 — SSL with verify-full

`ssl_config` enables TLS for the instance. pg-ephemeral generates a fresh CA
and server certificate at boot, mounts them into the container, and exports
`PGSSLMODE=verify-full` (plus `PGSSLROOTCERT`) so the host's `psql` and any
process started via `run-env` validate the cert against the generated CA.

This is the production-like path: trust mode is **never** used.

## Config

```toml
[ssl_config]
hostname = "localhost"
```

The `hostname` must match what the client connects to (it goes into the
server cert's CN/SAN). For pg-ephemeral that's `localhost` in almost every
case.

## Run

```sh
cd examples/08-ssl

# psql will negotiate TLS and validate against the generated CA.
pg-ephemeral

# Inside psql, confirm the connection is encrypted:
postgres=# SHOW ssl;             -- on
postgres=# \conninfo             -- says SSL connection (...)
```

## CLI override

You can enable SSL ad-hoc without putting it in the config file:

```sh
pg-ephemeral --no-config-file --ssl-hostname localhost
```

`--ssl-hostname` overrides whatever (if anything) `ssl_config` had.

## What this demonstrates

- `ssl_config` in TOML, with the required `hostname` field.
- The `--ssl-hostname` CLI override for the no-config path.
- That pg-ephemeral does verify-full by default — there's no opt-in trust
  mode, on purpose.
