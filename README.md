# pg_cranelift

A toy JIT backend for postgres using [cranelift](https://cranelift.dev/).

## Some useful commands (I'll fill in the README later)

```bash
sudo apt install postgresql-14 postgresql-server-dev-14
```

```bash
make clean && make && make install
```

```bash
pg_ctlcluster 14 ccluster restart
```

```bash
psql -h /tmp -p 5433 -d postgres -U <user>
```

```bash
cargo install --force cbindgen
```
