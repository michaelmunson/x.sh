
export const EXAPP_X = `
# Comprehensive sample app for the \`x\` app framework (\`x exapp …\`).
# Demonstrates: metadata, root options/arguments, nested commands, YAML anchors,
# every synopsis form from the cheat-sheet, and split \`$.import\` handler files.

name: exapp
version: 0.1.0
description: >-
  Exercise app — exercises options, positionals, nesting, validation,
  \`$.import\`, and bash helpers (\`x-opt\`, \`x-arg\`, \`x-opts\`, \`x-args\`, \`x-run\`, \`x-usage\`).

options:
  - "[-v | --version]"
  - "[-g | --global <note=''>]"

arguments:
  - "[<topic='overview'>]"

commands:
  demo:
    description: nested demos (each subcommand highlights different DSL pieces)

    commands:
      opts:
        description: >-
          Option forms — bool pair, value, defaulted value, repeating values,
          enum choice, \`=\` default, requires-chain, and a required bare flag.
        options:
          - "[-n | --dry-run]"
          - "[-o | --out <path>]"
          - "[-c | --count <n='1'>]"
          - "[-D | --define <kv> ...]"
          - "[--kind={alpha|beta|gamma}]"
          - "[--label=<text='demo'>]"
          - "[--src=<path> [--dst=<path>]]"
          - "[--default={yes|no}]"
          - "--commit"

      args:
        description: >-
          Positional forms — required, optional, optional-with-default,
          and greedy repeat (must be last).
        arguments:
          - "<one> [<two>] [<three='3'>] [<rest>...]"

      pick:
        description: required choice — \`(alt1|alt2|alt3)\` becomes positional name \`choice\`
        arguments:
          - "(north|south|east|west)"

      tail:
        description: optional repeating positional \`[<items>...]\`
        arguments:
          - "[<items>...]"

      group:
        description: >-
          Non-leaf group — invoke \`x exapp demo group\` with no subcommand to see auto-help.
        commands:
          alpha:
            description: nested leaf α — shares options via YAML anchor (see \`reflect\`)
            options: &demo_verbose
              - "[-V | --verbose]"
            arguments:
              - "<msg>"
          beta:
            description: nested leaf β — same shared options as \`alpha\`
            options: *demo_verbose
            arguments:
              - "<msg>"

      reflect:
        description: dump parsed options/args via \`x-opts\` / \`x-args\` (and \`x-run\`)
        options: *demo_verbose
        arguments:
          - "[<label='state'>]"

      sparse:
        description: optional repeat with a default (see also \`tail\` for plain \`[<items>...]\`)
        arguments:
          - "[<parts='*'>...]"

      usage-tip:
        description: >-
          One-line handler — forwards to x-usage (same idea as README create → x-usage create).

      io:
        description: interactive builtins
        commands:
          read:
            description: read a line of text
          confirm:
            description: confirm a yes/no question
          select:
            description: select an option from a list
          multiselect:
            description: select an option from a list

$.import:
  - ./exapp.handlers-a.yml
  - ./exapp.handlers-b.yml
`;
export const EXAPP_HANDLERS_A = `
# Handlers merged via \`$.import\` (first file). Keys are dotted command paths.

"":
  |
    topic=$(x-arg topic)
    note=$(x-opt global)
    echo "exapp — topic=\${topic:-overview}  global-note=\${note:-∅}"
    echo "Try:  x exapp --help"
    echo "       x exapp demo --help"

demo.opts:
  |
    echo "dry-run=$(x-opt dry-run)"
    echo "out=$(x-opt out)"
    echo "count=$(x-opt count)"
    mapfile -t defs < <(x-opt define)
    printf 'define[%s]\n' "\${defs[@]:-}"
    echo "kind=$(x-opt kind)"
    echo "label=$(x-opt label)"
    echo "src=$(x-opt src)"
    echo "dst=$(x-opt dst)"
    echo "commit=$(x-opt commit)"

demo.args:
  |
    echo "one=$(x-arg one)"
    echo "two=$(x-arg two)"
    echo "three=$(x-arg three)"
    mapfile -t rest < <(x-arg rest)
    printf 'rest[%s]\n' "\${rest[@]:-}"

demo.pick:
  |
    echo "direction=$(x-arg choice)"

demo.tail:
  |
    mapfile -t items < <(x-arg items)
    if ((\${#items[@]})); then
      printf 'items: %s\n' "\${items[@]}"
    else
      echo "(no items)"
    fi

demo.sparse:
  |
    mapfile -t parts < <(x-arg parts)
    echo "parts=\${parts[*]}"

demo.usage-tip: x-usage demo.opts
`;
export const EXAPP_HANDLERS_B = `
# Second import file — merged keys must not duplicate the first import.

demo.group.alpha:
  |
    msg=$(x-arg msg)
    if [[ $(x-opt verbose) == true ]]; then
      echo "[verbose] alpha ← $msg"
    else
      echo "alpha ← $msg"
    fi

demo.group.beta:
  |
    msg=$(x-arg msg)
    if [[ $(x-opt verbose) == true ]]; then
      echo "[verbose] beta ← $msg"
    else
      echo "beta ← $msg"
    fi

demo.reflect:
  |
    declare -A opts args
    x-opts opts
    x-args args
    echo "--- x-opts ($(x-arg label)) ---"
    for k in "\${!opts[@]}"; do echo "  $k=\${opts[$k]}"; done | sort
    echo "--- x-args ---"
    for k in "\${!args[@]}"; do echo "  $k=\${args[$k]}"; done | sort
    echo "--- x-run ---"
    x-run echo "ran nested echo via x-run"

# IO commands
demo.io.read: |
  answer=$(x-io-read "What is your favorite color?")
  if [ "$answer" != "blue" ]; then
    echo "Wrong answer"
    exit 1
  else
    echo "Correct!"
  fi

demo.io.confirm: |
  answer=$(x-io-confirm "Are you sure you want to submit?" --default no)
  echo "Submit = $answer"
  answer=$(x-io-confirm "Are you sure you want to delete?" --default yes)
  echo "Delete = $answer"

demo.io.select: |
  x-io-select "Choose a color" \
    "red=Red" \
    "green=Green" \
    "blue=Blue" \
    "yellow/orange=Yellow or Orange" \
    -v color
  echo "Color = $color"

demo.io.multiselect: |
  x-io-select --multi "Choose a color" \
    "red=Red" \
    "green=Green" \
    "blue=Blue" \
    "yellow/orange=Yellow or Orange" \
    -v colors 
  
  echo "First Color = \${colors[0]}"
  echo "All Colors = \${colors[*]}"
`;
