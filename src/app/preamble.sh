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
#   x-prt [(-s|--style) <style>] [<text>...]  styled print via ANSI SGR codes
#   x-tui [--init|--exit|--clear|...] [<text>...]  terminal control via ANSI escape sequences
#   x-env-load .<group>   load a named env group from the app `env:` block
#   x-path-root             print the directory containing the app `.x.yml` file
#
# `x` exports the data via env vars:
#   X_OPT_<name>           value of option (repeats joined with newlines)
#   X_ARG_<name>           value of arg   (repeats joined with newlines)
#   X_OPTS_PAIRS           "k=v\n..." for all options (one per line)
#   X_ARGS_PAIRS           "k=v\n..." for all args
#   X_BIN, X_APP, X_APP_FILE, X_PATH_ROOT
#   X_ENV_GROUP_<name>   newline-separated KEY=VALUE pairs for named env groups
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

x-path-root() {
  printf '%s\n' "${X_PATH_ROOT:?}"
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

# Map one style token to an SGR numeric code. Prints code to stdout; return 1 if unknown.
_x_style_token_to_code() {
  case "$1" in
    reset | normal) printf '%s' 0 ;;
    bold) printf '%s' 1 ;;
    dim | faint) printf '%s' 2 ;;
    italic) printf '%s' 3 ;;
    underline) printf '%s' 4 ;;
    blink) printf '%s' 5 ;;
    inverse | reverse) printf '%s' 7 ;;
    hidden) printf '%s' 8 ;;
    strikethrough | strike) printf '%s' 9 ;;
    black) printf '%s' 30 ;;
    red) printf '%s' 31 ;;
    green) printf '%s' 32 ;;
    yellow) printf '%s' 33 ;;
    blue) printf '%s' 34 ;;
    magenta) printf '%s' 35 ;;
    cyan) printf '%s' 36 ;;
    white) printf '%s' 37 ;;
    default) printf '%s' 39 ;;
    bright-black) printf '%s' 90 ;;
    bright-red) printf '%s' 91 ;;
    bright-green) printf '%s' 92 ;;
    bright-yellow) printf '%s' 93 ;;
    bright-blue) printf '%s' 94 ;;
    bright-magenta) printf '%s' 95 ;;
    bright-cyan) printf '%s' 96 ;;
    bright-white) printf '%s' 97 ;;
    bg-black) printf '%s' 40 ;;
    bg-red) printf '%s' 41 ;;
    bg-green) printf '%s' 42 ;;
    bg-yellow) printf '%s' 43 ;;
    bg-blue) printf '%s' 44 ;;
    bg-magenta) printf '%s' 45 ;;
    bg-cyan) printf '%s' 46 ;;
    bg-white) printf '%s' 47 ;;
    bg-default) printf '%s' 49 ;;
    bg-bright-black) printf '%s' 100 ;;
    bg-bright-red) printf '%s' 101 ;;
    bg-bright-green) printf '%s' 102 ;;
    bg-bright-yellow) printf '%s' 103 ;;
    bg-bright-blue) printf '%s' 104 ;;
    bg-bright-magenta) printf '%s' 105 ;;
    bg-bright-cyan) printf '%s' 106 ;;
    bg-bright-white) printf '%s' 107 ;;
    *) return 1 ;;
  esac
}

# Convert comma-separated style names to an opening SGR sequence (e.g. \033[1;31m).
_x_style_to_sgr() {
  local style_csv=$1
  local -a codes=()
  local token code IFS=,
  for token in $style_csv; do
    token="${token#"${token%%[![:space:]]*}"}"
    token="${token%"${token##*[![:space:]]}"}"
    [[ -z "$token" ]] && continue
    code=$(_x_style_token_to_code "$token") || {
      printf '%s\n' "x-prt: unknown style: $token" >&2
      return 2
    }
    codes+=("$code")
  done
  if ((${#codes[@]} == 0)); then
    printf '\033[0m'
    return 0
  fi
  local IFS=';'
  printf '\033[%sm' "${codes[*]}"
}

x-prt() {
  local sgr=
  local is_print_newline=0
  while (($#)); do
    case "$1" in
      -s | --style)
        [[ -n "${2:-}" ]] || {
          printf '%s\n' "usage: x-prt [(-s|--style) <style>] [<text> ...]" >&2
          return 2
        }
        sgr=$(_x_style_to_sgr "$2") || return $?
        shift 2
        ;;
      -s=* | --style=*)
        sgr=$(_x_style_to_sgr "${1#*=}") || return $?
        shift
        ;;
      -n | --newline)
        is_print_newline=1
        shift
        ;;
      *)
        if [[ -n "$sgr" ]]; then
          printf '%b%s\033[0m' "$sgr" "$1"
        else
          printf '%s' "$1"
        fi
        shift
        ;;
    esac
    if [[ "$is_print_newline" -eq 1 ]]; then
      echo
    fi
  done
}

x-tui() {
  while (($# > 0)); do
    case "$1" in
      --init) printf '\033[?1049h\033[H\033[?25l'; shift ;;
      --exit) printf '\033[?1049l\033[?25h'; shift ;;
      --clear) printf '\033[2J'; shift ;;
      --clear-line) printf '\033[2K'; shift ;;
      --home) printf '\033[H'; shift ;;
      --hide) printf '\033[?25l'; shift ;;
      --show) printf '\033[?25h'; shift ;;
      --save) printf '\033[s'; shift ;;
      --restore) printf '\033[u'; shift ;;
      --move)
        [[ -n "${2:-}" ]] || {
          printf '%s\n' "x-tui: --move: expected row,col" >&2
          return 2
        }
        local row col IFS=,
        read -r row col <<<"$2"
        [[ -n "$row" && -n "$col" && "$row" =~ ^[0-9]+$ && "$col" =~ ^[0-9]+$ ]] || {
          printf '%s\n' "x-tui: --move: expected row,col integers" >&2
          return 2
        }
        printf '\033[%d;%dH' "$row" "$col"
        shift 2
        ;;
      --up)
        [[ -n "${2:-}" && "$2" =~ ^[0-9]+$ ]] || {
          printf '%s\n' "x-tui: --up: expected count" >&2
          return 2
        }
        printf '\033[%dA' "$2"
        shift 2
        ;;
      --down)
        [[ -n "${2:-}" && "$2" =~ ^[0-9]+$ ]] || {
          printf '%s\n' "x-tui: --down: expected count" >&2
          return 2
        }
        printf '\033[%dB' "$2"
        shift 2
        ;;
      --right)
        [[ -n "${2:-}" && "$2" =~ ^[0-9]+$ ]] || {
          printf '%s\n' "x-tui: --right: expected count" >&2
          return 2
        }
        printf '\033[%dC' "$2"
        shift 2
        ;;
      --left)
        [[ -n "${2:-}" && "$2" =~ ^[0-9]+$ ]] || {
          printf '%s\n' "x-tui: --left: expected count" >&2
          return 2
        }
        printf '\033[%dD' "$2"
        shift 2
        ;;
      --no-wrap) printf '\033[?7l'; shift ;;
      --wrap) printf '\033[?7h'; shift ;;
      *) printf '%s' "$1"; shift ;;
    esac
  done
}

x-env-load() {
  local group="${1:?usage: x-env-load .<group>}"
  group="${group#.}"
  local var="X_ENV_GROUP_${group}"
  local pairs="${!var-}"
  if [[ -z "$pairs" ]]; then
    echo "x-env-load: unknown env group .${group}" >&2
    return 1
  fi
  local line k v
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    k="${line%%=*}"
    v="${line#*=}"
    [[ -n "$k" ]] && export "$k=$v"
  done <<< "$pairs"
}

export -f x-opt x-arg x-opts x-args x-run x-usage x-io-read x-io-confirm x-io-select x-prt x-tui x-env-load x-path-root
