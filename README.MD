# Markdown Cat

## Background/Lore
A simple CLI tool to concatenate all `.md` files in a directory tree into a single, user-specified markdown file. Perfect for compiling notes, documentation, or project readmes.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Flags](#flags)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features
- Recursively searches for `.md` files
- Appends content to a specified output file
- Marks origin of each file with headings
- Cross-platform support (Linux, macOS, Windows)

## Installation

### Using GitHub Releases
Download the binary for your OS from [Releases](https://github.com/gbiagomba/md_cat/releases), then:
```bash
chmod +x md_cat-linux
./md_cat-linux ./notes output.md
```

### Using Cargo
```bash
cargo install md_cat
```

## Flags
- `-d, --directory`: Directory to search for `.md` files (default: current directory)
- `-o, --output`: Output file to write the concatenated markdown content (default: `output.md`)

## Usage
```bash
md_cat -d ./notes -o output.md
```

### Docker
```bash
docker build -t md_cat .
docker run -v $PWD:/app -it gbiagomba/md_cat:latest -d ./notes -o output.md
```



## Contributing
Contributions are welcome! Please feel free to submit a pull request.

## License
This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.