# Windows File Picker

A minimal Windows binary for file picking using native IFileDialog APIs, built in Rust. Provides the same dark-themed dialog as File Explorer with Quick Access/OneDrive sidebar, multi-select, folder picking, file filters, and initial directory support.

![Windows File Picker UI](/img/ui.png){: style="max-height: 500px;"}

## Features

- **Native Windows File Dialog**: Uses the same interface as Windows File Explorer
- **Dark Theme Support**: Matches your system theme automatically
- **Multi-select**: Select multiple files at once
- **Folder Picking**: Choose folders instead of files
- **File Filters**: Filter by specific file extensions
- **Initial Directory**: Start the dialog in a specific folder
- **JSON Output**: Structured output for easy integration with Node.js and other applications
- **Quick Access Sidebar**: Full Windows Quick Access and OneDrive integration

## Installation

### Prerequisites

- Windows 10/11
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
git clone <your-repo-url>
cd filepicker
cargo build --release
```

The executable will be available at `target/release/filepicker.exe`

## Usage

### Basic Examples

```bash
# Pick a single file
./filepicker.exe

# Pick multiple files
./filepicker.exe --multi

# Pick a folder
./filepicker.exe --mode folder

# Custom title and filters
./filepicker.exe --title "Select Images" --filter "*.jpg;*.png;*.gif"

# Start in specific directory
./filepicker.exe --initial "C:\Users\YourName\Documents"
```

### Command Line Arguments

| Argument | Description | Default | Example |
|----------|-------------|---------|---------|
| `--mode` | Selection mode: `files` or `folder` | `files` | `--mode folder` |
| `--multi` | Enable multi-select | `false` | `--multi` |
| `--title` | Dialog window title | `"Open"` | `--title "Select Images"` |
| `--initial` | Initial directory path | `""` | `--initial "C:\Users"` |
| `--filter` | File extension filters | `""` | `--filter "*.txt;*.csv;*.md"` |

### Output Format

The program outputs JSON to stdout:

```json
// Successful selection
{
  "canceled": false,
  "paths": [
    "C:\\Users\\axwt\\Documents\\file1.txt",
    "C:\\Users\\axwt\\Documents\\file2.csv"
  ]
}

// User canceled
{
  "canceled": true,
  "paths": []
}
```

## Integration Examples

### Node.js Integration

```javascript
const { spawn } = require('child_process');

function openFilePicker(options = {}) {
  return new Promise((resolve, reject) => {
    const args = [];

    if (options.multi) args.push('--multi');
    if (options.mode) args.push('--mode', options.mode);
    if (options.title) args.push('--title', options.title);
    if (options.filter) args.push('--filter', options.filter);
    if (options.initial) args.push('--initial', options.initial);

    const picker = spawn('./bin/win/filepicker.exe', args);

    let output = '';
    picker.stdout.on('data', (data) => {
      output += data.toString();
    });

    picker.on('close', (code) => {
      if (code === 0) {
        try {
          const result = JSON.parse(output);
          resolve(result);
        } catch (err) {
          reject(new Error('Failed to parse output'));
        }
      } else {
        reject(new Error(`Process exited with code ${code}`));
      }
    });
  });
}

// Usage
openFilePicker({
  multi: true,
  title: 'Select Images',
  filter: '*.jpg;*.png;*.gif'
}).then(result => {
  if (!result.canceled) {
    console.log('Selected files:', result.paths);
  }
});
```

### CEP Extension Integration

For Adobe CEP extensions, place the binary at `./bin/win/filepicker.exe` and call it from your Node.js backend:

```javascript
const path = require('path');
const { spawn } = require('child_process');

const pickerPath = path.join(__dirname, 'bin', 'win', 'filepicker.exe');

// Use in your CEP extension
function selectFiles() {
  const picker = spawn(pickerPath, ['--multi', '--filter', '*.psd;*.ai;*.jpg']);
  // Handle output as shown above
}
```

## Development

### Project Structure

```
filepicker/
├── src/
│   └── main.rs          # Main application code
├── target/              # Build output (gitignored)
├── Cargo.toml          # Dependencies and metadata
├── Cargo.lock          # Dependency lock file
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

### Dependencies

- `windows` - Windows API bindings
- `serde` - Serialization framework
- `serde_json` - JSON serialization

### Building for Distribution

```bash
# Build optimized release binary
cargo build --release

# The binary will be at target/release/filepicker.exe (165KB)
```

### Suppressing Console Window

To prevent the console window from flashing when the executable runs, uncomment the first line in `src/main.rs`:

```rust
#![windows_subsystem = "windows"]
```

## License

MIT