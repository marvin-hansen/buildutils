# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# https://users.rust-lang.org/t/ive-ran-clippy-on-100k-crates/117127
# https://www.reddit.com/r/rust/comments/a4wblu/how_to_configure_clippy_to_be_as_annoying_as/
MIGRATION_DATA="migrations"  cargo clippy  --workspace --all-targets --fix --allow-dirty --exclude proto_cmdb --exclude proto_imdb --exclude proto_mddb --exclude  proto_smdb -- -Wclippy::all
