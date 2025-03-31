# Furse

[![Made with Rust](https://img.shields.io/badge/Made_with-Rust-b11522?labelColor=e82833&logo=rust)](https://www.rust-lang.org)
[![GitHub - Furse](https://img.shields.io/badge/GitHub-Furse-8E47FE?logo=github)](https://github.com/gorilla-devs/furse)
[![license - MIT](https://img.shields.io/github/license/gorilla-devs/furse)](https://github.com/gorilla-devs/furse/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/furse?logo=rust)](https://crates.io/crates/furse)
[![docs.rs](https://img.shields.io/docsrs/furse/latest?label=docs.rs&logo=docsdotrs)](https://docs.rs/furse)

Furse is a simple library for using the [CurseForge REST API](https://docs.curseforge.com/rest-api#accessing-the-service) in Rust.
It uses [reqwest](https://docs.rs/reqwest) for sending requests and deserialising responses to strongly typed structs using [Serde](https://serde.rs).

## Features

- API calls
  - Get mod by mod ID (<https://docs.curseforge.com/rest-api/#get-mod>)
  - Get mod's HTML description by ID (<https://docs.curseforge.com/rest-api/#get-mod-description>)
  - Get mod's files by mod ID (<https://docs.curseforge.com/rest-api/#get-mod-files>)
  - Get file by the mod ID and file ID (<https://docs.curseforge.com/rest-api/#get-mod-file>)
  - Get file's HTML changelog by mod ID and file ID (<https://docs.curseforge.com/rest-api/#get-mod-file-changelog>)
  - Download a file from a `File`
  - Download a file by mod ID and file ID (<https://docs.curseforge.com/rest-api/#get-mod-file-download-url>)
- Schemas and their dependant schemas
  - Mod <https://docs.curseforge.com/#tocS_Mod>
  - File <https://docs.curseforge.com/#tocS_File>
