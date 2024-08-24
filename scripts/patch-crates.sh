# source this file

update_lumos_dependencies() {
  declare project_root="$1"
  declare lumos_ver="$2"
  declare tomls=()
  while IFS='' read -r line; do tomls+=("$line"); done < <(find "$project_root" -name Cargo.toml)

  sed -i -e "s#\(lumos-program = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-program = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-program-test = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-program-test = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-sdk = \"\).*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-sdk = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-client = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-client = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-cli-config = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-cli-config = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-clap-utils = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-clap-utils = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-account-decoder = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-account-decoder = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-faucet = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-faucet = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-zk-token-sdk = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(lumos-zk-token-sdk = { version = \"\)[^\"]*\(\"\)#\1=$lumos_ver\2#g" "${tomls[@]}" || return $?
}

patch_crates_io_lumos() {
  declare Cargo_toml="$1"
  declare lumos_dir="$2"
  cat >> "$Cargo_toml" <<EOF
[patch.crates-io]
EOF
patch_crates_io_lumos_no_header "$Cargo_toml" "$lumos_dir"
}

patch_crates_io_lumos_no_header() {
  declare Cargo_toml="$1"
  declare lumos_dir="$2"
  cat >> "$Cargo_toml" <<EOF
lumos-account-decoder = { path = "$lumos_dir/account-decoder" }
lumos-clap-utils = { path = "$lumos_dir/clap-utils" }
lumos-client = { path = "$lumos_dir/client" }
lumos-cli-config = { path = "$lumos_dir/cli-config" }
lumos-program = { path = "$lumos_dir/sdk/program" }
lumos-program-test = { path = "$lumos_dir/program-test" }
lumos-sdk = { path = "$lumos_dir/sdk" }
lumos-faucet = { path = "$lumos_dir/faucet" }
lumos-zk-token-sdk = { path = "$lumos_dir/zk-token-sdk" }
EOF
}
