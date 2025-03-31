# Furse

[![Made with Rust](https://img.shields.io/badge/Made_with-Rust-b11522?labelColor=e82833&logo=rust)](https://www.rust-lang.org)
[![GitHub - Furse](https://img.shields.io/badge/GitHub-Furse-8E47FE?logo=github)](https://github.com/gorilla-devs/furse)
[![license - MIT](https://img.shields.io/github/license/gorilla-devs/furse)](https://github.com/gorilla-devs/furse/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/furse?logo=rust)](https://crates.io/crates/furse)
[![docs.rs](https://img.shields.io/docsrs/furse/latest?label=docs.rs&logo=docsdotrs)](https://docs.rs/furse)

Furse is a simple library for using the [CurseForge REST API](https://docs.curseforge.com/rest-api#accessing-the-service) in Rust.
It uses [reqwest](https://docs.rs/reqwest) for sending requests and deserialising responses to strongly typed structs using [Serde](https://serde.rs).

## Features

- Strongly typed structures for API responses
- Useful examples in the method documentations
- Implementations for the following API calls
  - [Get mod by mod ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mod_file) ([official documentation](https://docs.curseforge.com/rest-api#get-mod))
  - [Get mods by mod IDs](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mods) ([official documentation](https://docs.curseforge.com/rest-api#get-mods))
  - [Get HTML description by mod ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mod_description) ([official documentation](https://docs.curseforge.com/rest-api#get-mod-description))
  - [Get all of the mod's files by mod ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mod_files) ([official documentation](https://docs.curseforge.com/rest-api#get-mod-files))
  - [Get file by mod ID and file ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mod_file) ([official documentation](https://docs.curseforge.com/rest-api#get-mod-file))
  - [Get files by file IDs](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_files) ([official documentation](https://docs.curseforge.com/rest-api#get-files))
  - [Get file's HTML changelog by mod ID and file ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_mod_file_changelog) ([official documentation](https://docs.curseforge.com/rest-api#get-mod-file-changelog))
  - [Get file's download URL by mod ID and file ID](https://docs.rs/furse/latest/furse/struct.Furse.html#method.file_download_url) ([official documentation](https://docs.curseforge.com/rest-api#get-mod-file-download-url))
  - [Get files that match the given fingerprints](https://docs.rs/furse/latest/furse/struct.Furse.html#method.get_fingerprint_matches) ([official documentation](https://docs.curseforge.com/rest-api#get-fingerprints-matches))
