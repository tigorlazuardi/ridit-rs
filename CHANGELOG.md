<a name="unreleased"></a>
## [Unreleased]


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


[Unreleased]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.4.2...HEAD
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
