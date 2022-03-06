# Notion-CLI

A little CLI to quickly add new items to a database in Notion on Regolith Linux. Uses Rofi to show a dmenu like launcher, from which the input is used to create a new page in Notion with a database as it's parent.

## Installation

Use cargo to build from source.

## Usage

First configure your credentials with the configure command.

This will promt for your notion secret and database id.

```bash
notion-cli configure
```

Then add new todos with the add command.

```bash
notion-cli add
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)