# stc-cli

Rust command-line helper for Canonical XML 1.1 canonicalization used by the STC taxpayer client.

## Purpose

The tool canonicalizes UBL/XAdES XML before invoice hashing and digital signing.

## Usage

```bash
stc-cli input.xml output.xml
```

Example:

```bash
stc-cli /app/work/input.xml /app/work/output.xml
```

## Canonicalization Mode

- Canonical XML 1.1
- Comments excluded
- UTF-8 XML input/output

## Role in STC Client

```text
UBL XML generation
-> C14N 1.1 canonicalization
-> SHA-256 hash
-> XAdES signature creation
-> STC clearance/reporting submission
```

## Recommended Tests

The test suite covers:

- comment removal
- attribute ordering
- namespace ordering
- empty element expansion
- UBL invoice canonicalization
- SignedInfo canonicalization
- SignedProperties canonicalization
- UTF-8 Arabic text handling
- stable SHA-256 hashes for XML differences normalized by C14N
- invalid XML returning a non-zero exit code

Canonical XML preserves text nodes, including indentation whitespace between elements. Whitespace-insensitive invoice hashing requires a separate XML normalization step before canonicalization.
