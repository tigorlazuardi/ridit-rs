<a name="unreleased"></a>
## [Unreleased]


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


[Unreleased]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.3...HEAD
[v0.1.3]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/tigorlazuardi/ridit-rs/compare/v0.1.0...v0.1.1
