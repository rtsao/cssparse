const os = require("os");
const fs = require("fs");
const path = require("path");

const key = `${process.platform} ${os.arch()} ${os.endianness()}`;
const packages = {
  "darwin x64 LE": "cssparse-darwin-64",
  "linux x64 LE": "cssparse-linux-64",
  "win32 x64 LE": "cssparse-windows-64",
};

const id = packages[key];

const src = path.join(__dirname, "native", "index.node");
const dest = path.join(__dirname, "npm", "node_modules", id, `${id}.node`);

fs.copyFileSync(src, dest);
