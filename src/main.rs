use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_keccak::{Hasher, Keccak};

#[derive(Parser)]
#[command(name = "stork-asset")]
#[command(about = "A small CLI tool for generating Stork asset configurations")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the authentication token
    SetToken {
        /// The authentication token to store
        token: String,
    },
    /// Get the current authentication token
    GetToken,
    /// Get available assets
    GetAssets,
    /// Generate an asset configuration file
    #[command(aliases = ["gen", "generate", "gen-config", "gen-conf"])]
    GenerateConfig {
        /// Output file path (must end in .yaml)
        #[arg(short = 'o', long = "output")]
        output: String,
        
        /// Number of random assets to include
        #[arg(short = 'r', long = "random")]
        random: Option<usize>,
        
        /// Comma-separated list of assets to include
        #[arg(short = 'a', long = "assets")]
        assets: Option<String>,

        /// Fallback period in seconds
        #[arg(short = 'f', long = "fallback", default_value = "60")]
        fallback_period: u64,

        /// Percent change threshold
        #[arg(short = 'p', long = "percent", default_value = "1.0")]
        percent_change: f64,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct AuthConfig {
    auth_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AssetConfig {
    asset_id: String,
    fallback_period_sec: u64,
    percent_change_threshold: f64,
    encoded_asset_id: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    assets: BTreeMap<String, AssetConfig>,
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Failed to get config directory");
    path.push("asset_conf");
    path.push("config.json");
    path
}

fn load_config() -> AuthConfig {
    let path = get_config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).expect("Failed to read config file");
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AuthConfig::default()
    }
}

fn save_config(config: &AuthConfig) {
    let path = get_config_path();
    fs::create_dir_all(path.parent().unwrap()).expect("Failed to create config directory");
    let content = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(path, content).expect("Failed to write config file");
}

fn validate_output_path(path: &str) -> Result<(), String> {
    if !path.to_lowercase().ends_with(".yaml") && !path.to_lowercase().ends_with(".yml") {
        return Err("Output file must have .yaml or .yml extension".to_string());
    }
    
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            return Err("Output directory does not exist".to_string());
        }
    }
    
    Ok(())
}

fn calculate_encoded_asset_id(asset_id: &str) -> String {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(asset_id.as_bytes());
    keccak.finalize(&mut output);
    format!("0x{}", hex::encode(output))
}

fn get_available_assets(token: &str) -> Result<Vec<String>, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", token))
            .expect("Invalid token format"),
    );

    match client
        .get("https://rest.jp.stork-oracle.network/v1/prices/assets")
        .headers(headers)
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                let response: serde_json::Value = response.json().unwrap();
                if let Some(assets) = response["data"].as_array() {
                    Ok(assets
                        .iter()
                        .filter_map(|a| a.as_str().map(String::from))
                        .collect())
                } else {
                    Err("Invalid response format from server".to_string())
                }
            } else {
                Err(format!("Server returned status {} - check your token with get-token and set-token", response.status()))
            }
        }
        Err(e) => Err(format!("Error making request: {}", e)),
    }
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        None => {
            println!("No command provided. Use --help to see available commands.");
        }
        Some(cmd) => {
            match cmd {
                Commands::SetToken { token } => {
                    let mut config = load_config();
                    config.auth_token = Some(token);
                    save_config(&config);
                    println!("Authentication token updated successfully");
                }
                Commands::GetToken => {
                    let config = load_config();
                    match config.auth_token {
                        Some(token) => println!("{}", token),
                        None => println!("No authentication token set"),
                    }
                }
                Commands::GetAssets => {
                    let config = load_config();
                    match config.auth_token {
                        Some(token) => {
                            let client = Client::new();
                            let mut headers = HeaderMap::new();
                            headers.insert(
                                AUTHORIZATION,
                                HeaderValue::from_str(&format!("Basic {}", token))
                                    .expect("Invalid token format"),
                            );

                            match client
                                .get("https://rest.jp.stork-oracle.network/v1/prices/assets")
                                .headers(headers)
                                .send()
                            {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        let response: serde_json::Value = response.json().unwrap();
                                        if let Some(assets) = response["data"].as_array() {
                                            println!("Assets:\n");
                                            for asset in assets {
                                                println!("{}", asset.as_str().unwrap_or("Invalid asset name"));
                                            }
                                            println!("\nTotal Assets: {}", assets.len());
                                        }
                                    } else {
                                        println!("Error: Server returned status {}", response.status());
                                        if response.status() == 401 {
                                            println!("Check your token with: \n\n   stork-asset get-token \n\nChange your token with: \n\n   stork-asset set-token <token>");
                                        }
                                    }
                                }
                                Err(e) => println!("Error making request: {}", e),
                            }
                        }
                        None => println!("No authentication token set. Set token with: \n\n   asset-conf set-token <token>"),
                    }
                }
                Commands::GenerateConfig { 
                    output, 
                    random, 
                    assets, 
                    fallback_period, 
                    percent_change 
                } => {
                    if let Err(e) = validate_output_path(&output) {
                        println!("Error: {}", e);
                        return;
                    }

                    let config = load_config();
                    match config.auth_token {
                        Some(token) => {
                            match get_available_assets(&token) {
                                Ok(mut available_assets) => {
                                    let mut selected_assets = Vec::new();

                                    // First, add specifically requested assets
                                    if let Some(asset_list) = assets {
                                        let requested_assets: Vec<_> = asset_list.split(',')
                                            .map(|s| s.trim().to_string())
                                            .collect();
                                        
                                        // Validate all assets exist
                                        for asset in &requested_assets {
                                            if !available_assets.contains(asset) {
                                                println!("Error: Asset '{}' not found in available assets", asset);
                                                return;
                                            }
                                            selected_assets.push(asset.clone());
                                            // Remove from available_assets to prevent duplicates in random selection
                                            if let Some(pos) = available_assets.iter().position(|x| x == asset) {
                                                available_assets.swap_remove(pos);
                                            }
                                        }
                                    }

                                    // Then add random assets if requested
                                    if let Some(n) = random {
                                        if n > 0 {
                                            if n > available_assets.len() {
                                                println!("Warning: Requested {} additional random assets but only {} are available", 
                                                    n, available_assets.len());
                                            }
                                            let mut rng = rand::thread_rng();
                                            selected_assets.extend(
                                                available_assets
                                                    .choose_multiple(&mut rng, n.min(available_assets.len()))
                                                    .cloned()
                                            );
                                        }
                                    }

                                    if selected_assets.is_empty() {
                                        println!("Error: No assets selected. Use -a and/or -r to specify assets");
                                        return;
                                    }

                                    let mut config_map = BTreeMap::new();
                                    for asset_id in selected_assets {
                                        let encoded_id = calculate_encoded_asset_id(&asset_id);
                                        let asset_config = AssetConfig {
                                            asset_id: asset_id.clone(),
                                            fallback_period_sec: fallback_period,
                                            percent_change_threshold: percent_change,
                                            encoded_asset_id: encoded_id,
                                        };
                                        config_map.insert(asset_id, asset_config);
                                    }

                                    let config = Config {
                                        assets: config_map,
                                    };

                                    let yaml_content = serde_yaml::to_string(&config)
                                        .expect("Failed to serialize to YAML");
                                    
                                    fs::write(&output, yaml_content)
                                        .map_err(|e| println!("Error writing file: {}", e))
                                        .ok();
                                    
                                    println!("Successfully generated config with {} assets at {}", config.assets.len(), output);
                                }
                                Err(e) => println!("Error: {}", e),
                            }
                        }
                        None => println!("No authentication token set. Set token with: \n\n   asset-conf set-token <token>"),
                    }
                }
            }
        }
    }
}