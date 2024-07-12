/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// cli

// ----------------------------------------------------------------

use clap::{Parser, Subcommand};

// ----------------------------------------------------------------

#[derive(Parser)]
#[clap(
    name = "tester",
    version = "1.0.0",
    author = "photowey",
    about = "A simple command line tool for omiga server"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start(StartArgs),
}

#[derive(Parser)]
struct StartArgs {
    #[clap(
    // --omiga.server.port=9320
        long = "omiga.server.port",
        default_value = "9320",
        help = "Sets the port for the omega server"
    )]
    omega_server_port: String,
    #[clap(
    // --omiga.application.name=omiga
        long = "omiga.application.name",
        default_value = "omiga",
        help = "Sets the application name for the omega server"
    )]
    omega_application_name: String,
}

pub fn try_parse_command_line_kv_args() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start(args) => {
            println!(
                "Starting server:[{}] on port: {}[HTTP]",
                args.omega_application_name, args.omega_server_port
            );
        }
    }
}
