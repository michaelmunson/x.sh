import * as vscode from 'vscode';
import * as yaml from 'js-yaml';

// ─── Types ────────────────────────────────────────────────────────────────────

interface CommandDef {
  description?: string;
  options?: string[];
  arguments?: string | string[];
  commands?: Record<string, CommandDef>;
}

interface XAppDoc {
  name?: string;
  version?: string;
  description?: string;
  options?: string[];
  arguments?: string | string[];
  commands?: Record<string, CommandDef>;
  $?: Record<string, string>;
  '$.import'?: string | string[];
}

interface ParsedOption {
  long?: string;
  short?: string;
  requires: string[];
}

interface ParsedOptionGroup {
  members: string[];
}

function stripRepeatSuffix(s: string): [string, boolean] {
  const trimmed = s.trimEnd();
  if (trimmed.endsWith('...')) {
    return [trimmed.slice(0, -3).trimEnd(), true];
  }
  return [trimmed, false];
}

function isOptionParenGroup(inner: string): boolean {
  return inner
    .split('|')
    .map(p => p.trim())
    .filter(p => p.length > 0)
    .every(seg => seg.startsWith('-'));
}

function collectMutexAlternatives(tokens: string[]): string[][] {
  const alternatives: string[][] = [];
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
      if (i >= tokens.length || tokens[i] === '|') break;
      alt.push(tokens[i]);
      i++;
    }
    alternatives.push(alt);
  }
  return alternatives;
}

const DIAG = 'x.sh';

// ─── Extension Lifecycle ──────────────────────────────────────────────────────

let diagnosticCollection: vscode.DiagnosticCollection;

export function activate(context: vscode.ExtensionContext) {
  diagnosticCollection = vscode.languages.createDiagnosticCollection('xsh');
  context.subscriptions.push(diagnosticCollection);

  vscode.workspace.textDocuments.forEach(lintDocument);

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(lintDocument),
    vscode.workspace.onDidChangeTextDocument(e => lintDocument(e.document)),
    vscode.workspace.onDidCloseTextDocument(doc => diagnosticCollection.delete(doc.uri))
  );

  const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBar.text = '$(symbol-misc) x.sh';
  statusBar.tooltip = 'x.sh extension active';
  context.subscriptions.push(statusBar);

  vscode.window.onDidChangeActiveTextEditor(editor => {
    if (editor?.document.languageId === 'xsh') {
      statusBar.show();
    } else {
      statusBar.hide();
    }
  });

  if (vscode.window.activeTextEditor?.document.languageId === 'xsh') {
    statusBar.show();
  }

  console.log('x.sh extension activated');
}

export function deactivate() {
  diagnosticCollection?.dispose();
}

// ─── Linting ─────────────────────────────────────────────────────────────────

function lintDocument(document: vscode.TextDocument) {
  if (document.languageId !== 'xsh') return;

  const config = vscode.workspace.getConfiguration('xsh.lint');
  if (!config.get<boolean>('enabled', true)) {
    diagnosticCollection.delete(document.uri);
    return;
  }

  const diagnostics: vscode.Diagnostic[] = [];
  const text = document.getText();

  let parsed: XAppDoc;
  try {
    parsed = yaml.load(text) as XAppDoc;
  } catch (e: unknown) {
    const yamlError = e as { mark?: { line: number; column: number }; message: string };
    const line = yamlError.mark?.line ?? 0;
    const col = yamlError.mark?.column ?? 0;
    const range = new vscode.Range(line, col, line, col + 1);
    diagnostics.push(new vscode.Diagnostic(range, `${DIAG}: YAML parse error: ${yamlError.message}`, vscode.DiagnosticSeverity.Error));
    diagnosticCollection.set(document.uri, diagnostics);
    return;
  }

  if (!parsed || typeof parsed !== 'object') {
    diagnosticCollection.set(document.uri, diagnostics);
    return;
  }

  if (!parsed.name) {
    diagnostics.push(new vscode.Diagnostic(new vscode.Range(0, 0, 0, 0), `${DIAG}: missing required field "name"`, vscode.DiagnosticSeverity.Error));
  }

  if (parsed.name && !/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(parsed.name)) {
    const nameLine = findKeyLine(document, 'name');
    diagnostics.push(new vscode.Diagnostic(
      lineRange(document, nameLine),
      `${DIAG}: "name" must start with a letter and contain only letters, digits, hyphens, and underscores`,
      vscode.DiagnosticSeverity.Error
    ));
  }

  if (parsed.version && !/^\d+\.\d+\.\d+/.test(parsed.version)) {
    const verLine = findKeyLine(document, 'version');
    diagnostics.push(new vscode.Diagnostic(
      lineRange(document, verLine),
      `${DIAG}: "version" should use semantic versioning (e.g. 1.0.0)`,
      vscode.DiagnosticSeverity.Warning
    ));
  }

  if (parsed.$ && parsed['$.import']) {
    const line = findKeyLine(document, '$') || findKeyLine(document, '$.import');
    diagnostics.push(new vscode.Diagnostic(
      lineRange(document, line),
      `${DIAG}: cannot use both "$:" and "$.import:" in the same app file`,
      vscode.DiagnosticSeverity.Error
    ));
  }

  const allPaths = new Set<string>(['']);
  if (parsed.commands) {
    collectAllPaths(parsed.commands, '', allPaths);
  }

  const leafPaths = new Set<string>();
  if (!parsed.commands || Object.keys(parsed.commands).length === 0) {
    leafPaths.add('');
  } else {
    collectLeafPaths(parsed.commands, '', leafPaths);
  }

  const handlerKeys = new Set<string>(Object.keys(parsed.$ ?? {}));

  if (parsed.$) {
    const seen = new Set<string>();
    for (const key of Object.keys(parsed.$)) {
      if (seen.has(key)) {
        const handlerLine = findHandlerKeyLine(document, key);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, handlerLine),
          `${DIAG}: duplicate handler key in "$:" block: "${key}"`,
          vscode.DiagnosticSeverity.Error
        ));
      }
      seen.add(key);
    }
  }

  if (config.get<boolean>('warnOnUnreferencedHandlers', true)) {
    for (const key of handlerKeys) {
      if (!allPaths.has(key)) {
        const handlerLine = findHandlerKeyLine(document, key);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, handlerLine),
          `${DIAG}: handler key "${displayPath(key)}" does not match any command in "commands:"`,
          vscode.DiagnosticSeverity.Error
        ));
      }
    }
  }

  if (config.get<boolean>('warnOnMissingHandlers', true)) {
    for (const path of leafPaths) {
      if (!handlerKeys.has(path)) {
        const cmdLine = findCommandLine(document, path);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, cmdLine),
          `${DIAG}: leaf command has no handler — add "${displayPath(path)}: ..." under "$:"`,
          vscode.DiagnosticSeverity.Error
        ));
      }
    }
  }

  if (config.get<boolean>('requireDescription', false)) {
    lintDescriptions(document, parsed.commands ?? {}, '', diagnostics);
  }

  validateCommandScope(document, '', parsed.options ?? [], normalizeArguments(parsed.arguments), diagnostics);
  if (parsed.commands) {
    lintCommandScopes(document, parsed.commands, '', diagnostics);
  }

  lintHandlerValues(document, parsed.$ ?? {}, diagnostics);

  diagnosticCollection.set(document.uri, diagnostics);
}

function displayPath(path: string): string {
  return path === '' ? '$' : path;
}

// ─── Command path collection ─────────────────────────────────────────────────

function collectAllPaths(
  commands: Record<string, CommandDef>,
  prefix: string,
  out: Set<string>
) {
  for (const [name, def] of Object.entries(commands)) {
    const path = prefix ? `${prefix}.${name}` : name;
    out.add(path);
    if (def?.commands) {
      collectAllPaths(def.commands, path, out);
    }
  }
}

function collectLeafPaths(
  commands: Record<string, CommandDef>,
  prefix: string,
  out: Set<string>
) {
  for (const [name, def] of Object.entries(commands)) {
    const path = prefix ? `${prefix}.${name}` : name;
    if (def?.commands && Object.keys(def.commands).length > 0) {
      collectLeafPaths(def.commands, path, out);
    } else {
      out.add(path);
    }
  }
}

function normalizeArguments(args: string | string[] | undefined): string[] {
  if (!args) return [];
  return Array.isArray(args) ? args : [args];
}

// ─── Per-command validation (mirrors src/app/validate.rs) ────────────────────

function lintCommandScopes(
  document: vscode.TextDocument,
  commands: Record<string, CommandDef>,
  prefix: string,
  diagnostics: vscode.Diagnostic[]
) {
  for (const [name, def] of Object.entries(commands)) {
    const path = prefix ? `${prefix}.${name}` : name;
    validateCommandScope(document, path, def?.options ?? [], normalizeArguments(def?.arguments), diagnostics);
    if (def?.commands) {
      lintCommandScopes(document, def.commands, path, diagnostics);
    }
  }
}

function validateCommandScope(
  document: vscode.TextDocument,
  path: string,
  options: string[],
  argumentsList: string[],
  diagnostics: vscode.Diagnostic[]
) {
  lintOptionExpressions(document, options, diagnostics);

  const parsedOptions: ParsedOption[] = [];
  const parsedGroups: ParsedOptionGroup[] = [];
  for (const opt of options) {
    try {
      const parsed = parseOptionExpression(opt);
      parsedOptions.push(...parsed.options);
      parsedGroups.push(...parsed.groups);
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : String(e);
      const line = findOptionLine(document, opt);
      diagnostics.push(new vscode.Diagnostic(
        lineRange(document, line),
        `${DIAG}: ${message}`,
        vscode.DiagnosticSeverity.Error
      ));
    }
  }

  const seenLong = new Set<string>();
  const seenShort = new Set<string>();
  const definedLongs = new Set(parsedOptions.flatMap(o => (o.long ? [o.long] : [])));

  for (const opt of parsedOptions) {
    if (opt.long) {
      if (seenLong.has(opt.long)) {
        const line = findOptionByLong(document, opt.long, path);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: duplicate option \`--${opt.long}\``,
          vscode.DiagnosticSeverity.Error
        ));
      }
      seenLong.add(opt.long);
    }
    if (opt.short) {
      if (seenShort.has(opt.short)) {
        const line = findOptionByShort(document, opt.short, path);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: duplicate option \`-${opt.short}\``,
          vscode.DiagnosticSeverity.Error
        ));
      }
      seenShort.add(opt.short);
    }
  }

  for (const opt of parsedOptions) {
    for (const req of opt.requires) {
      if (!definedLongs.has(req)) {
        const line = findOptionLine(document, `--${req}`);
        const name = opt.long ? `--${opt.long}` : `-${opt.short}`;
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: option \`${name}\` requires \`--${req}\`, but \`--${req}\` is not defined on this command`,
          vscode.DiagnosticSeverity.Error
        ));
      }
    }
  }

  for (const group of parsedGroups) {
    for (const member of group.members) {
      if (!definedLongs.has(member)) {
        const line = findOptionLine(document, `--${member}`);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: option group references \`--${member}\`, but \`--${member}\` is not defined on this command`,
          vscode.DiagnosticSeverity.Error
        ));
      }
    }
  }

  lintArgumentExpressions(document, argumentsList, diagnostics);

  const seenArgs = new Set<string>();
  for (const argExpr of argumentsList) {
    let argNames: string[];
    try {
      argNames = parseArgumentNames(argExpr);
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : String(e);
      const line = findArgumentLine(document, argExpr);
      diagnostics.push(new vscode.Diagnostic(
        lineRange(document, line),
        `${DIAG}: ${message}`,
        vscode.DiagnosticSeverity.Error
      ));
      continue;
    }
    for (const name of argNames) {
      if (seenArgs.has(name)) {
        const line = findArgumentLine(document, `<${name}>`);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: duplicate argument \`<${name}>\``,
          vscode.DiagnosticSeverity.Error
        ));
      }
      seenArgs.add(name);
    }
  }
}

// ─── Synopsis parsing (subset of src/app/synopsis.rs) ────────────────────────

function parseOptionExpression(expr: string): { options: ParsedOption[]; groups: ParsedOptionGroup[] } {
  const trimmed = expr.trim();
  if (!trimmed) return { options: [], groups: [] };

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

function parseParenOptionGroup(inner: string): { options: ParsedOption[]; groups: ParsedOptionGroup[] } {
  const tokens = tokenizeTop(inner);
  const alternatives = collectMutexAlternatives(tokens);
  if (alternatives.length === 0) {
    throw new Error('empty option group "(…)"');
  }
  if (alternatives.length === 1) {
    return { options: [parseSingleOption(alternatives[0], false)], groups: [] };
  }
  const options: ParsedOption[] = [];
  const members: string[] = [];
  for (const alt of alternatives) {
    const opt = parseSingleOption(alt, false);
    const name = opt.long ?? opt.short;
    if (!name) throw new Error(`option in mutex group has no name`);
    members.push(name);
    options.push(opt);
  }
  return { options, groups: [{ members }] };
}

function parseOptionChain(inner: string, optional: boolean): { options: ParsedOption[]; groups: ParsedOptionGroup[] } {
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

  const results: ParsedOption[] = [primary];
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

function parseSingleOption(tokens: string[], optional: boolean): ParsedOption {
  if (tokens.length === 0) {
    throw new Error('empty option');
  }

  let long: string | undefined;
  let short: string | undefined;

  for (const t of tokens) {
    if (t === '|' || t === '...') continue;
    if (t.startsWith('--')) {
      long = t.slice(2).split('=')[0];
      continue;
    }
    if (t.startsWith('-') && t.length === 2) {
      short = t[1];
      continue;
    }
    if (t.startsWith('<') || t.startsWith('{') || t.startsWith('(')) continue;
    if (!optional && t.startsWith('--')) {
      long = t.slice(2);
    }
  }

  if (!long && !short) {
    throw new Error(`could not parse option from "${tokens.join(' ')}"`);
  }

  return { long, short, requires: [] };
}

function parseArgumentNames(expr: string): string[] {
  const trimmed = expr.trim();
  if (!trimmed) return [];

  const tokens = tokenizeTop(trimmed);
  const names: string[] = [];
  for (const tok of tokens) {
    names.push(...extractPlaceholderNames(tok));
  }
  return names;
}

function extractPlaceholderNames(token: string): string[] {
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

function tokenizeTop(input: string): string[] {
  const out: string[] = [];
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
      const close = { '[': ']', '(': ')', '{': '}', '<': '>' }[c] as string;
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
  if (current) out.push(current);
  return out;
}

function findMatching(chars: string[], start: number, open: string, close: string): number {
  let depth = 0;
  let i = start;
  while (i < chars.length) {
    const c = chars[i];
    if (c === '\'' || c === '"') {
      i = findQuoteEnd(chars, i, c) + 1;
      continue;
    }
    if (c === open) depth++;
    else if (c === close) {
      depth--;
      if (depth === 0) return i;
    }
    i++;
  }
  throw new Error(`unbalanced "${open}" in expression`);
}

function findQuoteEnd(chars: string[], start: number, quote: string): number {
  let i = start + 1;
  while (i < chars.length) {
    if (chars[i] === '\\' && i + 1 < chars.length) {
      i += 2;
      continue;
    }
    if (chars[i] === quote) return i;
    i++;
  }
  throw new Error(`unclosed ${quote} in expression`);
}

function stripOuter(s: string, open: string, close: string): string | undefined {
  if (s.length >= 2 && s.startsWith(open) && s.endsWith(close)) {
    return s.slice(1, -1);
  }
  return undefined;
}

// ─── Description linting ─────────────────────────────────────────────────────

function lintDescriptions(
  document: vscode.TextDocument,
  commands: Record<string, CommandDef>,
  prefix: string,
  diagnostics: vscode.Diagnostic[]
) {
  for (const [name, def] of Object.entries(commands)) {
    const path = prefix ? `${prefix}.${name}` : name;
    if (!def?.description) {
      const line = findCommandLine(document, path);
      diagnostics.push(new vscode.Diagnostic(
        lineRange(document, line),
        `${DIAG}: command "${path}" is missing a description`,
        vscode.DiagnosticSeverity.Warning
      ));
    }
    if (def?.commands) {
      lintDescriptions(document, def.commands, path, diagnostics);
    }
  }
}

// ─── Bracket balance checks ──────────────────────────────────────────────────

function lintOptionExpressions(
  document: vscode.TextDocument,
  options: string[],
  diagnostics: vscode.Diagnostic[]
) {
  for (const opt of options) {
    lintBracketBalance(document, opt, findOptionLine(document, opt), diagnostics);
  }
}

function lintArgumentExpressions(
  document: vscode.TextDocument,
  argumentsList: string[],
  diagnostics: vscode.Diagnostic[]
) {
  for (const arg of argumentsList) {
    lintBracketBalance(document, arg, findArgumentLine(document, arg), diagnostics);
  }
}

function lintBracketBalance(
  document: vscode.TextDocument,
  expr: string,
  line: number,
  diagnostics: vscode.Diagnostic[]
) {
  let depth = 0;
  for (const ch of expr) {
    if (ch === '[') depth++;
    if (ch === ']') depth--;
    if (depth < 0) {
      diagnostics.push(new vscode.Diagnostic(
        lineRange(document, line),
        `${DIAG}: expression has unmatched "]"`,
        vscode.DiagnosticSeverity.Error
      ));
      return;
    }
  }
  if (depth !== 0) {
    diagnostics.push(new vscode.Diagnostic(
      lineRange(document, line),
      `${DIAG}: expression has unclosed "["`,
      vscode.DiagnosticSeverity.Error
    ));
  }
}

// ─── Handler value linting ───────────────────────────────────────────────────

function lintHandlerValues(
  document: vscode.TextDocument,
  handlers: Record<string, string>,
  diagnostics: vscode.Diagnostic[]
) {
  for (const [key, value] of Object.entries(handlers)) {
    if (typeof value !== 'string') {
      const line = findHandlerKeyLine(document, key);
      diagnostics.push(new vscode.Diagnostic(
        lineRange(document, line),
        `${DIAG}: handler "${displayPath(key)}" value must be a string`,
        vscode.DiagnosticSeverity.Error
      ));
      continue;
    }
    const trimmed = value.trim();
    if (!trimmed.startsWith('x-') && !trimmed.includes('\n') && trimmed !== '') {
      if (/^[a-zA-Z][a-zA-Z0-9_-]+$/.test(trimmed)) {
        const line = findHandlerKeyLine(document, key);
        diagnostics.push(new vscode.Diagnostic(
          lineRange(document, line),
          `${DIAG}: handler "${displayPath(key)}" looks like a bare word — did you mean "x-${trimmed}" or a multiline bash script (use |)?`,
          vscode.DiagnosticSeverity.Hint
        ));
      }
    }
  }
}

// ─── Line finding ────────────────────────────────────────────────────────────

function findKeyLine(document: vscode.TextDocument, key: string): number {
  const escaped = key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const re = new RegExp(`^"?${escaped}"?\\s*:`);
  for (let i = 0; i < document.lineCount; i++) {
    if (re.test(document.lineAt(i).text.trim())) return i;
  }
  return 0;
}

function findOptionLine(document: vscode.TextDocument, opt: string): number {
  const needle = opt.trim().substring(0, Math.min(20, opt.length));
  for (let i = 0; i < document.lineCount; i++) {
    if (document.lineAt(i).text.includes(needle)) return i;
  }
  return 0;
}

function findOptionByLong(document: vscode.TextDocument, long: string, _path: string): number {
  return findOptionLine(document, `--${long}`);
}

function findOptionByShort(document: vscode.TextDocument, short: string, _path: string): number {
  for (let i = 0; i < document.lineCount; i++) {
    const line = document.lineAt(i).text;
    if (new RegExp(`\\[-${short}(\\s|\\]|=)`).test(line) || new RegExp(`\\s-${short}(\\s|\\||\\]|=)`).test(line)) {
      return i;
    }
  }
  return 0;
}

function findArgumentLine(document: vscode.TextDocument, arg: string): number {
  const needle = arg.trim().substring(0, Math.min(20, arg.length));
  for (let i = 0; i < document.lineCount; i++) {
    if (document.lineAt(i).text.includes(needle)) return i;
  }
  return 0;
}

function findCommandLine(document: vscode.TextDocument, path: string): number {
  if (path === '') return findKeyLine(document, 'name');
  const leaf = path.split('.').pop()!;
  const re = new RegExp(`^\\s+${leaf}\\s*:`);
  for (let i = 0; i < document.lineCount; i++) {
    if (re.test(document.lineAt(i).text)) return i;
  }
  return 0;
}

function findHandlerKeyLine(document: vscode.TextDocument, key: string): number {
  const escaped = key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const re = new RegExp(`^\\s+"?${escaped}"?\\s*:`);
  for (let i = 0; i < document.lineCount; i++) {
    if (re.test(document.lineAt(i).text)) return i;
  }
  return 0;
}

function lineRange(document: vscode.TextDocument, line: number): vscode.Range {
  const lineText = document.lineAt(line).text;
  return new vscode.Range(line, 0, line, lineText.length);
}
