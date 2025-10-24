#!/usr/bin/env node
"use strict";

const path = require('path');
const { spawn } = require('child_process');

// Get the server script path
const serverPath = path.join(__dirname, 'server.js');

// Launch the server
const server = spawn('node', [serverPath], {
	stdio: 'inherit',
	shell: false
});

server.on('error', (err: Error) => {
	console.error('Failed to start language server:', err);
	process.exit(1);
});

server.on('exit', (code: number | null) => {
	if (code !== 0) {
		console.error(`Language server exited with code ${code}`);
		process.exit(code || 1);
	}
});
