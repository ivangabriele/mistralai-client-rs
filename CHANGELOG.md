## [](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.10.0...v) (2024-06-22)

### Features

* **constants:** add OpenMixtral8x22b, MistralTiny & CodestralLatest to Model enum ([ecd0c30](https://github.com/ivangabriele/mistralai-client-rs/commit/ecd0c3028fdcfab32b867eb1eed86182f5f4ab81))

### Bug Fixes

* **chat:** implement Clone trait for ChatParams & ResponseFormat ([0df67b1](https://github.com/ivangabriele/mistralai-client-rs/commit/0df67b1b2571fb04b636ce015a2daabe629ff352))
## [0.10.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.9.0...v) (2024-06-07)

### ⚠ BREAKING CHANGES

* **chat:** - `Chat::ChatParams.safe_prompt` & `Chat::ChatRequest.safe_prompt` are now `bool` instead of `Option<bool>`. Default is `false`.
- `Chat::ChatParams.temperature` & `Chat::ChatRequest.temperature` are now `f32` instead of `Option<f32>`. Default is `0.7`.
- `Chat::ChatParams.top_p` & `Chat::ChatRequest.top_p` are now `f32` instead of `Option<f32>`. Default is `1.0`.

### Features

* **chat:** add response_format for JSON return values ([85c3611](https://github.com/ivangabriele/mistralai-client-rs/commit/85c3611afbbe8df30dfc7512cc381ed304ce4024))
* **chat:** add the 'system' and 'tool' message roles ([#10](https://github.com/ivangabriele/mistralai-client-rs/issues/10)) ([2fc0642](https://github.com/ivangabriele/mistralai-client-rs/commit/2fc0642a5e4c024b15710acaab7735480e8dfe6a))
* **chat:** change safe_prompt, temperature & top_p to non-Option types ([cf68a77](https://github.com/ivangabriele/mistralai-client-rs/commit/cf68a773201ebe0e802face52af388711acf0c27))

### Bug Fixes

* **chat:** skip serializing tool_calls if null, to avoid 422 error ([da5fe54](https://github.com/ivangabriele/mistralai-client-rs/commit/da5fe54115ce622379776661a440e2708b24810c))
## [0.9.0](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.8.0...v) (2024-04-13)


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
