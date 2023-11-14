# links

[![GitHub License][shield-license]][license]

![Minidash screenshot][screenshot]

links is a simple, self-hostable and open source link aggregator that is meant to serve as an alternative to [littlelink][littlelink] or [linktree][linktree].

## Running this project

To run this project, simply run:

```sh
docker run -it -p 3000:3000 -v $PWD/data:/app/data ghcr.io/phntxx/links
```

You will however need the `data`-directory for this. Just download [the data directory from this repository](https://github.com/phntxx/links/tree/main/data/) as a starting-off point and then adjust that to fit your needs.

## Development

This project is built using cargo, so building the project is as simple as running:

```sh
cargo run
```

However, environment variables are needed for this to run right, see [Configuration:Environment Variables](#environment-variables)

## Configuration

This project can be configured using files and environment variables to suit your needs just right.

### Files

This project requires two files: The template and the configuration file.

The template file is a simple [handlebar][handlebars]-template. Use the one supplied [here][template] or use it as an example to make your own.

The configuration file is a YAML file that contains all of your links, be it apps or bookmarks. By default, it looks like this:

```yml
---
title: title
subtitle: subtitle
footer: footer
links:

  - name: link-name
    url: link-url
    icon: link-icon (optional)
    description: link-description (optional)
    color:
      primary: link-color-primary (optional)
      secondary: link-color-secondary (optional)
    
  ...
```

**NOTE:** Brand icons are fetched using [font awesome's brand icon catalog][font-awesome]. So for e.g. the discord icon, simply use `discord` as the value of the `ìcon` parameter.

### Environment variables

This tool requires three environment variables to be set:

- `TEMPLATE_FILE`: The path to the template file. With the docker image this defaults to `/app/template.hbs`.
- `CONFIG_FILE`: The path to the config file. With the docker image this defaults to `/app/config.yml`.
- `STATIC_PATH`: The path where static files are hosted. These can then be accessed by navigating to - e.g. - `/static/test.png`. With the docker image this defaults to `/app/data/static`.
- `ADDRESS`: The IP-address and port that the web server should listen on. With the docker image this defaults to `0.0.0.0:3000`.

These three variables need to be set when running `cargo run` in order for the program to do anything.

Additionally, since this project uses the [`env-logger`][env-logger] crate, the `RUST_LOG` environment variable can also be modified to adjust logging levels.

[screenshot]: img/screenshot.png
[font-awesome]: https://fontawesome.com/search?o=r&f=brands
[littlelink]: https://github.com/sethcottle/littlelink
[linktree]: https://linktr.ee
[handlebars]: https://handlebarsjs.com
[data-dir]: https://github.com/phntxx/links/tree/main/data
[template]: https://github.com/phntxx/links/tree/main/data/template.hbs
[env-logger]: https://crates.io/crates/env_logger
[license]: https://github.com/phntxx/links/LICENSE
[shield-license]: https://img.shields.io/github/license/phntxx/links.svg
