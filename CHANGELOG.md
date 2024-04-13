## [](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.8.0...v) (2024-04-13)


### ⚠ BREAKING CHANGES

* `Model.OpenMistral8x7b` has been renamed to `Model.OpenMixtral8x7b`.

### Bug Fixes

* **deps:** update rust crate reqwest to 0.12.0 ([#6](https://github.com/ivangabriele/mistralai-client-rs/issues/6)) ([fccd59c](https://github.com/ivangabriele/mistralai-client-rs/commit/fccd59c0cc783edddec1b404363faabb009eecd6))
* fix typo in OpenMixtral8x7b model name ([#8](https://github.com/ivangabriele/mistralai-client-rs/issues/8)) ([6a99eca](https://github.com/ivangabriele/mistralai-client-rs/commit/6a99eca49c0cc8e3764a56f6dfd7762ec44a4c3b))

## [0.8.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.7.0...v) (2024-03-09)


### ⚠ BREAKING CHANGES

* Too many to count in this version. Check the README examples.

### Features

* add function calling support to client.chat() & client.chat_async() ([74bf8a9](https://github.com/ivangabriele/mistralai-client-rs/commit/74bf8a96ee31f9d54ee3d7404619e803a182918b))

## [0.7.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.6.0...v) (2024-03-05)


### ⚠ BREAKING CHANGES

* - Rename `ClientError.ApiKeyError` to `MissingApiKey`.
- Rename `ClientError.ReadResponseTextError` to `ClientError.UnreadableResponseText`.

### Bug Fixes

* fix failure when api key as param and not env ([ef5d475](https://github.com/ivangabriele/mistralai-client-rs/commit/ef5d475e2d0e3fe040c44d6adabf7249e9962835))

## [0.6.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.5.0...v) (2024-03-04)


### ⚠ BREAKING CHANGES

* You can't set the `stream` option for `client.chat*()`.

Either use `client.chat_stream()` if you want to use streams
or use `client.chat()` / `client.chat_async()` otherwise.

### Features

* add client.chat_stream() method ([4a4219d](https://github.com/ivangabriele/mistralai-client-rs/commit/4a4219d3eaa8f0ae953ee6182b36bf464d1c4a21))

## [0.5.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.4.0...v) (2024-03-04)


### Features

* add client.embeddings_async() method ([3c22891](https://github.com/ivangabriele/mistralai-client-rs/commit/3c228914f78b0edd4a592091265b88d0bc55568b))
* add client.list_models_async() method ([b69f7c6](https://github.com/ivangabriele/mistralai-client-rs/commit/b69f7c617c15dd63abb61d004636512916d766bb))

## [0.4.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.3.0...v) (2024-03-04)


### ⚠ BREAKING CHANGES

* `Client::new()` now returns a `Result`.
* `APIError` is renamed to `ApiError`.

### Features

* add client.chat_async() method ([1dd59f6](https://github.com/ivangabriele/mistralai-client-rs/commit/1dd59f67048c10458ab0382af8fdfe4ed21c82fa))
* add missing api key error ([1deab88](https://github.com/ivangabriele/mistralai-client-rs/commit/1deab88251fc706e0415a5e416ab9aee4b52f6f3))
* wrap Client::new() return in a Result ([3387618](https://github.com/ivangabriele/mistralai-client-rs/commit/33876183e41340f426aa1dd1b6d8b5c05c8e15b9))

## [0.3.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.2.0...v) (2024-03-04)


### ⚠ BREAKING CHANGES

* Models are now enforced by `Model` & `EmbedModel` enums.

### Features

* add client.embeddings() method ([f44d951](https://github.com/ivangabriele/mistralai-client-rs/commit/f44d95124767c3a3f14c78c4be3d9c203fac49ad))

## [0.2.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.1.0...v) (2024-03-03)


### ⚠ BREAKING CHANGES

* Chat completions must now be called directly from client.chat() without building a request in between.

### Features

* add client.list_models() method ([814b991](https://github.com/ivangabriele/mistralai-client-rs/commit/814b9918b3aca78bfd606b5b9bb470b70ea2a5c6))
* simplify chat completion call ([7de2b19](https://github.com/ivangabriele/mistralai-client-rs/commit/7de2b19b981f1d65fe5c566fcaf521e4f2a9ced1))

## [0.1.0](https://github.com/ivangabriele/mistralai-client-rs/compare/7d3b438d16e9936591b6454525968c5c2cdfd6ad...v0.1.0) (2024-03-03)

### Features

- add chat completion without streaming ([7d3b438](https://github.com/ivangabriele/mistralai-client-rs/commit/7d3b438d16e9936591b6454525968c5c2cdfd6ad))
