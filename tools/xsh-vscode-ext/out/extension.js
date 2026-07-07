"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const yaml = __importStar(require("js-yaml"));
const RESERVED_ROOT_KEYS = new Set([
    'name', 'version', 'description', 'help', 'dir', 'env', 'import',
    'options', 'opts', 'arguments', 'args', '$', '$.import',
]);
const RESERVED_COMMAND_KEYS = new Set([
    'description', 'help', 'options', 'opts', 'arguments', 'args',
    '$', 'dir', 'env', 'alias',
]);
function stripRepeatSuffix(s) {
    const trimmed = s.trimEnd();
    if (trimmed.endsWith('...')) {
        return [trimmed.slice(0, -3).trimEnd(), true];
    }
    return [trimmed, false];
}
function isOptionParenGroup(inner) {
    return inner
        .split('|')
        .map(p => p.trim())
        .filter(p => p.length > 0)
        .every(seg => seg.startsWith('-'));
}
function collectMutexAlternatives(tokens) {
    const alternatives = [];
    let i = 0;
    while (i < tokens.length) {
        if (tokens[i] === '|') {
            i++;
            continue;
        }
        const alt = [tokens[i]];
        i++;
        while (true) {
            if (i < tokens.length && tokens[i] === '|') {
                const prev = alt[alt.length - 1] ?? '';
                if (prev.startsWith('-') && prev.length === 2 && i + 1 < tokens.length && tokens[i + 1].startsWith('--')) {
                    i++;
                    alt.push(tokens[i]);
                    i++;
                    continue;
                }
                break;
            }
            if (i >= tokens.length || tokens[i] === '|')
                break;
            alt.push(tokens[i]);
            i++;
        }
        alternatives.push(alt);
    }
    return alternatives;
}
/** Split a multiline `opts:`/`args:` string into individual expressions, or pass an array through as-is. */
function normalizeSynopsisValue(value) {
    if (typeof value === 'string') {
        return value
            .split('\n')
            .map(s => s.trim())
            .filter(s => s.length > 0);
    }
    if (Array.isArray(value)) {
        return value.filter((v) => typeof v === 'string');
    }
    return [];
}
const DIAG = 'x.sh';
/** `# x.sh` first line — kept for documents without a `.x.yml`/`x.yml` filename. */
const XSH_LOCAL_FIRST_LINE = /^#\s*x\.sh\b/i;
function hasLocalFirstLine(document) {
    return document.lineCount > 0 && XSH_LOCAL_FIRST_LINE.test(document.lineAt(0).text);
}
function isXshDocument(document) {
    if (document.languageId === 'xsh' || document.languageId === 'xsh-local') {
        return true;
    }
    if (document.languageId === 'yaml' && hasLocalFirstLine(document)) {
        return true;
    }
    return false;
}
// ─── Extension Lifecycle ──────────────────────────────────────────────────────
let diagnosticCollection;
function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('xsh');
    context.subscriptions.push(diagnosticCollection);
    vscode.workspace.textDocuments.forEach(lintDocument);
    context.subscriptions.push(vscode.workspace.onDidOpenTextDocument(lintDocument), vscode.workspace.onDidChangeTextDocument(e => lintDocument(e.document)), vscode.workspace.onDidCloseTextDocument(doc => diagnosticCollection.delete(doc.uri)));
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.text = '$(symbol-misc) x.sh';
    statusBar.tooltip = 'x.sh extension active';
    context.subscriptions.push(statusBar);
    vscode.window.onDidChangeActiveTextEditor(editor => {
        if (editor && isXshDocument(editor.document)) {
            statusBar.show();
        }
        else {
            statusBar.hide();
        }
    });
    if (vscode.window.activeTextEditor && isXshDocument(vscode.window.activeTextEditor.document)) {
        statusBar.show();
    }
    console.log('x.sh extension activated');
}
function deactivate() {
    diagnosticCollection?.dispose();
}
// ─── Linting ─────────────────────────────────────────────────────────────────
//
// Both apps (*.x.yml) and project-local scripts (x.yml) share the same
// grammar in v3: commands are `.name:` keys (string shorthand for `$`, or a
// mapping with help/args/opts/dir/env/alias/$ and further `.name:` children).
// This mirrors the structural checks in src/app/loader.rs and
// src/app/validate.rs.
function lintDocument(document) {
    if (!isXshDocument(document))
        return;
    const config = vscode.workspace.getConfiguration('xsh.lint');
    if (!config.get('enabled', true)) {
        diagnosticCollection.delete(document.uri);
        return;
    }
    const diagnostics = [];
    const text = document.getText();
    let parsed;
    try {
        parsed = yaml.load(text);
    }
    catch (e) {
        const yamlError = e;
        const line = yamlError.mark?.line ?? 0;
        const col = yamlError.mark?.column ?? 0;
        const range = new vscode.Range(line, col, line, col + 1);
        diagnostics.push(new vscode.Diagnostic(range, `${DIAG}: YAML parse error: ${yamlError.message}`, vscode.DiagnosticSeverity.Error));
        diagnosticCollection.set(document.uri, diagnostics);
        return;
    }
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
        diagnosticCollection.set(document.uri, diagnostics);
        return;
    }
    const root = parsed;
    if ('commands' in root) {
        const line = findKeyLine(document, 'commands');
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: "commands:" was removed in v3 — define commands with ".command-name:" keys instead`, vscode.DiagnosticSeverity.Error));
    }
    if (root.$ !== undefined && typeof root.$ === 'object' && root.$ !== null && !Array.isArray(root.$)) {
        const line = findKeyLine(document, '$');
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: top-level "$:" handler map was removed in v3 — define scripts inline with "$:" under each command`, vscode.DiagnosticSeverity.Error));
    }
    if (root.name !== undefined && typeof root.name === 'string' && !/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(root.name)) {
        const nameLine = findKeyLine(document, 'name');
        diagnostics.push(new vscode.Diagnostic(lineRange(document, nameLine), `${DIAG}: "name" must start with a letter and contain only letters, digits, hyphens, and underscores`, vscode.DiagnosticSeverity.Error));
    }
    if (root.version !== undefined && typeof root.version === 'string' && !/^\d+\.\d+\.\d+/.test(root.version)) {
        const verLine = findKeyLine(document, 'version');
        diagnostics.push(new vscode.Diagnostic(lineRange(document, verLine), `${DIAG}: "version" should use semantic versioning (e.g. 1.0.0)`, vscode.DiagnosticSeverity.Warning));
    }
    const importBlock = root.import;
    if (root['$.import'] !== undefined && importBlock?.$ !== undefined) {
        const line = findKeyLine(document, '$.import') || findKeyLine(document, 'import');
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: cannot use both "$.import:" and "import.$:" in the same file`, vscode.DiagnosticSeverity.Error));
    }
    const hasHandlerImport = Boolean(root['$.import'] ||
        (importBlock?.$ && (Array.isArray(importBlock.$) ? importBlock.$.length : true)));
    checkAliasPair(document, root, '(root)', 'description', 'help', diagnostics);
    checkAliasPair(document, root, '(root)', 'options', 'opts', diagnostics);
    checkAliasPair(document, root, '(root)', 'arguments', 'args', diagnostics);
    for (const key of Object.keys(root)) {
        if (RESERVED_ROOT_KEYS.has(key))
            continue;
        if (key.startsWith('.'))
            continue;
        const line = findKeyLine(document, key);
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: unknown key "${key}" — use a dot-prefixed command key (e.g. ".${key}:") or a reserved property`, vscode.DiagnosticSeverity.Error));
    }
    const options = normalizeSynopsisValue(root.opts ?? root.options);
    const args = normalizeSynopsisValue(root.args ?? root.arguments);
    validateCommandScope(document, '(root)', options, args, diagnostics);
    const rootChildren = dotCommandEntries(root);
    const hasRootScript = typeof root.$ === 'string';
    if (rootChildren.length === 0 && !hasRootScript && !hasHandlerImport) {
        diagnostics.push(new vscode.Diagnostic(lineRange(document, 0), `${DIAG}: this file defines no commands and no root "$:" script`, vscode.DiagnosticSeverity.Warning));
    }
    for (const [name, value] of rootChildren) {
        lintCommandNode(document, name, value, config, hasHandlerImport, diagnostics);
    }
    diagnosticCollection.set(document.uri, diagnostics);
}
/** `.name:` entries of a mapping node, in declaration order. */
function dotCommandEntries(node) {
    return Object.entries(node).filter(([key]) => key.startsWith('.') && key.length > 1);
}
function checkAliasPair(document, node, path, a, b, diagnostics) {
    if (node[a] !== undefined && node[b] !== undefined) {
        const line = findKeyLine(document, b);
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: cannot use both "${a}:" and "${b}:" on ${path}`, vscode.DiagnosticSeverity.Error));
    }
}
function lintCommandNode(document, path, value, config, hasHandlerImport, diagnostics) {
    const displayPath = path.replace(/^\./, '').replace(/\.\./g, '.');
    const leaf = path.split('.').pop() ?? path;
    if (typeof value === 'string') {
        // Shorthand: `.cmd: <script>` — equivalent to `.cmd: { $: <script> }`.
        if (!value.trim()) {
            const line = findCommandLine(document, leaf);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: script for "${displayPath}" is empty`, vscode.DiagnosticSeverity.Warning));
        }
        return;
    }
    if (!value || typeof value !== 'object' || Array.isArray(value)) {
        const line = findCommandLine(document, leaf);
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: "${displayPath}" must be a script string or a mapping`, vscode.DiagnosticSeverity.Error));
        return;
    }
    const node = value;
    for (const key of Object.keys(node)) {
        if (RESERVED_COMMAND_KEYS.has(key))
            continue;
        if (key.startsWith('.'))
            continue;
        const line = findNestedKeyLine(document, leaf, key);
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: unknown key "${key}" on command "${displayPath}" — use a dot-prefixed subcommand key (e.g. ".${key}:")`, vscode.DiagnosticSeverity.Error));
    }
    checkAliasPair(document, node, `"${displayPath}"`, 'description', 'help', diagnostics);
    checkAliasPair(document, node, `"${displayPath}"`, 'options', 'opts', diagnostics);
    checkAliasPair(document, node, `"${displayPath}"`, 'arguments', 'args', diagnostics);
    const children = dotCommandEntries(node);
    const hasScript = typeof node.$ === 'string';
    const hasAlias = typeof node.alias === 'string';
    if (hasAlias) {
        const extras = ['options', 'opts', 'arguments', 'args', 'dir', 'env', '$'].filter(k => node[k] !== undefined);
        if (extras.length > 0 || children.length > 0) {
            const line = findCommandLine(document, leaf);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: alias command "${displayPath}" may only define "alias" and "help"/"description"`, vscode.DiagnosticSeverity.Error));
        }
    }
    else {
        if (config.get('warnOnMissingHandlers', true) && children.length === 0 && !hasScript && !hasHandlerImport) {
            const line = findCommandLine(document, leaf);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: leaf command "${displayPath}" has no script — add a "$: ..." entry (or "alias:")`, vscode.DiagnosticSeverity.Error));
        }
        if (config.get('requireDescription', false) && node.description === undefined && node.help === undefined) {
            const line = findCommandLine(document, leaf);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: command "${displayPath}" is missing a description`, vscode.DiagnosticSeverity.Warning));
        }
        const options = normalizeSynopsisValue(node.opts ?? node.options);
        const args = normalizeSynopsisValue(node.args ?? node.arguments);
        validateCommandScope(document, `"${displayPath}"`, options, args, diagnostics);
    }
    for (const [name, child] of children) {
        lintCommandNode(document, `${path}${name}`, child, config, hasHandlerImport, diagnostics);
    }
}
// ─── Per-command validation (mirrors src/app/validate.rs) ────────────────────
function validateCommandScope(document, path, options, argumentsList, diagnostics) {
    lintOptionExpressions(document, options, diagnostics);
    const parsedOptions = [];
    const parsedGroups = [];
    for (const opt of options) {
        try {
            const parsed = parseOptionExpression(opt);
            parsedOptions.push(...parsed.options);
            parsedGroups.push(...parsed.groups);
        }
        catch (e) {
            const message = e instanceof Error ? e.message : String(e);
            const line = findOptionLine(document, opt);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: ${message}`, vscode.DiagnosticSeverity.Error));
        }
    }
    const seenLong = new Set();
    const seenShort = new Set();
    const definedLongs = new Set(parsedOptions.flatMap(o => (o.long ? [o.long] : [])));
    for (const opt of parsedOptions) {
        if (opt.long) {
            if (seenLong.has(opt.long)) {
                const line = findOptionByLong(document, opt.long, path);
                diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: duplicate option \`--${opt.long}\``, vscode.DiagnosticSeverity.Error));
            }
            seenLong.add(opt.long);
        }
        if (opt.short) {
            if (seenShort.has(opt.short)) {
                const line = findOptionByShort(document, opt.short, path);
                diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: duplicate option \`-${opt.short}\``, vscode.DiagnosticSeverity.Error));
            }
            seenShort.add(opt.short);
        }
    }
    for (const opt of parsedOptions) {
        for (const req of opt.requires) {
            if (!definedLongs.has(req)) {
                const line = findOptionLine(document, `--${req}`);
                const name = opt.long ? `--${opt.long}` : `-${opt.short}`;
                diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: option \`${name}\` requires \`--${req}\`, but \`--${req}\` is not defined on this command`, vscode.DiagnosticSeverity.Error));
            }
        }
    }
    for (const group of parsedGroups) {
        for (const member of group.members) {
            if (!definedLongs.has(member)) {
                const line = findOptionLine(document, `--${member}`);
                diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: option group references \`--${member}\`, but \`--${member}\` is not defined on this command`, vscode.DiagnosticSeverity.Error));
            }
        }
    }
    lintArgumentExpressions(document, argumentsList, diagnostics);
    const seenArgs = new Set();
    for (const argExpr of argumentsList) {
        let argNames;
        try {
            argNames = parseArgumentNames(argExpr);
        }
        catch (e) {
            const message = e instanceof Error ? e.message : String(e);
            const line = findArgumentLine(document, argExpr);
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: ${message}`, vscode.DiagnosticSeverity.Error));
            continue;
        }
        for (const name of argNames) {
            if (seenArgs.has(name)) {
                const line = findArgumentLine(document, `<${name}>`);
                diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: duplicate argument \`<${name}>\``, vscode.DiagnosticSeverity.Error));
            }
            seenArgs.add(name);
        }
    }
}
// ─── Synopsis parsing (subset of src/app/synopsis.rs) ────────────────────────
function parseOptionExpression(expr) {
    const trimmed = expr.trim();
    if (!trimmed)
        return { options: [], groups: [] };
    const [body, flagRepeats] = stripRepeatSuffix(trimmed);
    const parenInner = stripOuter(body, '(', ')');
    if (parenInner && body.startsWith('(') && isOptionParenGroup(parenInner)) {
        return parseParenOptionGroup(parenInner);
    }
    const bracketInner = stripOuter(body, '[', ']');
    if (bracketInner && body.startsWith('[')) {
        const chain = parseOptionChain(bracketInner, true);
        if (flagRepeats) {
            // flag-level repeat — names still register for duplicate/requires checks
        }
        return chain;
    }
    if (body.startsWith('-')) {
        return parseOptionChain(body, false);
    }
    throw new Error(`could not parse option expression "${expr}"`);
}
function parseParenOptionGroup(inner) {
    const tokens = tokenizeTop(inner);
    const alternatives = collectMutexAlternatives(tokens);
    if (alternatives.length === 0) {
        throw new Error('empty option group "(…)"');
    }
    if (alternatives.length === 1) {
        return { options: [parseSingleOption(alternatives[0], false)], groups: [] };
    }
    const options = [];
    const members = [];
    for (const alt of alternatives) {
        const opt = parseSingleOption(alt, false);
        const name = opt.long ?? opt.short;
        if (!name)
            throw new Error(`option in mutex group has no name`);
        members.push(name);
        options.push(opt);
    }
    return { options, groups: [{ members }] };
}
function parseOptionChain(inner, optional) {
    const tokens = tokenizeTop(inner);
    if (tokens.length === 0) {
        throw new Error('empty option group');
    }
    let split = tokens.length;
    for (let idx = 0; idx < tokens.length; idx++) {
        const t = tokens[idx];
        if (t.startsWith('[') && t.endsWith(']')) {
            split = idx;
            break;
        }
    }
    const primaryTokens = tokens.slice(0, split);
    const dependentTokens = tokens.slice(split);
    const primary = parseSingleOption(primaryTokens, optional);
    const results = [primary];
    const primaryName = primary.long ?? primary.short;
    for (const depTok of dependentTokens) {
        const innerDep = stripOuter(depTok, '[', ']');
        if (!innerDep) {
            throw new Error(`expected nested optional "[...]", got "${depTok}"`);
        }
        const dep = parseOptionChain(innerDep, true);
        for (const opt of dep.options) {
            if (primaryName) {
                opt.requires.push(primaryName);
            }
            results.push(opt);
        }
    }
    return { options: results, groups: [] };
}
function parseSingleOption(tokens, optional) {
    if (tokens.length === 0) {
        throw new Error('empty option');
    }
    let long;
    let short;
    for (const t of tokens) {
        if (t === '|' || t === '...')
            continue;
        if (t.startsWith('--')) {
            long = t.slice(2).split('=')[0];
            continue;
        }
        if (t.startsWith('-') && t.length === 2) {
            short = t[1];
            continue;
        }
        if (t.startsWith('<') || t.startsWith('{') || t.startsWith('('))
            continue;
        if (!optional && t.startsWith('--')) {
            long = t.slice(2);
        }
    }
    if (!long && !short) {
        throw new Error(`could not parse option from "${tokens.join(' ')}"`);
    }
    return { long, short, requires: [] };
}
function parseArgumentNames(expr) {
    const trimmed = expr.trim();
    if (!trimmed)
        return [];
    const tokens = tokenizeTop(trimmed);
    const names = [];
    for (const tok of tokens) {
        names.push(...extractPlaceholderNames(tok));
    }
    return names;
}
function extractPlaceholderNames(token) {
    let [body] = stripRepeatSuffix(token);
    const optionalInner = stripOuter(body, '[', ']');
    if (optionalInner) {
        body = optionalInner;
    }
    const requiredInner = stripOuter(body, '(', ')');
    if (requiredInner && body.startsWith('(')) {
        return ['choice'];
    }
    const angleInner = stripOuter(body, '<', '>');
    if (angleInner) {
        const nameMatch = angleInner.match(/^([a-zA-Z][a-zA-Z0-9_-]*)/);
        if (nameMatch) {
            return [nameMatch[1]];
        }
    }
    return [];
}
function tokenizeTop(input) {
    const out = [];
    let current = '';
    const chars = [...input];
    let i = 0;
    while (i < chars.length) {
        const c = chars[i];
        if (/\s/.test(c)) {
            if (current) {
                out.push(current);
                current = '';
            }
            i++;
            continue;
        }
        if ('[({<'.includes(c)) {
            const close = { '[': ']', '(': ')', '{': '}', '<': '>' }[c];
            const end = findMatching(chars, i, c, close);
            current += chars.slice(i, end + 1).join('');
            i = end + 1;
            continue;
        }
        if (c === '\'' || c === '"') {
            const end = findQuoteEnd(chars, i, c);
            current += chars.slice(i, end + 1).join('');
            i = end + 1;
            continue;
        }
        current += c;
        i++;
    }
    if (current)
        out.push(current);
    return out;
}
function findMatching(chars, start, open, close) {
    let depth = 0;
    let i = start;
    while (i < chars.length) {
        const c = chars[i];
        if (c === '\'' || c === '"') {
            i = findQuoteEnd(chars, i, c) + 1;
            continue;
        }
        if (c === open)
            depth++;
        else if (c === close) {
            depth--;
            if (depth === 0)
                return i;
        }
        i++;
    }
    throw new Error(`unbalanced "${open}" in expression`);
}
function findQuoteEnd(chars, start, quote) {
    let i = start + 1;
    while (i < chars.length) {
        if (chars[i] === '\\' && i + 1 < chars.length) {
            i += 2;
            continue;
        }
        if (chars[i] === quote)
            return i;
        i++;
    }
    throw new Error(`unclosed ${quote} in expression`);
}
function stripOuter(s, open, close) {
    if (s.length >= 2 && s.startsWith(open) && s.endsWith(close)) {
        return s.slice(1, -1);
    }
    return undefined;
}
// ─── Bracket balance checks ──────────────────────────────────────────────────
function lintOptionExpressions(document, options, diagnostics) {
    for (const opt of options) {
        lintBracketBalance(document, opt, findOptionLine(document, opt), diagnostics);
    }
}
function lintArgumentExpressions(document, argumentsList, diagnostics) {
    for (const arg of argumentsList) {
        lintBracketBalance(document, arg, findArgumentLine(document, arg), diagnostics);
    }
}
function lintBracketBalance(document, expr, line, diagnostics) {
    let depth = 0;
    for (const ch of expr) {
        if (ch === '[')
            depth++;
        if (ch === ']')
            depth--;
        if (depth < 0) {
            diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: expression has unmatched "]"`, vscode.DiagnosticSeverity.Error));
            return;
        }
    }
    if (depth !== 0) {
        diagnostics.push(new vscode.Diagnostic(lineRange(document, line), `${DIAG}: expression has unclosed "["`, vscode.DiagnosticSeverity.Error));
    }
}
// ─── Line finding ────────────────────────────────────────────────────────────
function findKeyLine(document, key) {
    const escaped = key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const re = new RegExp(`^"?${escaped}"?\\s*:`);
    for (let i = 0; i < document.lineCount; i++) {
        if (re.test(document.lineAt(i).text.trim()))
            return i;
    }
    return 0;
}
function findOptionLine(document, opt) {
    const needle = opt.trim().substring(0, Math.min(20, opt.length));
    for (let i = 0; i < document.lineCount; i++) {
        if (document.lineAt(i).text.includes(needle))
            return i;
    }
    return 0;
}
function findOptionByLong(document, long, _path) {
    return findOptionLine(document, `--${long}`);
}
function findOptionByShort(document, short, _path) {
    for (let i = 0; i < document.lineCount; i++) {
        const line = document.lineAt(i).text;
        if (new RegExp(`\\[-${short}(\\s|\\]|=)`).test(line) || new RegExp(`\\s-${short}(\\s|\\||\\]|=)`).test(line)) {
            return i;
        }
    }
    return 0;
}
function findArgumentLine(document, arg) {
    const needle = arg.trim().substring(0, Math.min(20, arg.length));
    for (let i = 0; i < document.lineCount; i++) {
        if (document.lineAt(i).text.includes(needle))
            return i;
    }
    return 0;
}
/** Best-effort line lookup for a `.name:` command key (ignores exact nesting depth, like the old local-file finder). */
function findCommandLine(document, name) {
    const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const re = new RegExp(`^\\s*\\.${escaped}\\s*:`);
    for (let i = 0; i < document.lineCount; i++) {
        if (re.test(document.lineAt(i).text))
            return i;
    }
    return 0;
}
function findNestedKeyLine(document, parentName, childKey) {
    const parentEscaped = parentName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const parentRe = new RegExp(`^(\\s*)\\.${parentEscaped}\\s*:`);
    let parentLine = -1;
    let parentIndent = 0;
    for (let i = 0; i < document.lineCount; i++) {
        const match = parentRe.exec(document.lineAt(i).text);
        if (match) {
            parentLine = i;
            parentIndent = match[1].length;
            break;
        }
    }
    if (parentLine < 0) {
        return findKeyLine(document, childKey);
    }
    const childEscaped = childKey.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const childRe = new RegExp(`^\\s*"?${childEscaped}"?\\s*:`);
    for (let i = parentLine + 1; i < document.lineCount; i++) {
        const lineText = document.lineAt(i).text;
        if (/\S/.test(lineText)) {
            const indent = lineText.length - lineText.trimStart().length;
            if (indent <= parentIndent)
                break;
        }
        if (childRe.test(lineText))
            return i;
    }
    return findKeyLine(document, childKey);
}
function lineRange(document, line) {
    const lineText = document.lineAt(line).text;
    return new vscode.Range(line, 0, line, lineText.length);
}
//# sourceMappingURL=extension.js.map