# shellcheck shell=bash
# x app framework — bash builtins preamble.
#
# `x` injects this file before each handler body so the script can call:
#   x-opt <name>           prints the value of an option (or "true" for flags)
#   x-arg <name>           prints the value of a positional argument
#   x-opts <assoc-name>    populates a caller-named bash assoc array (nameref)
#   x-args <assoc-name>    populates a caller-named bash assoc array (nameref)
#   x-run <cmd> [args...]  runs a command with x-* helpers in scope
#   x-usage <cmd-path>     prints generated --help for the given path
#   x-io-read [-v|--var NAME] [...]           prompt text; `-v NAME` assigns to global shell var
#   x-io-confirm [-v|--var NAME] [...]       yes/no; `-v NAME` assigns `true` or `false`
#   x-io-select [-v|--var NAME] [...]       pick id=label; `-v` assigns (string or indexed array when --multi)
#
# `x` exports the data via env vars:
#   X_OPT_<name>           value of option (repeats joined with newlines)
#   X_ARG_<name>           value of arg   (repeats joined with newlines)
#   X_OPTS_PAIRS           "k=v\n..." for all options (one per line)
#   X_ARGS_PAIRS           "k=v\n..." for all args
#   X_BIN, X_APP, X_APP_FILE
#
# Names with `-` in them are translated to `_` for env-var lookups.

x-opt() {
  local n="${1:?usage: x-opt <name>}"
  local var="X_OPT_${n//-/_}"
  printf '%s\n' "${!var-}"
}

x-arg() {
  local n="${1:?usage: x-arg <name>}"
  local var="X_ARG_${n//-/_}"
  printf '%s\n' "${!var-}"
}

x-opts() {
  local target="${1:?usage: x-opts <assoc-array-name>}"
  # shellcheck disable=SC2178
  local -n __ref="$target"
  __ref=()
  local line k v
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    k="${line%%=*}"
    v="${line#*=}"
    [[ -n "$k" ]] && __ref["$k"]="$v"
  done <<< "${X_OPTS_PAIRS-}"
}

x-args() {
  local target="${1:?usage: x-args <assoc-array-name>}"
  # shellcheck disable=SC2178
  local -n __ref="$target"
  __ref=()
  local line k v
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    k="${line%%=*}"
    v="${line#*=}"
    [[ -n "$k" ]] && __ref["$k"]="$v"
  done <<< "${X_ARGS_PAIRS-}"
}

x-run() {
  if [[ $# -lt 1 ]]; then
    echo "usage: x-run <command> [args...]" >&2
    return 2
  fi
  "$@"
}

x-usage() {
  local cmd_path="${1-}"
  "$X_BIN" __usage "$X_APP_FILE" "$cmd_path"
}

# Pop optional -v / --var from "$@"; forward args in global _x_io__parsed_fwd[],
# variable name in _x_io__parsed_var. Caller copies then unsets both.
# (No namerefs: macOS /bin/bash is 3.2 and does not support declare -n.)
_x_io_take_var_fwd() {
  _x_io__parsed_var=
  _x_io__parsed_fwd=()
  while (($#)); do
    case "$1" in
      -v | --var)
        [[ -n "${2:-}" ]] || {
          printf '%s\n' "x-io: ${1}: expected a shell variable name" >&2
          return 2
        }
        if [[ -n "$_x_io__parsed_var" ]]; then
          printf '%s\n' "x-io: multiple -v / --var is not supported" >&2
          return 2
        fi
        [[ "$2" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]] || {
          printf '%s\n' "x-io: invalid variable name: $2" >&2
          return 2
        }
        _x_io__parsed_var=$2
        shift 2
        ;;
      --var=*)
        if [[ -n "$_x_io__parsed_var" ]]; then
          printf '%s\n' "x-io: multiple -v / --var is not supported" >&2
          return 2
        fi
        local vn=${1#--var=}
        [[ "$vn" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]] || {
          printf '%s\n' "x-io: invalid variable name: $vn" >&2
          return 2
        }
        _x_io__parsed_var=$vn
        shift
        ;;
      *)
        _x_io__parsed_fwd+=("$1")
        shift
        ;;
    esac
  done
}

# Global scalar; NAME must match ^[a-zA-Z_][a-zA-Z0-9_]*$
_x_io_assign_global_scalar() {
  eval "$1=$(printf '%q' "$2")"
}

# Global indexed array from line-oriented file (bash 3.2; no mapfile / declare -g).
_x_io_assign_global_array_from_file() {
  local name=$1 file=$2 i=0 line
  eval "$name=()"
  while IFS= read -r line || [[ -n "$line" ]]; do
    eval "${name}[$i]=$(printf '%q' "$line")"
    i=$((i + 1))
  done <"$file"
}

x-io-read() {
  local -a fwd=()
  local var=
  _x_io_take_var_fwd "$@" || return "$?"
  fwd=("${_x_io__parsed_fwd[@]}")
  var=${_x_io__parsed_var:-}
  unset _x_io__parsed_fwd
  unset _x_io__parsed_var
  if [[ -z "$var" ]]; then
    "${X_BIN:?}" __io read "${fwd[@]}"
    return $?
  fi
  local out ec
  out="$("${X_BIN:?}" __io read "${fwd[@]}")"
  ec=$?
  [[ "$ec" -eq 0 ]] || return "$ec"
  _x_io_assign_global_scalar "$var" "$out"
  return 0
}

x-io-confirm() {
  local -a fwd=()
  local var=
  _x_io_take_var_fwd "$@" || return "$?"
  fwd=("${_x_io__parsed_fwd[@]}")
  var=${_x_io__parsed_var:-}
  unset _x_io__parsed_fwd
  unset _x_io__parsed_var
  if [[ -z "$var" ]]; then
    "${X_BIN:?}" __io confirm "${fwd[@]}"
    return $?
  fi
  local out ec
  out="$("${X_BIN:?}" __io confirm "${fwd[@]}")"
  ec=$?
  [[ "$ec" -eq 0 ]] || return "$ec"
  _x_io_assign_global_scalar "$var" "$out"
  return 0
}

x-io-select() {
  local -a fwd=()
  local var=
  local multi=0 tok
  _x_io_take_var_fwd "$@" || return "$?"
  fwd=("${_x_io__parsed_fwd[@]}")
  var=${_x_io__parsed_var:-}
  unset _x_io__parsed_fwd
  unset _x_io__parsed_var
  for tok in "${fwd[@]}"; do
    if [[ "$tok" == --multi ]]; then
      multi=1
      break
    fi
  done
  if [[ -z "$var" ]]; then
    "${X_BIN:?}" __io select "${fwd[@]}"
    return $?
  fi
  if [[ "$multi" -eq 1 ]]; then
    local tmp ec
    tmp=$(mktemp "${TMPDIR:-/tmp}/x-io-select.XXXXXX") || return 2
    "${X_BIN:?}" __io select "${fwd[@]}" >"$tmp"
    ec=$?
    if [[ "$ec" -ne 0 ]]; then
      rm -f "$tmp"
      return "$ec"
    fi
    _x_io_assign_global_array_from_file "$var" "$tmp"
    rm -f "$tmp"
    return 0
  fi
  local out ec
  out="$("${X_BIN:?}" __io select "${fwd[@]}")"
  ec=$?
  [[ "$ec" -eq 0 ]] || return "$ec"
  _x_io_assign_global_scalar "$var" "$out"
  return 0
}

export -f x-opt x-arg x-opts x-args x-run x-usage x-io-read x-io-confirm x-io-select
