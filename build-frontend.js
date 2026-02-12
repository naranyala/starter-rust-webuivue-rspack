#!/usr/bin/env bun

import fs from 'fs/promises';
import { execSync } from 'child_process';

async function buildFrontend() {
  console.log('Building frontend...');

  const frontendDir = './frontend';
  const originalDir = process.cwd();
  process.chdir(frontendDir);

  try {
    // Install dependencies if needed
    console.log('Checking frontend dependencies...');
    try {
      await fs.access('node_modules');
      console.log('Frontend dependencies already installed.');
    } catch {
      console.log('Installing frontend dependencies...');
      execSync('bun install', { stdio: 'inherit' });
    }

    // Run rspack production build
    console.log('Running rspack production build...');
    execSync('bun run build:incremental', { stdio: 'inherit' });

    // Note: With Rspack, files are already in the correct location: dist/static/js and dist/static/css
    // No flattening needed as with the previous build system
    console.log('Rspack output is already in correct structure');

    // Copy static files to root for WebUI server
    console.log('Copying static files to root...');
    await fs.mkdir('../static/js', { recursive: true });
    await fs.mkdir('../static/css', { recursive: true });

    const rootJsFiles = await fs.readdir('./dist/static/js/');
    for (const file of rootJsFiles) {
      const srcPath = `./dist/static/js/${file}`;
      const destPath = `../static/js/${file}`;
      if ((await fs.stat(srcPath)).isFile()) {
        await fs.copyFile(srcPath, destPath);
        console.log(`  Copied to root: ${file}`);
      }
    }

    const rootCssFiles = await fs.readdir('./dist/static/css/');
    for (const file of rootCssFiles) {
      const srcPath = `./dist/static/css/${file}`;
      const destPath = `../static/css/${file}`;
      if ((await fs.stat(srcPath)).isFile()) {
        await fs.copyFile(srcPath, destPath);
        console.log(`  Copied to root: ${file}`);
      }
    }

    // Now update the HTML paths to use root static
    console.log('Updating index.html paths...');
    let indexHtml = await fs.readFile('./dist/index.html', 'utf8');

    // Update paths to reference the static directory where Rust serves files
    // Rspack generates paths like /static/js/filename.js, /static/css/filename.css
    // These should remain as-is since Rust serves from the static directory
    
    // Update the title in the HTML
    indexHtml = indexHtml.replace(
      /<title>[^<]*<\/title>/,
      '<title>Rust WebUI Application</title>'
    );

    // Write updated index.html
    await fs.writeFile('./dist/index.html', indexHtml);

    console.log('Frontend build completed successfully!');
    console.log('Output: frontend/dist/');
  } catch (error) {
    console.error('Error during frontend build:', error);
    process.exit(1);
  } finally {
    process.chdir(originalDir);
  }
}

async function pathExists(p) {
  try {
    await fs.access(p);
    return true;
  } catch {
    return false;
  }
}

buildFrontend();
