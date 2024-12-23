# Stork Asset CLI

A small CLI for generating Stork Oracle asset configurations.

## Installation

Install the CLI with:

```bash
curl -fsSL https://raw.githubusercontent.com/HenryMBaldwin/stork-asset-cli/refs/heads/master/install.sh | sudo bash
```

or clone down this repository and install the CLI with cargo:

```bash
git clone https://github.com/henrymbaldwin/stork-asset-cli
cd stork-asset-cli
```
Install the CLI with cargo:

```bash
cargo install --path .
```


## Usage

```bash
stork-asset --help
```

```bash
stork-asset --version
```

```bash
stork-asset update
```

### Authentication

Before using the tool, you need to set up your auth token for use with the stork rest api:

```bash
stork-asset set-token <token>
```
and confirm with:

```bash
stork-asset get-token
```

### Getting Asset Information

You can get all available assets with:

```bash
stork-asset get-assets
```
and optionally add the `-e` flag to get the encoded asset IDs as well:

```bash
stork-asset get-assets -e
```
You can also get the encoded asset IDs for specific assets with:

```bash
stork-asset get-encoded -a <asset_id1>,<asset_id2>,...
```
You can check if specific assets are available with:

```bash
stork-asset check <asset_id1>,<asset_id2>,...
```

### Generating an Asset Configuration

You can generate an asset configuration with:

```bash
stork-asset gen-config\
-r <Number of Random Assets>\
-a <Specific Comma Separated Asset IDs>\
-o <Output YAMLFile>
```

where one or both of -r and -a must be provided, and -o is required.

Optionally, you can provide:

```bash
-f <Fallback Period in Seconds>\
-p <Percentage Change Threshold>
```

## Example

#### Generate config with 5 random assets
```bash
stork-asset gen-config -r 5 -o config.yaml
```

#### Generate config with specific assets
```bash
stork-asset gen-config -a BTCUSD,ETHUSD,SUIUSD -o config.yaml
```

#### Combine specific with additional random assets
```bash
stork-asset gen-config -a BTCUSD,ETHUSD,SUIUSD -r 5 -o config.yaml
```

#### Customize fallback period and percentage change threshold
```bash
stork-asset gen-config -a BTCUSD,ETHUSD,SUIUSD -r 5 -f 120 -p 0.05 -o config.yaml
```

### Example Output
bash:
```bash
stork-asset gen-config -a BTCUSD,ETHUSD,SUIUSD -o config.yaml
```
config.yaml:
```yaml
assets:
  BTCUSD:
    asset_id: BTCUSD
    fallback_period_sec: 60
    percent_change_threshold: 1.0
    encoded_asset_id: 0x7404e3d104ea7841c3d9e6fd20adfe99b4ad586bc08d8f3bd3afef894cf184de
  ETHUSD:
    asset_id: ETHUSD
    fallback_period_sec: 60
    percent_change_threshold: 1.0
    encoded_asset_id: 0x59102b37de83bdda9f38ac8254e596f0d9ac61d2035c07936675e87342817160
  SUIUSD:
    asset_id: SUIUSD
    fallback_period_sec: 60
    percent_change_threshold: 1.0
    encoded_asset_id: 0xa24cc95a4f3d70a0a2f7ac652b67a4a73791631ff06b4ee7f729097311169b81
```
## Development

To run the cli locally, you can use the following command:

```bash
cargo run -- <args>
```

To install the cli locally, you can use the following command:

```bash
cargo install --path .
```

To create a new release compatible with the github actions workflow, you can do the following:

<ol>
<li>Update the version in the <code>Cargo.toml</code> file</li>
<li>Update the version in the <code>src/main.rs</code> file</li>
<li>Tag the release with the new version using <code>git tag -a v&lt;version&gt; -m "&lt;version&gt; Release"</code></li>
<li>Push the tag to the remote repository using <code>git push origin v&lt;version&gt;</code></li>
</ol>

This will trigger the github actions workflow to build and release the new version.