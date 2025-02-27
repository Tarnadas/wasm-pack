# wasm-pack build

The `wasm-pack build` command creates the files neccessary for JavaScript
interoperability and for publishing a package to npm. This involves compiling
your code to wasm and generating a pkg folder. This pkg folder will contain the
wasm binary, a JS wrapper file, your `README`, and a `package.json` file.

The `pkg` directory is automatically `.gitignore`d by default, since it contains
build artifacts which are not intended to be checked into version
control.<sup>[0](#footnote-0)</sup>

## Path

The `wasm-pack build` command can be given an optional path argument, e.g.:

```
wasm-pack build examples/js-hello-world
```

This path should point to a directory that contains a `Cargo.toml` file. If no
path is given, the `build` command will run in the current directory.

## Output Directory

By default, `wasm-pack` will generate a directory for it's build output called `pkg`.
If you'd like to customize this you can use the `--out-dir` flag.

```
wasm-pack build --out-dir out
```

The above command will put your build artifacts in a directory called `out`, instead
of the default `pkg`.

## Generated file names

Flag `--out-name` sets the prefix for output file names. If not provided, package name is used instead.

Usage examples, assuming our crate is named `dom`:

```
wasm-pack build
# will produce files
# dom.d.ts  dom.js  dom_bg.d.ts  dom_bg.wasm  package.json  README.md

wasm-pack build --out-name index
# will produce files
# index.d.ts  index.js  index_bg.d.ts  index_bg.wasm  package.json  README.md
```


## Profile

The `build` command accepts an optional profile argument: one of `--dev`,
`--profiling`, or `--release`. If none is supplied, then `--release` is used.

This controls whether debug assertions are enabled, debug info is generated, and
which (if any) optimizations are enabled.

| Profile       | Debug Assertions | Debug Info | Optimizations | Notes                                 |
|---------------|------------------|------------|---------------|---------------------------------------|
| `--dev`       | Yes              | Yes        | No            | Useful for development and debugging. |
| `--profiling` | No               | Yes        | Yes           | Useful when profiling and investigating performance issues. |
| `--release`   | No               | No         | Yes           | Useful for shipping to production.    |

The `--dev` profile will build the output package using cargo's [default
non-release profile][cargo-profile-sections-documentation]. Building this way is
faster but applies few optimizations to the output, and enables debug assertions
and other runtime correctness checks. The `--profiling` and `--release` profiles
use cargo's release profile, but the former enables debug info as well, which
helps when investigating performance issues in a profiler.

The exact meaning of the profile flags may evolve as the platform matures.

[cargo-profile-sections-documentation]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections

## Target

The `build` command accepts a `--target` argument. This will customize the JS
that is emitted and how the WebAssembly files are instantiated and loaded. For
more documentation on the various strategies here, see the [documentation on
using the compiled output][deploy].

```
wasm-pack build --target nodejs
```

| Option    | Usage | Description                                                                                                     |
|-----------|------------|-----------------------------------------------------------------------------------------------------|
| *not specified* or `bundler` | [Bundler][bundlers] | Outputs JS that is suitable for interoperation with a Bundler like Webpack. You'll `import` the JS and the `module` key is specified in `package.json`. `sideEffects: false` is by default. |
| `nodejs`  | [Node.js][deploy-nodejs] | Outputs JS that uses CommonJS modules, for use with a `require` statement. `main` key in `package.json`. |
| `web` | [Native in browser][deploy-web] | Outputs JS that can be natively imported as an ES module in a browser, but the WebAssembly must be manually instantiated and loaded. |
| `no-modules` | [Native in browser][deploy-web] | Same as `web`, except the JS is included on a page and modifies global state, and doesn't support as many `wasm-bindgen` features as `web` |

[deploy]: https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html
[bundlers]: https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#bundlers
[deploy-nodejs]: https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#nodejs
[deploy-web]: https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#without-a-bundler

## Scope

The init command also accepts an optional `--scope` argument. This will scope
your package name, which is useful if your package name might conflict with
something in the public registry. For example:

```
wasm-pack build examples/js-hello-world --scope test
```

This command would create a `package.json` file for a package called
`@test/js-hello-world`. For more information about scoping, you can refer to
the npm documentation [here][npm-scope-documentation].

[npm-scope-documentation]: https://docs.npmjs.com/misc/scope

## Mode

The `build` command accepts an optional `--mode` argument.
```
wasm-pack build examples/js-hello-world --mode no-install
```

| Option        | Description                                                                              |
|---------------|------------------------------------------------------------------------------------------|
| `no-install`  | `wasm-pack init` implicitly and create wasm binding  without installing `wasm-bindgen`.  |
| `normal`      | do all the stuffs of `no-install` with installed `wasm-bindgen`.                         |

## Multiple build targets per package

The `build` command accepts an optional `--is-child` argument.
```
wasm-pack build examples/js-hello-world --is-child --out-name child
```

This command will extend a previously generated package, which makes it possible to have multiple build targets per package.
For example it would be feasible to have an optimized build for web and Node.

Following is an example which will use the wee_alloc feature for web, because size is more crucial whereas Node should be optimized for speed.

```
wasm-pack build examples/js-hello-world --out-dir pkg -- --features "wee_alloc"
wasm-pack build examples/js-hello-world --out-dir pkg --out-name js-hello-node --is-child
```

## Extra options

The `build` command can pass extra options straight to `cargo build` even if they are not
supported in wasm-pack. To use them you should add standalone `--` argument at the very
end of your command, and all the arguments you want to pass to cargo should go after.
For example, to build the previous example using cargo's offline feature:

```
wasm-pack build examples/js-hello-world --mode no-install -- --offline
```

<hr style="font-size: 1.5em; margin-top: 2.5em"/>

<sup id="footnote-0">0</sup> If you need to include additional assets in the pkg
directory and your NPM package, we intend to have a solution for your use case
soon. [↩](#wasm-pack-build)
