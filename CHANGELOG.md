# Changelog

## `1.5.10`
### 17.06.2023

- Remove `deny_unknown_fields`
- Update dependencies, replace `lazy_static` with `once_cell`
- Update fields and documentation

## `1.5.6`
### 12.11.2022

- Update dependencies
- Get `get_mods` API call
- Remove redundant doc comments

## `1.5.3`
### 03.09.2022

- Update dependencies
- Remove `bytes` dependency

## `1.5.1`
### 10.07.2022

- Fixed a bug where if an empty string was provided for an `Option<URL>` field, the deserialiser would try to parse the url and error out rather than return `None`.
- Renamed the `Datetime` alias to `UtcTime`

## `1.4.0`
### 15.06.2022

- Dependencies are specified with `~`
- Extract the fingerprint calculation to a separate function

## `1.3.0`
### 03.06.2022

- Replaced the mess of number types in `structures` with `Number` which is an alias for `usize`
- Added the `get_fingerprint_matches()` fingerprint call and it's relevant structs

## `1.2.3`
### 31.05.2022

Add the Quilt mod loader to `ModLoaderType`

## `1.1.2`
### 14.05.2022

- Remove `download_mod_file_from_file()` and `download_mod_file_from_file_id`
- Add `file_download_url()` to get the download url of a file

## `1.1.1`
### 05.05.2022

Make the `logo` field of `Mod` nullable because CurseForge is having an issue where some of the mods' logos are null.

## `1.1.0`
### 01.05.2022

- Add url parse error
- Make the tests actually capture errors
- Improve requests to use url parsing
- All structs consistently use the following:
```rust
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
```
- All enums consistently use the following:
```rust
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
```
- Added missing fields for `Mod`

## [1.0.3] - 05.03.2022

- Removed `file_id` field of `FileDependency`

## [1.0.2] - 02.02.2022

- Implement `Debug` and `Clone` for `Furse`

## [1.0.1] - 23.01.2022

- Make `get_mod_files()` use a pagesize of 10000 because paginations are ignored

## [1.0.0] - 22.01.2022

Initial release
