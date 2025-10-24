#!/bin/bash

# PohLang Language Server Launcher (Unix/Linux/Mac)

# Check if node command is available
if ! command -v node &> /dev/null; then
    echo "Error: 'node' command not found. Please install Node.js."
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Launch the LSP server
exec node "$SCRIPT_DIR/out/server.js" "$@"