# Adding Sea-ORM

First, we need to install Sea-ORM CLI,

```bash
cargo install sea-orm-cli
```

You can also specify the version you want to install. In my case, the above command installed `v1.0.0` of the CLI. To
install the same version as mine, type in the following,

```bash
cargo install sea-orm-cli@1.0.0
```

This will install `v1.0.0` instead of the latest release. Any CLI operation mentioned in this post will be with this
version of the Sea-ORM CLI.

Now we will add two `migration` crate. As the name suggests, `migration` will hold the the migrations for our models.

To create the `migration` crate, type in the following command from within your project directory,

```bash
sea-orm-cli migrate init
```

This will create a `migration` crate and should look like this,

```bash
migration
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs
    ├── m20220101_000001_create_table.rs
    └── main.rs
```

We are now done with our Sea-ORM setup. Now let's test it by adding a `User` model.
