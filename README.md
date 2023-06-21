![Build](https://github.com/net-stalker/net-monitor/actions/workflows/rust.yml/badge.svg?branch=develop)

# net-monitor

Capture packets and redirect them to the cloud for the feature analyze.

## How to start development

I guess if you read this document you already cloned a repository with the project.  
To build the project for development, run the following command:

1. Build and install NNG library  
    ```shell
    git clone https://github.com/nanomsg/nng.git  
    cd nng  
    mkdir build && cd build  
    cmake ..  
    make  
    make install  
    ```
2. Install libpq library
    ```shell
    brew install libpq
    echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```
3. Build project
    ```shell
    cargo clean && cargo build
    ```

Next, you need to start TimescaleDB and update the SQL queries using Liquibase. Run the following
commands:

```shell
docker-compose build timescaledb && docker-compose up timescaledb
docker-compose build timescaledb-migrations_liquibase && docker-compose up timescaledb-migrations_liquibase
```

After that, open your preferred IDE, find the net-all-in-one binary, and run it. You should be able
to find some data in the captured_traffic table in TimescaleDB.

Enjoy developing!

## How to run a platform using docker-compose

Prerequisites: **docker** and **docker-compose** should be installed to local machine.

Configuration: zero configuration.

Usage: To start platform you need to execute command in a terminal
run: 
```shell
docker-compose build && docker-compose up
```

To access to the service you can use netcat util. Example, how to check if Timescaledb is up:

```shell
nc -vz localhost 5432
```

Troubleshooting:

## [SQL Migrations](net-timescale%2Fmigrations)

## Configuration

## A Framework for Writing Distributed Applications

Project is in monorepo.  
You write a single package, using only language-native data structures and method calls.
A **package** is a bundle of one or more crates that provides a set of functionality. A package
contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package
that contains the **binary crate (entrypoint)** for the command-line tool you’ve been using to build
your code. A package can contain as many binary crates as you like, but at most only one **library**
crate. A package must contain at least one crate, whether that’s a library or binary crate. The
library contains a set **modules**. There are reserved module names in the system currently,
**component** and **command**. In the future these modules will be grows. Every module should
be localed in the rust appropriate directory and implemented appropriate trait. For instance, the
component should be located in the component directory and should implement net_core::layer::
NetComponent. In the future will be created some set of rules to check it and restrict in CI flow.
