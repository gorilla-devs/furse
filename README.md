# Furse

[![github badge](https://img.shields.io/badge/GitHub-Furse-informational?style=for-the-badge&logo=github&labelColor=555555)](https://github.com/theRookieCoder/furse/)
[![crates badge](https://img.shields.io/crates/v/furse?logo=rust&style=for-the-badge)](https://crates.io/crates/furse/)
[![docs.rs](https://img.shields.io/docsrs/furse?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&style=for-the-badge)](https://docs.rs/furse/)

Furse is a simple library for using the [CurseForge API](https://docs.curseforge.com/) in Rust projects.
It uses [Reqwest](https://docs.rs/reqwest/) as its HTTPS client and deserialises responses to strongly typed structs using [SerDe](https://serde.rs/).

## Features

This crate includes the following:
- API calls:
  - Get mod by mod ID (<https://docs.curseforge.com/#get-mod>)
  - Get mod's HTML description by ID (<https://docs.curseforge.com/#get-mod-description>)
  - Get mod's files by mod ID (<https://docs.curseforge.com/#get-mod-files>)
  - Get file by the mod ID and file ID (<https://docs.curseforge.com/#get-mod-file>)
  - Get file's HTML changelog by mod ID and file ID (<https://docs.curseforge.com/#get-mod-file-changelog>)
  - Download a file from a `File`
  - Download a file by mod ID and file ID (<https://docs.curseforge.com/#get-mod-file-download-url>)
- Schemas and their dependant schemas:
  - Mod <https://docs.curseforge.com/#tocS_Mod>
  - File <https://docs.curseforge.com/#tocS_File>

This crate uses [Rustls](https://docs.rs/rustls/) rather than OpenSSL, because OpenSSL is outdated and slower.
