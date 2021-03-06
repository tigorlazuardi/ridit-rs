<a name="unreleased"></a>
## [Unreleased]


<a name="v0.4.11"></a>
## [v0.4.11] - 2021-09-30
### Feature
- **server:** implemented download trigger


<a name="v0.4.10"></a>
## [v0.4.10] - 2021-09-29
### Feature
- **cli-aspect-ratio:** added text response on configuration for minimum-size
- **cli-aspect-ratio:** added text response on configuration edit
- **config:** added profile command
- **config:** added server port setting
- **docker:** added dockerfile
- **server:** added state status
- **server:** added port and ip configuration
- **server:** impl From for ProtoDownloadMeta from DownloadMeta

### Fix
- removed helloworld proto load from code
- **config:** default config path now will not panic, but instead raw string paths only

### Perf
- **cli-start:** uses fasthash hasher to display cli bar more smoothly

### Refactor
- **cli:** display bars are prettier
- **cli-start:** simplify bar prefix code
- **download_meta:** removed unnecessary codes
- **proto:** split profile and server into separate protos
- **reddit:** now download status is handled via streams
- **server.proto:** added download meta

### Revert
- **cargo.toml:** release profile will not prioritize size for speed reason

### WIP
- **proto:** added skeleton for ridit proto
- **proto:** added ridit service and profile service

### Wip
- **grpc:** added grpc template


<a name="v0.4.9"></a>
## [v0.4.9] - 2021-09-20
### Feature
- **pad:** added padding on bar text


<a name="v0.4.8"></a>
## [v0.4.8] - 2021-09-20
### Clean
- **repository:** hardcoded username useragent now in static

### Cleanup
- **pkg:** removed pkg from app (unused codes)

### Feature
- **cargo.toml:** release profile optimized for binary size
- **cli:** support for tty detection

### Format
- **cargo.toml:** format

### Refactor
- **reddit:** download listing text now dependend on Printout Enum


<a name="v0.4.7"></a>
## [v0.4.7] - 2021-09-20
### Feat
- **download:** images in temp folder are deleted upon successful copy

### Fix
- **download:** removed images should be temp file NOT the downloaded file

### Version
- bump to 0.4.7


<a name="v0.4.6"></a>
## [v0.4.6] - 2021-09-20
### Feature
- **config:** added download threads support. (default to 4).

### Fix
- **config:** now config if field is not complete, will be filled with default value
- **download_thread:** println prompt grammar is now proper

### Revert
- **download_thread:** default value is reverted back from 4 to 8

### Version
- bump to 0.4.6


<a name="v0.4.5"></a>
## [v0.4.5] - 2021-09-20
### Version
- bump to 0.4.5 to match current git tag


<a name="v0.4.4"></a>
## [v0.4.4] - 2021-09-20
### Feature
- **config:** added proper_name in subreddit key
- **config:** config now does not lowercase subreddit but instead checks for proper casing on adding subs

### Fix
- **cli:** added write config after selecting remove subreddit
- **reddit:** removed unnecessary fields in listing for json deserializing
- **reqwest:** reqwest now using rustls-tls instead of openssl for tls matching
- **user_agent:** renamed user_agent to proper specification of reddit

### Refactor
- **listing:** renamed variable to not blatantly shadows usual convention in glance reading
- **reddit:** now download meta and download operation result is passed to the top level function


<a name="v0.4.3"></a>
## [v0.4.3] - 2021-09-15
### Feature
- **config:** moved from hashmap to btreemap
- **reddit:** increased poke image size from 512 bytes to 20kB for image signature
- **reddit:** changed user agent to include repo name
- **reddit:** progress_bar now only show on certain enums

### Fix
- **reddit:** now error from downloading images are properly reported


<a name="v0.4.2"></a>
## [v0.4.2] - 2021-09-14
### Feature
- **cli:** added progress bar for downloading
- **config:** adding subreddits check now handled in parallel


<a name="v0.4.1"></a>
## [v0.4.1] - 2021-09-14
### Feature
- **cli:** adding subreddits only checks the net if it is not registered yet
- **config:** update default path to ~/Pictures/ridit for linux and default mobile range to 0.3
- **whole:** downloads now start downloading from every subreddit for every profile

### Refactor
- **impl Subreddit:** for loop does not use name.to_owned() anymore


<a name="v0.4.0"></a>
## [v0.4.0] - 2021-09-13
### Feature
- **cli:** finished manual downloading command


<a name="v0.3.0"></a>
## [v0.3.0] - 2021-09-04
### Doc
- **cli:** removed profile specific configuration example for aspect-ratio

### Feature
- **cli:** mutable borrow fixes on config get mut
- **cli:** added print cli

### Refactor
- **cli:** removed modify_config and it's derivative
- **cli:** aspect ratio does not read config twice now
- **cli:** removed unused imports


<a name="v0.2.1"></a>
## [v0.2.1] - 2021-09-03
### Feature
- **cli:** more detailed reason for error to write configuration
- **cli:** support for sort in adding subreddit
- **cli:** added subreddit cli implementations
- **config:** download timeout moved to top level
- **pkg:** shorten on error definition
- **subreddit:** guard for 0 subreddit and print added subreddits

### Fix
- **cli:** subcommand aspec handle now uses await


<a name="v0.2.0"></a>
## [v0.2.0] - 2021-09-03
### Feature
- **cli:** removed daemon subcmd and edited help text
- **cli:** added aspect ratio implementation commands
- **config:** moved from u32 to usize
- **config:** removed active_daemon config
- **config:** added write default if config does not exist handling
- **config:** added error context to project dir
- **listing:** moved check handler to meta so it can be handled elsewhere as well
- **reddit:** removed backoff and retry_fn from dependency
- **reddit:** get listing now uses unbounded channel
- **reddit:** update repository
- **reddit:** changed retry to tokio_retry
- **reddit:** added download images
- **reddit:** reddit now poke image size first to get image sizes if `download_first` is set
- **sort:** implemented display

### Fix
- **reddit:** create_dir_all runs first before any storing is made


<a name="v0.1.4"></a>
## [v0.1.4] - 2021-08-23
### Doc
- **config:** update modify config explanation

### Feature
- **listing:** added image size and minimum size check

### Fix
- **listing:** fix extension check

### WIP
- **config:** exposed configuration struct
- **config:** added active daemon for subreddits that will be downloaded by daemon
- **config:** added active settings
- **config:** added modify config by profile
- **config:** added write config and modify config api


<a name="v0.1.3"></a>
## [v0.1.3] - 2021-08-23
### Feature
- **config:** added read config api

### Fix
- **config:** proper `Default` implementation and added deref/derefmut for custom type

### Format
- format files using rustfmt config
- added rustfmt config

### Refactor
- **config:** removed deref for more ergonomic code

### Update
- **config:** added serialize derive

### WIP
- **config:** config now uses profile as top most key
- **config:** added default implementations to config


<a name="v0.1.2"></a>
## [v0.1.2] - 2021-08-21
### Doc
- create README.MD

### License
- create LICENSE

### WIP
- **config:** added config models


<a name="v0.1.1"></a>
## [v0.1.1] - 2021-08-20
### Update
- **cli:** added command for start and daemon

### WIP
- **reddit:** added reddit api models


<a name="v0.1.0"></a>
## v0.1.0 - 2021-08-19
### Doc
- **cli:** added aliases for aspect ratio
- **cli:** aliases now visible
- **cli:** added doc for out format
- **cli:** added name and about
- **cli:** added docs for aspect ratio children commands
- **cli:** added docs for aspect ratio subcommand

### Feat
- **cli:** added download settings command
- **cli:** added subreddit cmd
- **cli:** added StructOpt

### Feature
- **cli:** added list command
- **cli:** added download_first argument

### Refactor
- **cli:** renamed app from ridit-rs to ridit
- **cli:** moved function to static method
- **cli:** moved functions to method


[Unreleased]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.11...HEAD
[v0.4.11]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.10...v0.4.11
[v0.4.10]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.9...v0.4.10
[v0.4.9]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.8...v0.4.9
[v0.4.8]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.7...v0.4.8
[v0.4.7]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.6...v0.4.7
[v0.4.6]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.5...v0.4.6
[v0.4.5]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.4...v0.4.5
[v0.4.4]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.3...v0.4.4
[v0.4.3]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.2.1...v0.3.0
[v0.2.1]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.4...v0.2.0
[v0.1.4]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.0...v0.1.1
