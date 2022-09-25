# Kafka Schema Registry CLI

An opinionated client for the [Confluent Schema Registry](https://docs.confluent.io/current/schema-registry/docs/index.html).

This cli uses a simple configuration file to manage schemas in the registry.

The topic name follows a strict naming convention:

`{purpose}.{service}.{resource}`

- purpose: One of the following - [request, reply, event, store, log]
- service: The name of the service or domain that owns this topic
- resource: The name of the resource or object this schema is about

The naming convention is used to generate the topic name and the schema name.


## Installation

To install using [brew on Mac or Linux](https://brew.sh/):

Install kscli:
```bash
```sh
brew tap drujensen/kscli
brew install kscli
```

## Usage

To start off, we need to create a config file.  The default location of the config file is: `config/ksconfig.yml`

### Initialize config file

To create an example config file:
```sh
kscli init
```

If you need to specify a different location, you will need to use the `-c` flag:
```sh
kscli -c example.yml init
```

Here is what the config file looks like:
```yaml
---
service: blog
schema_path: "./schemas"
topics:
  - resource: post
    purpose: request
    properties:
      retry: true
      dlt: true
  - resource: post
    purpose: reply
    properties:
      retry: true
      dlt: true
```

Sample avsc files will be created in the `schema_path` directory. The avsc files will be named using the naming convention.

The file naming convention is: `{schema_path}/{purpose}-{resource}.avsc`

A sample avsc file:
```json
{
  "type": "record",
  "name": "PostRequest",
  "namespace": "com.example.blog",
  "fields": [
      {"name": "id", "type": "string"},
      {"name": "title", "type": "string"},
      {"name": "body", "type": "string"},
  ]
}
```

### Push to the Schema Registry

After setting up the config file and any changes to the schema, you can push the changes to the Schema Registry.

To push the changes:
```sh
kscli push
```

Push will create the topics and schemas in the registry.  If you enabled retry and/or dlt, it will create another schema for each topic.  These names are compatible with Java Spring Boot naming convention for RetryConfiguration.

This also changes the DLT compatibility to NONE so that the DLT topic can be used with any schema.  If you don't set this to NONE, you could end up in an endless loop trying to process a message and getting a failure to register the new schema because its incompatible with the previous schema.


### Pull from the Schema Registry

If you need to pull the schemas from the Schema Registry, you can do so with the following command:
```sh
kscli pull
```

## Enviroment Variables

There are cases where you want to override some of the configuration settings.  You can do this using environment variables.

The following environment variables are supported:
```sh
KAFKA_SCHEMA_CLI_CONFIG={path to the config file}
KAFKA_SCHEMA_REGISTRY_URL={url to the schema registry}
```

## Development

This was built using the Rust programming language.  If you want to contribute, you will need to install Rust.  You can find the installation instructions [here](https://www.rust-lang.org/tools/install).

To build the application:
```bash
cargo build
```

To run the application:
```bash
cargo run
```

To install the application:
```bash
cargo install --path .
```

## Contributing

1. Fork it (<https://github.com/drujensen/kscli/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [Dru Jensen](https://github.com/drujensen) - creator and maintainer
