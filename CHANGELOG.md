## [](https://github.com/ivangabriele/mistralai-client-rs/compare/v0.2.0...v) (2024-03-04)


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
