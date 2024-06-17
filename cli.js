#!/usr/bin/env node

const { chokidarStart } = require('.')
chokidarStart(process.argv.slice(1))