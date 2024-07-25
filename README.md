<h1 align="center">Raven</h1>

<div align="center"></div>

Share data across your machines in local networks.

Raven is a CLI tool for people that work with multiple machines and often need a quick way to share data. Raven provides a simple interface to share data by sending text messages or files across machines.

## Features

- Receive files
- Send messages

## Future Features

- **Daemonize**: run a `raven` receiving client as a daemon to receive data in the background
- **File Storage**: improve file storage mechanisms to not mess with other received files by allowing the creation of folders
- **Mailbox**: received text messages will be saved to a "mailbox" that can be checked whenever yhe user want
- **Transfer Limits**: define limits to data that can be received to not overflow the machine storage/memory
- **Trustness**: authentication system to mark some clients as trusty clients so they can use different transfer limits
- **Encryption**: encrypt data being shared to keep your privacy
- **Integration**: integrate `raven` with tools as `xclip`, `bat`, etc. to provide a better user experience 
- **Configuration Port**: older version configurations can be auto ported to newer versions

## Installation

At the moment there are no packages for Raven, you must compile from the source (rust v1.79.0 required).

```bash
cargo install --git https://github.com/OJarrisonn/raven.git --branch master
```

## Usage

Every subcommand has a `--help` page where one may find more info.

### Receiving

Currently Raven doesn't work in background quite properly. You must keep a running instance of `raven receive`. The option `--from` and `--port` override the options `receiver.address` and `receiver.port`, respectively, in the `config.toml` to open a tcp listener where incoming ravens arrive. The receiver will store the messages in the [mailbox](#mailbox)

Files sent will be saved to `$HOME/.raven/data/` with the same name it has on the sending host.

### Sending

There are two subcommands useful for sending data: `send` and `send-file`.

The former is used to send text messages, the later to send files. Both commands require the receiver's address via `--to` flag and optionally the port (`--port`) where the receiving host is probably waiting for ravens.

### Mailbox

The mailbox is where one manages the received messages, there are 3 subcommands: 

- `list`: shows the received ravens, use `--file` or `--message` to filter
- `show`: shows the content of a received text raven or the path of a received file by it's `id` (shown in the `list`) use `--file` or `--message` to indicate which one to show
- `delete`: deletes a message (`--message`) or a file (`--file`) from the mailbox by it's `id`. If deleting a file, the file will also be deleted from the file system.

Every received raven holds the information about the sender, when it arrived and it's content.

The mailbox entries can be checked out in the `mailbox.toml` file in the raven home folder.

### Configuration

By default the configuration is located at `$HOME/.raven/config.toml` but this behaviour can be overwritten by the use of the environment variable `RAVEN_HOME`.

The config file currently only has two options: `receiver.address` and `receiver.port` to setup where to open a tcp listener when `raven receive`.

## License

See [LICENSE](./LICENSE)

## Contributing

You may contribute by creating issues with bugs you've found or feature requests.

Also, you may fork the repo, implement your feature/fix and open a PR.