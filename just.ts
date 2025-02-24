#!/usr/bin/env -S deno run --allow-all
import $ from "@david/dax";
import { Command, EnumType } from "@cliffy/command";

const envEnum = new EnumType(["linux", "windows", "macos", "macos"]);

interface Env {
  binary: string;
}

async function installLinuxDeps() {
  // TODO: check if Ubuntu
  await $`sudo apt-get update`;
  await $`sudo apt-get install clang pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev mold mesa-vulkan-drivers`;
}

async function installWasmDeps() {
  await Promise.all([
    $`rustup component add rustc-codegen-cranelift-preview --toolchain nightly`,
    $`cargo install -f wasm-bindgen-cli --version 0.2.95`,
    $`cargo install wasm-opt`,
  ]);
}

async function buildWasm() {
  await $`cargo build --release --target wasm32-unknown-unknown`;
}

async function prepareWasmPackage(env: Env = { binary: "redaerok-app" }) {
  // Gen JS
  await $`wasm-bindgen --out-name ${env.binary} --out-dir wasm --target web target/wasm32-unknown-unknown/release/${env.binary}.wasm`;
  // Optimize Wasm
  await $`wasm-opt -O wasm/${env.binary}_bg.wasm -o ${env.binary}.wasm`;
  // Compress Wasm using brotli
  await $`brotli wasm/${env.binary}_bg.wasm -o web/${env.binary}_bg.wasm`;
  await $`mv wasm/${env.binary}.js web/`;
  // Copy assets
  if (!(await $`test -d assets`)) {
    await $`mkdir assets`;
  }
  await $`cp -r assets web/`;
}

async function buildRelease() {
  await $`cargo b --release`;
}

// TODO: Migrate All workflow script to this file
await new Command()
  .name("just")
  .description("Command used to build whole project")
  .version("0.1.0")
  .type("env", envEnum)
  .globalOption("--env <level:env>", "Environment to build", {
    default: "linux",
  })
  .description("Script for the dinosaur game")
  .action(async () => {
    await buildRelease();
  })
  .command("install-linux-deps", "Install dependencies")
  .action(async () => {
    await installLinuxDeps();
  })
  .command("install-wasm-deps", "Install wasm dependencies")
  .action(async () => {
    await installWasmDeps();
  })
  .command("build-wasm", "Build wasm")
  .action(async () => {
    await buildWasm();
  })
  .command("prepare-wasm-package", "Prepare wasm package")
  .action(async () => {
    await prepareWasmPackage();
  })
  .command("web", "Web build")
  .action(async () => {
    await installWasmDeps();
    await buildWasm();
    await prepareWasmPackage();
  })
  .parse(Deno.args);
