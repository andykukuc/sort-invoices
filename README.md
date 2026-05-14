# sort-invoices

A Rust binary that automatically organizes invoice files into subfolders by type. Designed to run as a cron job or systemd service on a Linux server hosting a Samba share.

## What it does

Scans every client folder under the invoice base directory and moves files into typed subfolders:

| Extension | Destination |
|-----------|-------------|
| `.pdf` | `PDF/` |
| `.xlsm`, `.xlsx`, `.xls` | `Excel/` |
| `.doc`, `.docx` | `Word/` |

Hidden files (`.filename`) and temp files (`~filename`) are skipped automatically. Unknown extensions are logged and left in place.

## Example

Before:
```
invoices/
  Konrad/
    20260514_00042_Freightliner_FL505.xlsm
    20260514_00042_Freightliner_FL505.pdf
```

After:
```
invoices/
  Konrad/
    Excel/
      20260514_00042_Freightliner_FL505.xlsm
    PDF/
      20260514_00042_Freightliner_FL505.pdf
```

## Configuration

Edit the constants at the top of `src/main.rs`:

```rust
const BASE: &str = "/mnt/samba_pool/samba/invoices";
const LOG_PATH: &str = "/var/log/sort-invoices.log";
```

## Build

```bash
git clone git@github.com:andykukuc/sort-invoices.git
cd sort-invoices
cargo build --release
```

Binary: `target/release/sort-invoices`

## Usage

```bash
./target/release/sort-invoices
```

Sample log output:
```
[2026-05-14 02:00:01] --- sort-invoices started ---
[2026-05-14 02:00:01] MOVED: Konrad/invoice.pdf -> PDF/
[2026-05-14 02:00:01] MOVED: Konrad/invoice.xlsm -> Excel/
[2026-05-14 02:00:01] SKIPPED: Konrad/notes.txt (unknown extension: txt)
[2026-05-14 02:00:01] --- sort-invoices complete ---
```

## Automating with cron

```cron
0 3 * * * /usr/local/bin/sort-invoices >> /var/log/sort-invoices.log 2>&1
```

## License

MIT
