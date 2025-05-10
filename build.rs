use std::path::Path;
use std::process::Command;
use std::{fs, time::SystemTime};

fn generate_client(openapi_spec: &Path, output_dir: &str, package_name: &str, force: bool) {
    let output_path = Path::new(output_dir);

    if !openapi_spec.exists() {
        println!(
            "cargo:warning=OpenAPI spec not found at {}",
            openapi_spec.display()
        );
        return;
    }

    let regenerate = if force {
        true
    } else {
        match (fs::metadata(openapi_spec), fs::metadata(output_path)) {
            (Ok(spec_meta), Ok(out_meta)) => {
                let spec_time = spec_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let out_time = out_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                spec_time > out_time
            }
            _ => true,
        }
    };

    if !regenerate {
        println!(
            "cargo:warning=OpenAPI client for {} is up to date, skipping generation",
            package_name
        );
        return;
    }

    println!(
        "cargo:warning=Generating OpenAPI client for {} from {}",
        package_name,
        openapi_spec.display()
    );

    let status = Command::new("openapi-generator")
        .args(&[
            "generate",
            "-i",
            openapi_spec.to_str().unwrap(),
            "-g",
            "rust",
            "-o",
            output_dir,
            "--additional-properties",
            &format!("packageName={}", package_name),
            "--additional-properties=useSingleRequestParameter=true",
            "--additional-properties=hideGenerationTimestamp=true",
            "--additional-properties=prependFormOrBodyParameters=true",
            "--skip-validate-spec",
            "--global-property=skipFormModel=true",
            "--global-property=skipOperationExample=true",
            "--global-property=generateSourceCodeOnly=true",
        ])
        .status()
        .expect("Failed to run openapi-generator");

    if !status.success() {
        panic!("Failed to generate OpenAPI client for {}", package_name);
    }

    println!(
        "cargo:warning=OpenAPI client for {} generated successfully",
        package_name
    );
}

fn main() {
    /*
    OpenAPI generator is used to generate the client code in each external service being used for the app.
    I use it to save time and effort, as it generates the client code for the API endpoints
    and the models for the request and response bodies.
    */

    let enabled = false; // Set to true to enable OpenAPI client generation
    if !enabled {
        println!("cargo:warning=OpenAPI client generation is disabled");
        return;
    }

    let version_check = Command::new("openapi-generator").arg("version").status();
    if version_check.is_err() {
        println!("cargo:warning=openapi-generator not found. Please install it with: brew install openapi-generator");
        return;
    }

    let force = false; // Set to true to force regeneration of the client code

    generate_client(
        Path::new("api/specs/github.json"),
        "src/services/github/generated",
        "github_client",
        force,
    );

    println!("cargo:rerun-if-changed=api/specs/shodan.json");
    println!("cargo:rerun-if-changed=api/specs/github.json");
}
