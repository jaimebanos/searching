# Company Logo Downloader

This Rust project reads a JSON file containing companies and their ISINs, downloads the logos for each company, and generates a new JSON file with details of the processed companies and the URL of their logos. It avoids re-downloading logos for companies that have already been processed.

## Features

- Reads a JSON file with company names and ISINs.
- Downloads company logos from the web.
- Generates a JSON file with processed companies and the URL of their logos.
- Ensures that logos for each company are only downloaded once.

## Requirements

- [Rust](https://www.rust-lang.org/) (version 1.56 or higher)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/jaimebanos/searching.git
    cd searching
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

## Configuration

### Input JSON Format

The input JSON file (`content.json`) should be in the following format:

```json
[
  {
    "company_name": "Example Corp",
    "isin": "US1234567890",
    "name": "Roche",
    "website": "https://www.roche.com"
  },
  {
    "company_name": "Another Company",
    "isin": "US0987654321",
    "name": "Roche",
    "website": "https://www.roche.com"
  }
]
```

### Tasks
- [] Api services
- [] Catch errors and notified
