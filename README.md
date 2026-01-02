# protoc-gen-fake: Protocol Buffer Fake Data Generator

`protoc-gen-fake` is a `protoc` plugin that generates fake data based on your Protocol Buffer schema definitions and custom annotations.

The fake data it generates is binary data in exactly the same format as defined by that schema. It helps developers quickly create realistic mock data at scale for testing, development, and demonstrations.


## Features

*   **Schema-driven Data Generation:** Generates fake data directly from your `.proto` file definitions.
*   **Customizable Fake Data:** Use `(gen_fake.fake_data)` options to provide _specific_ data types (e.g., names, addresses, emails), and minimum and maximum count for repeated or optional fields.
*   **Internationalization (i18n):** Generate fake data in various languages and locales.
*   **Flexible Output:** Choose between binary or Base64 encoded output, and control output paths.
    (Note: Base64 encoding provides compatibility with `protoc`'s string-based output mechanism, but for most use cases, direct binary file output (configured via `--fake_opt output_path`) is preferred. Base64 support may be re-evaluated for future versions if its utility does not justify the added complexity.)

## Table of Contents

*   [Installation](#installation)
*   [Usage](#usage)
    *   [Direct `protoc` Calls](#direct-protoc-calls)
    *   [`gen_fake.sh` Wrapper Script](#gen_fake.sh-wrapper-script)
    *   [Generating Language-Specific Protobuf Files](#generating-language-specific-protobuf-files)
    *   [Utilizing Generated Fake Data](#utilizing-generated-fake-data)
*   [Configuration](#configuration)
    *   [Message-Level Options](#message-level-options)
    *   [Field-Level Options](#field-level-options-gen_fakefake_data--)
*   [Internationalization (i18n)](#internationalization-i18n)
*   [Contributing](#contributing)


## Installation

`protoc-gen-fake` can be installed in several ways, depending on your preferences and development setup.

### From GitHub Releases (Recommended for most users)

The easiest way to get `protoc-gen-fake` is to download the pre-compiled binary for your operating system from the [GitHub Releases page](https://github.com/mike-williamson/protoc-gen-fake/releases).

1.  **Download:** Go to the [Releases page](https://github.com/mike-williamson/protoc-gen-fake/releases) and download the appropriate `.zip` or `.tar.gz` file for your system (e.g., `protoc-gen-fake-darwin-arm64` for macOS Apple Silicon).
2.  **Extract:** Extract the downloaded archive.
3.  **Add to PATH:** Move the `protoc-gen-fake` executable to a directory in your system's `$PATH` (e.g., `/usr/local/bin` or `~/.cargo/bin`).

    ```bash
    # Example for macOS/Linux
    mv /path/to/downloaded/protoc-gen-fake /usr/local/bin/
    ```

    Ensure the chosen directory is in your `$PATH`. If not, you might need to add it to your shell's configuration file (e.g., `.bashrc`, `.zshrc`, or `.profile`):

    ```bash
    export PATH="/usr/local/bin:$PATH"
    ```

### Via Cargo (For Rust developers)

If you have Rust and Cargo installed, you can install `protoc-gen-fake` directly from [crates.io](https://crates.io/crates/protoc-gen-fake):

```bash
cargo install protoc-gen-fake
```

This will compile the plugin from source and place the executable in `~/.cargo/bin/`, which should already be in your `$PATH`.

### From Source

If you prefer to build the plugin yourself:

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/mike-williamson/protoc-gen-fake.git
    cd protoc-gen-fake
    ```

2.  **Build in release mode:**

    ```bash
    cargo build --release
    ```

    This will create an executable named `protoc-gen-fake` in the `target/release/` directory.

3.  **Copy the executable to your `$PATH`:**

    You can copy the binary to a directory that is already in your `$PATH`, such as `~/.cargo/bin/` (or `/usr/local/bin`):

    ```bash
    cp target/release/protoc-gen-fake ~/.cargo/bin/
    ```

### Via Homebrew (For macOS & Linux users)

You can install `protoc-gen-fake` directly from this repository by tapping it:

```bash
brew tap lazarillo/protoc-gen-fake https://github.com/lazarillo/protoc-gen-fake
brew install protoc-gen-fake
```

Once installed, you can invoke `protoc-gen-fake` directly via `protoc`.

## Usage

### CLI Utility Flags

While `protoc-gen-fake` is primarily used as a plugin, the executable also supports standard CLI flags for version checking and help:

*   `--version` / `-V`: Prints the current version of the plugin.
*   `--help` / `-h`: Prints usage information.

```bash
protoc-gen-fake --version
protoc-gen-fake --help
```

### Direct `protoc` Calls

When using `protoc-gen-fake` directly with `protoc`, you need to specify the output directory for the generated fake data. For binary output, `protoc` does not pass the output directory to the plugin, so you must provide it explicitly using the `--fake_opt output_path` parameter.

```bash
protoc --proto_path=proto \
       --fake_out=. \
       --fake_opt output_path=mike_data/full_customer.bin \
       proto/examples/full_customer.proto
```

*   `--proto_path=proto`: Specifies the directory where your `.proto` files are located.
*   `--fake_out=.`: This tells `protoc` to invoke `protoc-gen-fake`. The value here (`.`) is largely ignored by `protoc-gen-fake` for binary output, but it's required by `protoc`.
*   `--fake_opt output_path=mike_data/full_customer.bin`: This is the crucial part for binary output. It tells `protoc-gen-fake` where to write the generated binary fake data file.
*   `proto/examples/full_customer.proto`: The `.proto` file for which to generate fake data.

To generate Base64 encoded output directly in the `CodeGeneratorResponse` (which `protoc` then writes to stdout or a file if `--fake_out` points to a file), you can specify `encoding=Base64`:

```bash
protoc --proto_path=proto \
       --fake_out=base64_output.txt \
       --fake_opt encoding=Base64 \
       proto/examples/full_customer.proto
```

### `gen_fake.sh` Wrapper Script

To simplify the command-line usage and avoid specifying the output path twice for binary output, you can use the provided `gen_fake.sh` wrapper script. This script handles the internal translation of a single `--out_path` argument into the necessary `protoc` and `--fake_opt` parameters.

```bash
./gen_fake.sh --out_path=mike_data/full_customer.bin proto/examples/full_customer.proto
```

*   `--out_path=mike_data/full_customer.bin`: Specifies the desired output path for the generated binary fake data.
*   `proto/examples/full_customer.proto`: The `.proto` file for which to generate fake data.

### Generating Language-Specific Protobuf Files

Before you can utilize the fake data in your application, you often need to generate language-specific protobuf classes from your `.proto` definitions. Here are examples for Python and Go.

#### Python

```bash
protoc --proto_path=proto \
       --python_out=examples \
       proto/examples/full_customer.proto \
       proto/gen_fake/fake_field.proto
```

*   `--python_out=examples`: Specifies the output directory for the generated Python protobuf files.

#### Go

(Note: This requires `protoc-gen-go` to be installed and in your `$PATH`)

```bash
protoc --proto_path=proto \
       --go_out=examples \
       --go_opt=paths=source_relative \
       proto/examples/full_customer.proto \
       proto/gen_fake/fake_field.proto
```

*   `--go_out=examples`: Specifies the output directory for the generated Go protobuf files.
*   `--go_opt=paths=source_relative`: Ensures that generated Go files have correct import paths.

### Utilizing Generated Fake Data

Once you have generated both the fake data (binary) and the language-specific protobuf classes, you can load and use the fake data in your application.

#### Python

Assuming you have generated `full_customer_pb2.py` into the `examples` directory and `full_customer.bin` into `mike_data/`:

```python
import sys
sys.path.append('examples') # Add the directory where protobuf files are generated

from full_customer_pb2 import FullCustomer

def load_fake_customer(file_path):
    with open(file_path, 'rb') as f:
        data = f.read()
    customer = FullCustomer()
    customer.ParseFromString(data)
    return customer

if __name__ == '__main__':
    fake_customer = load_fake_customer('mike_data/full_customer.bin')
    print(fake_customer)
```

#### Go

Assuming you have generated `full_customer.pb.go` into the `examples` directory and `full_customer.bin` into `mike_data/`:

```go
package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"path/filepath"

	"google.golang.org/protobuf/proto"

	examples "your_module_path/examples" // Replace with your actual module path
)

func main() {
	filePath := filepath.Join("mike_data", "full_customer.bin")
	data, err := ioutil.ReadFile(filePath)
	if err != nil {
		log.Fatalf("Failed to read fake data file: %v", err)
	}

	customer := &examples.FullCustomer{} // Assuming FullCustomer is in your examples package
	if err := proto.Unmarshal(data, customer); err != nil {
		log.Fatalf("Failed to unmarshal fake data: %v", err)
	}

	fmt.Printf("Loaded Fake Customer: %+v\n", customer)
}
```

## Configuration

`protoc-gen-fake` uses custom protobuf options to configure fake data generation for messages and fields.

### Message-Level Options

*   `option (gen_fake.fake_msg).include = true;`
    *   **Purpose:** By default, `protoc-gen-fake` will only generate data for the *first* top-level message defined in a `.proto` file. To explicitly include a specific message for fake data generation (especially useful if you have multiple top-level messages and want to target one, or if it's a nested message you want to ensure is generated as a root object), set this option to `true`. This effectively marks the message as an "entry point" for generation.

    ```proto
    message MyMessage {
      option (gen_fake.fake_msg).include = true;
      string id = 1 [(gen_fake.fake_data).data_type = "UUID"];
    }
    ```

### Field-Level Options (`(gen_fake.fake_data) = {...}`) 

These options are applied to individual fields within a message to control the type and characteristics of the fake data generated.

*   **Enabling Default Fake Data:**
    *   `[(gen_fake.fake_data) = {}]`
        *   **Purpose:** To opt-in a field for default fake data generation. If no specific fake data type (like `email`, `uuid`, etc.) is provided, the plugin will attempt to generate a sensible default based on the field's protobuf type (e.g., random string for `string`, random number for `int32`).

    ```proto
    message User {
      string name = 1 [(gen_fake.fake_data) = {}]; // Generates a default fake string
    }
    ```

*   **Specifying Fake Data Types:**
    *   You can specify a wide range of fake data types using the options within `(gen_fake.fake_data)`.
    *   **Example: Email and UUID**

    ```proto
    message User {
      string email = 1 [(gen_fake.fake_data).data_type = "SafeEmail"];
      string id = 2 [(gen_fake.fake_data).data_type = "UUID"];
    }
    ```

*   **Controlling Optional Field Generation:**
    *   `[(gen_fake.fake_data).min_count = 1]`
        *   **Purpose:** For optional fields, by default, `protoc-gen-fake` might or might not generate a value. Setting `min_count = 1` ensures that the optional field will always be generated.

    ```proto
    message Product {
      optional string description = 1 [(gen_fake.fake_data).min_count = 1]; // Always generate a description
    }
    ```

*   **Nested Messages:**
    *   `protoc-gen-fake` automatically recurses into nested messages. You apply field-level options to fields within the nested messages as usual.

    ```proto
    message Address {
      string street = 1 [(gen_fake.fake_data).data_type = "StreetName"];
      string city = 2 [(gen_fake.fake_data).data_type = "CityName"];
    }

    message Customer {
      string name = 1 [(gen_fake.fake_data).data_type = "Name"];
      Address home_address = 2 [(gen_fake.fake_data) = {}]; // Generates a fake Address
    }
    ```

    In the example above, `home_address` will have a fake `Address` generated for it, with its `street` and `city` fields populated according to their respective `fake_data` options.

## Internationalization (i18n)

`protoc-gen-fake` supports generating localized fake data by specifying a language for specific fields using the `language` option.

```proto
message UserProfile {
  string given_name = 1 [(gen_fake.fake_data).data_type = "FirstName", (gen_fake.fake_data).language = "ZH_TW"];
  string address_street = 2 [(gen_fake.fake_data).data_type = "StreetName", (gen_fake.fake_data).language = "AR_SA"];
}
```

*   `language = "ZH_TW"`: Generates a given name in Traditional Chinese.
*   `language = "AR_SA"`: Generates a street name in Arabic (Saudi Arabia).

Refer to the `fake` crate's documentation for a full list of supported locales and data providers.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
