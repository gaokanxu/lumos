<p align="center">
  <a href="https://lumos.com">
    <img alt="Lumos" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# Lumos Geyser Plugin Interface

This crate enables a plugin to be added into the Lumos Validator runtime to
take actions at the time of account updates or block and transaction processing;
for example, saving the account state to an external database. The plugin must
implement the `GeyserPlugin` trait. Please see the details of the
`geyser_plugin_interface.rs` for the interface definition.

The plugin should produce a `cdylib` dynamic library, which must expose a `C`
function `_create_plugin()` that instantiates the implementation of the
interface.

The https://github.com/lumos-labs/lumos-accountsdb-plugin-postgres repository
provides an example of how to create a plugin which saves the accounts data into
an external PostgreSQL database.

More information about Lumos is available in the [Lumos documentation](https://lumos.com/docs).

Still have questions?  Ask us on [Stack Exchange](https://sola.na/sse)
