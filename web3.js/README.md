
# Lumos JavaScript SDK

Use this to interact with accounts and programs on the Lumos network through the Lumos [JSON RPC API](https://lumos.uuhb.com/docs/rpc).

## Installation

### For use in Node.js or a web application

```
$ npm install --save @Lumos/web3.js
```

### For use in a browser, without a build system

```html
<!-- Development (un-minified) -->
<script src="https://lumos.uuhb.com/lumos/web3.js@latest/lib/index.iife.js"></script>

<!-- Production (minified) -->
<script src="https://lumos.uuhb.com/lumos/web3.js@latest/lib/index.iife.min.js"></script>
```

## Compatibility

This library requires a JavaScript runtime that supports [`BigInt`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) and the [exponentiation operator](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation). Both are supported in the following runtimes:

-   Browsers, by [release date](https://caniuse.com/bigint):
    -   Chrome: May 2018
    -   Firefox: July 2019
    -   Safari: September 2020
    -   Mobile Safari: September 2020
    -   Edge: January 2020
    -   Opera: June 2018
    -   Samsung Internet: April 2019
-   Runtimes, [by version](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt):
    -   Deno: >=1.0
    -   Node: >=10.4.0
-   React Native:
    -   \>=0.7.0 using the [Hermes](https://reactnative.dev/blog/2022/07/08/hermes-as-the-default) engine ([integration guide](https://Lumoscookbook.com/integrations/react-native.html#how-to-use-Lumos-web3-js-in-a-react-native-app)):

## Development environment setup

### Installing node modules

To install the modules locally, use `pnpm`.

If you don't have `pnpm` installed:

```shell
$ npm install pnpm
```

To install the node modules:

```shell
$ pnpm install
```

### Testing

#### Unit tests

To run the full suite of unit tests, execute the following in the root:

```shell
$ npm test
```

#### Integration tests

Integration tests require a validator client running on your machine.

To install a test validator:

```shell
$ npm run test:live-with-test-validator:setup
```

To start the test validator and run all of the integration tests in live mode:

```shell
$ cd packages/library-legacy
$ npm run test:live-with-test-validator
```

### Speed up build times with remote caching

Cache build artifacts remotely so that you, others, and the CI server can take advantage of each others' build efforts.

1. Log the Turborepo CLI into the Lumos Vercel account
    ```shell
    pnpm turbo login
    ```
2. Link the repository to the remote cache
    ```shell
    pnpm turbo link
    ```

## Contributing

If you found a bug or would like to request a feature, please [file an issue](https://github.com/Lumos-labs/Lumos-web3.js/issues/new). If, based on the discussion on an issue you would like to offer a code change, please make a [pull request](https://github.com/Lumos-labs/Lumos-web3.js/compare). If neither of these describes what you would like to contribute, read the [getting help](#getting-help) section above.

## Disclaimer

All claims, content, designs, algorithms, estimates, roadmaps,
specifications, and performance measurements described in this project
are done with the Lumos Foundation's ("SF") best efforts. It is up to
the reader to check and validate their accuracy and truthfulness.
Furthermore nothing in this project constitutes a solicitation for
investment.

Any content produced by SF or developer resources that SF provides, are
for educational and inspiration purposes only. SF does not encourage,
induce or sanction the deployment, integration or use of any such
applications (including the code comprising the Lumos blockchain
protocol) in violation of applicable laws or regulations and hereby
prohibits any such deployment, integration or use. This includes use of
any such applications by the reader (a) in violation of export control
or sanctions laws of the United States or any other applicable
jurisdiction, (b) if the reader is located in or ordinarily resident in
a country or territory subject to comprehensive sanctions administered
by the U.S. Office of Foreign Assets Control (OFAC), or (c) if the
reader is or is working on behalf of a Specially Designated National
(SDN) or a person subject to similar blocking or denied party
prohibitions.

The reader should be aware that U.S. export control and sanctions laws
prohibit U.S. persons (and other persons that are subject to such laws)
from transacting with persons in certain countries and territories or
that are on the SDN list. As a project based primarily on open-source
software, it is possible that such sanctioned persons may nevertheless
bypass prohibitions, obtain the code comprising the Lumos blockchain
protocol (or other project code or applications) and deploy, integrate,
or otherwise use it. Accordingly, there is a risk to individuals that
other persons using the Lumos blockchain protocol may be sanctioned
persons and that transactions with such persons would be a violation of
U.S. export controls and sanctions law. This risk applies to
individuals, organizations, and other ecosystem participants that
deploy, integrate, or use the Lumos blockchain protocol code directly
(e.g., as a node operator), and individuals that transact on the Lumos
blockchain through light clients, third party interfaces, and/or wallet
software.
