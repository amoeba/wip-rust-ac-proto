#!/bin/bash

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <type_name>"
    exit 1
fi

TYPE_NAME="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Testing type: $TYPE_NAME"

# Step 1: Run C# program to generate binary data
echo "Step 1: Generating binary data with C#..."
cd csharp-tester
dotnet run -- "$TYPE_NAME"
mv a.bin ..
cd ..

# Step 2: Run Rust program to read the binary data
echo "Step 2: Reading binary data with Rust..."
cd rust-tester
mv ../a.bin .
cargo run -- "$TYPE_NAME"
cd ..

echo "Test completed successfully for type: $TYPE_NAME"