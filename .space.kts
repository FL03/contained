job("Publish to Docker Hub") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    host("Build artifacts and a Docker image") {
        // assign project secrets to environment variables
        env["HUB_USER"] = Secrets("dockerhub_username")
        env["HUB_TOKEN"] = Secrets("dockerhub_token")

        shellScript {
            content = """
                docker login --username ${'$'}HUB_USER --password "${'$'}HUB_TOKEN"
            """
        }

        dockerBuildPush {
            context = "."
            file = "Dockerfile"
            labels["vendor"] = "Scattered-Systems, LLC"
            tags {
                +"scsys/conduit:latest"
                +"scsys/conduit:0.1.${"$"}JB_SPACE_EXECUTION_NUMBER"
            }
        }
    }
}

job("Test (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                apt-get update -y && apt-get upgrade -y
                apt-get install -y protobuf-compiler
                rustup default nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all-features
            """
        }
    }
}

job("Publish (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                apt-get update -y && apt-get upgrade -y
                apt-get install -y protobuf-compiler
                rustup default nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
                cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p aqueduct
            """
        }
    }
}