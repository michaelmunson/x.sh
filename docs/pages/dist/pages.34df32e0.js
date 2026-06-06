// modules are defined as an array
// [ module function, map of requires ]
//
// map of requires is short require name -> numeric require
//
// anything defined in a previous bundle is accessed via the
// orig method which is the require for previous bundles

(function (
  modules,
  entry,
  mainEntry,
  parcelRequireName,
  externals,
  distDir,
  publicUrl,
  devServer
) {
  /* eslint-disable no-undef */
  var globalObject =
    typeof globalThis !== 'undefined'
      ? globalThis
      : typeof self !== 'undefined'
      ? self
      : typeof window !== 'undefined'
      ? window
      : typeof global !== 'undefined'
      ? global
      : {};
  /* eslint-enable no-undef */

  // Save the require from previous bundle to this closure if any
  var previousRequire =
    typeof globalObject[parcelRequireName] === 'function' &&
    globalObject[parcelRequireName];

  var importMap = previousRequire.i || {};
  var cache = previousRequire.cache || {};
  // Do not use `require` to prevent Webpack from trying to bundle this call
  var nodeRequire =
    typeof module !== 'undefined' &&
    typeof module.require === 'function' &&
    module.require.bind(module);

  function newRequire(name, jumped) {
    if (!cache[name]) {
      if (!modules[name]) {
        if (externals[name]) {
          return externals[name];
        }
        // if we cannot find the module within our internal map or
        // cache jump to the current global require ie. the last bundle
        // that was added to the page.
        var currentRequire =
          typeof globalObject[parcelRequireName] === 'function' &&
          globalObject[parcelRequireName];
        if (!jumped && currentRequire) {
          return currentRequire(name, true);
        }

        // If there are other bundles on this page the require from the
        // previous one is saved to 'previousRequire'. Repeat this as
        // many times as there are bundles until the module is found or
        // we exhaust the require chain.
        if (previousRequire) {
          return previousRequire(name, true);
        }

        // Try the node require function if it exists.
        if (nodeRequire && typeof name === 'string') {
          return nodeRequire(name);
        }

        var err = new Error("Cannot find module '" + name + "'");
        err.code = 'MODULE_NOT_FOUND';
        throw err;
      }

      localRequire.resolve = resolve;
      localRequire.cache = {};

      var module = (cache[name] = new newRequire.Module(name));

      modules[name][0].call(
        module.exports,
        localRequire,
        module,
        module.exports,
        globalObject
      );
    }

    return cache[name].exports;

    function localRequire(x) {
      var res = localRequire.resolve(x);
      if (res === false) {
        return {};
      }
      // Synthesize a module to follow re-exports.
      if (Array.isArray(res)) {
        var m = {__esModule: true};
        res.forEach(function (v) {
          var key = v[0];
          var id = v[1];
          var exp = v[2] || v[0];
          var x = newRequire(id);
          if (key === '*') {
            Object.keys(x).forEach(function (key) {
              if (
                key === 'default' ||
                key === '__esModule' ||
                Object.prototype.hasOwnProperty.call(m, key)
              ) {
                return;
              }

              Object.defineProperty(m, key, {
                enumerable: true,
                get: function () {
                  return x[key];
                },
              });
            });
          } else if (exp === '*') {
            Object.defineProperty(m, key, {
              enumerable: true,
              value: x,
            });
          } else {
            Object.defineProperty(m, key, {
              enumerable: true,
              get: function () {
                if (exp === 'default') {
                  return x.__esModule ? x.default : x;
                }
                return x[exp];
              },
            });
          }
        });
        return m;
      }
      return newRequire(res);
    }

    function resolve(x) {
      var id = modules[name][1][x];
      return id != null ? id : x;
    }
  }

  function Module(moduleName) {
    this.id = moduleName;
    this.bundle = newRequire;
    this.require = nodeRequire;
    this.exports = {};
  }

  newRequire.isParcelRequire = true;
  newRequire.Module = Module;
  newRequire.modules = modules;
  newRequire.cache = cache;
  newRequire.parent = previousRequire;
  newRequire.distDir = distDir;
  newRequire.publicUrl = publicUrl;
  newRequire.devServer = devServer;
  newRequire.i = importMap;
  newRequire.register = function (id, exports) {
    modules[id] = [
      function (require, module) {
        module.exports = exports;
      },
      {},
    ];
  };

  // Only insert newRequire.load when it is actually used.
  // The code in this file is linted against ES5, so dynamic import is not allowed.
  // INSERT_LOAD_HERE

  Object.defineProperty(newRequire, 'root', {
    get: function () {
      return globalObject[parcelRequireName];
    },
  });

  globalObject[parcelRequireName] = newRequire;

  for (var i = 0; i < entry.length; i++) {
    newRequire(entry[i]);
  }

  if (mainEntry) {
    // Expose entry point to Node, AMD or browser globals
    // Based on https://github.com/ForbesLindesay/umd/blob/master/template.js
    var mainExports = newRequire(mainEntry);

    // CommonJS
    if (typeof exports === 'object' && typeof module !== 'undefined') {
      module.exports = mainExports;

      // RequireJS
    } else if (typeof define === 'function' && define.amd) {
      define(function () {
        return mainExports;
      });
    }
  }
})({"3dtlh":[function(require,module,exports,__globalThis) {
var global = arguments[3];
var HMR_HOST = null;
var HMR_PORT = null;
var HMR_SERVER_PORT = 1234;
var HMR_SECURE = false;
var HMR_ENV_HASH = "439701173a9199ea";
var HMR_USE_SSE = false;
module.bundle.HMR_BUNDLE_ID = "4b8ea06834df32e0";
"use strict";
/* global HMR_HOST, HMR_PORT, HMR_SERVER_PORT, HMR_ENV_HASH, HMR_SECURE, HMR_USE_SSE, chrome, browser, __parcel__import__, __parcel__importScripts__, ServiceWorkerGlobalScope */ /*::
import type {
  HMRAsset,
  HMRMessage,
} from '@parcel/reporter-dev-server/src/HMRServer.js';
interface ParcelRequire {
  (string): mixed;
  cache: {|[string]: ParcelModule|};
  hotData: {|[string]: mixed|};
  Module: any;
  parent: ?ParcelRequire;
  isParcelRequire: true;
  modules: {|[string]: [Function, {|[string]: string|}]|};
  HMR_BUNDLE_ID: string;
  root: ParcelRequire;
}
interface ParcelModule {
  hot: {|
    data: mixed,
    accept(cb: (Function) => void): void,
    dispose(cb: (mixed) => void): void,
    // accept(deps: Array<string> | string, cb: (Function) => void): void,
    // decline(): void,
    _acceptCallbacks: Array<(Function) => void>,
    _disposeCallbacks: Array<(mixed) => void>,
  |};
}
interface ExtensionContext {
  runtime: {|
    reload(): void,
    getURL(url: string): string;
    getManifest(): {manifest_version: number, ...};
  |};
}
declare var module: {bundle: ParcelRequire, ...};
declare var HMR_HOST: string;
declare var HMR_PORT: string;
declare var HMR_SERVER_PORT: string;
declare var HMR_ENV_HASH: string;
declare var HMR_SECURE: boolean;
declare var HMR_USE_SSE: boolean;
declare var chrome: ExtensionContext;
declare var browser: ExtensionContext;
declare var __parcel__import__: (string) => Promise<void>;
declare var __parcel__importScripts__: (string) => Promise<void>;
declare var globalThis: typeof self;
declare var ServiceWorkerGlobalScope: Object;
*/ var OVERLAY_ID = '__parcel__error__overlay__';
var OldModule = module.bundle.Module;
function Module(moduleName) {
    OldModule.call(this, moduleName);
    this.hot = {
        data: module.bundle.hotData[moduleName],
        _acceptCallbacks: [],
        _disposeCallbacks: [],
        accept: function(fn) {
            this._acceptCallbacks.push(fn || function() {});
        },
        dispose: function(fn) {
            this._disposeCallbacks.push(fn);
        }
    };
    module.bundle.hotData[moduleName] = undefined;
}
module.bundle.Module = Module;
module.bundle.hotData = {};
var checkedAssets /*: {|[string]: boolean|} */ , disposedAssets /*: {|[string]: boolean|} */ , assetsToDispose /*: Array<[ParcelRequire, string]> */ , assetsToAccept /*: Array<[ParcelRequire, string]> */ , bundleNotFound = false;
function getHostname() {
    return HMR_HOST || (typeof location !== 'undefined' && location.protocol.indexOf('http') === 0 ? location.hostname : 'localhost');
}
function getPort() {
    return HMR_PORT || (typeof location !== 'undefined' ? location.port : HMR_SERVER_PORT);
}
// eslint-disable-next-line no-redeclare
let WebSocket = globalThis.WebSocket;
if (!WebSocket && typeof module.bundle.root === 'function') try {
    // eslint-disable-next-line no-global-assign
    WebSocket = module.bundle.root('ws');
} catch  {
// ignore.
}
var hostname = getHostname();
var port = getPort();
var protocol = HMR_SECURE || typeof location !== 'undefined' && location.protocol === 'https:' && ![
    'localhost',
    '127.0.0.1',
    '0.0.0.0'
].includes(hostname) ? 'wss' : 'ws';
// eslint-disable-next-line no-redeclare
var parent = module.bundle.parent;
if (!parent || !parent.isParcelRequire) {
    // Web extension context
    var extCtx = typeof browser === 'undefined' ? typeof chrome === 'undefined' ? null : chrome : browser;
    // Safari doesn't support sourceURL in error stacks.
    // eval may also be disabled via CSP, so do a quick check.
    var supportsSourceURL = false;
    try {
        (0, eval)('throw new Error("test"); //# sourceURL=test.js');
    } catch (err) {
        supportsSourceURL = err.stack.includes('test.js');
    }
    var ws;
    if (HMR_USE_SSE) ws = new EventSource('/__parcel_hmr');
    else try {
        // If we're running in the dev server's node runner, listen for messages on the parent port.
        let { workerData, parentPort } = module.bundle.root('node:worker_threads') /*: any*/ ;
        if (workerData !== null && workerData !== void 0 && workerData.__parcel) {
            parentPort.on('message', async (message)=>{
                try {
                    await handleMessage(message);
                    parentPort.postMessage('updated');
                } catch  {
                    parentPort.postMessage('restart');
                }
            });
            // After the bundle has finished running, notify the dev server that the HMR update is complete.
            queueMicrotask(()=>parentPort.postMessage('ready'));
        }
    } catch  {
        if (typeof WebSocket !== 'undefined') try {
            ws = new WebSocket(protocol + '://' + hostname + (port ? ':' + port : '') + '/');
        } catch (err) {
            // Ignore cloudflare workers error.
            if (err.message && !err.message.includes('Disallowed operation called within global scope')) console.error(err.message);
        }
    }
    if (ws) {
        // $FlowFixMe
        ws.onmessage = async function(event /*: {data: string, ...} */ ) {
            var data /*: HMRMessage */  = JSON.parse(event.data);
            await handleMessage(data);
        };
        if (ws instanceof WebSocket) {
            ws.onerror = function(e) {
                if (e.message) console.error(e.message);
            };
            ws.onclose = function() {
                console.warn("[parcel] \uD83D\uDEA8 Connection to the HMR server was lost");
            };
        }
    }
}
async function handleMessage(data /*: HMRMessage */ ) {
    checkedAssets = {} /*: {|[string]: boolean|} */ ;
    disposedAssets = {} /*: {|[string]: boolean|} */ ;
    assetsToAccept = [];
    assetsToDispose = [];
    bundleNotFound = false;
    if (data.type === 'reload') fullReload();
    else if (data.type === 'update') {
        // Remove error overlay if there is one
        if (typeof document !== 'undefined') removeErrorOverlay();
        let assets = data.assets;
        // Handle HMR Update
        let handled = assets.every((asset)=>{
            return asset.type === 'css' || asset.type === 'js' && hmrAcceptCheck(module.bundle.root, asset.id, asset.depsByBundle);
        });
        // Dispatch a custom event in case a bundle was not found. This might mean
        // an asset on the server changed and we should reload the page. This event
        // gives the client an opportunity to refresh without losing state
        // (e.g. via React Server Components). If e.preventDefault() is not called,
        // we will trigger a full page reload.
        if (handled && bundleNotFound && assets.some((a)=>a.envHash !== HMR_ENV_HASH) && typeof window !== 'undefined' && typeof CustomEvent !== 'undefined') handled = !window.dispatchEvent(new CustomEvent('parcelhmrreload', {
            cancelable: true
        }));
        if (handled) {
            console.clear();
            // Dispatch custom event so other runtimes (e.g React Refresh) are aware.
            if (typeof window !== 'undefined' && typeof CustomEvent !== 'undefined') window.dispatchEvent(new CustomEvent('parcelhmraccept'));
            await hmrApplyUpdates(assets);
            hmrDisposeQueue();
            // Run accept callbacks. This will also re-execute other disposed assets in topological order.
            let processedAssets = {};
            for(let i = 0; i < assetsToAccept.length; i++){
                let id = assetsToAccept[i][1];
                if (!processedAssets[id]) {
                    hmrAccept(assetsToAccept[i][0], id);
                    processedAssets[id] = true;
                }
            }
        } else fullReload();
    }
    if (data.type === 'error') {
        // Log parcel errors to console
        for (let ansiDiagnostic of data.diagnostics.ansi){
            let stack = ansiDiagnostic.codeframe ? ansiDiagnostic.codeframe : ansiDiagnostic.stack;
            console.error("\uD83D\uDEA8 [parcel]: " + ansiDiagnostic.message + '\n' + stack + '\n\n' + ansiDiagnostic.hints.join('\n'));
        }
        if (typeof document !== 'undefined') {
            // Render the fancy html overlay
            removeErrorOverlay();
            var overlay = createErrorOverlay(data.diagnostics.html);
            // $FlowFixMe
            document.body.appendChild(overlay);
        }
    }
}
function removeErrorOverlay() {
    var overlay = document.getElementById(OVERLAY_ID);
    if (overlay) {
        overlay.remove();
        console.log("[parcel] \u2728 Error resolved");
    }
}
function createErrorOverlay(diagnostics) {
    var overlay = document.createElement('div');
    overlay.id = OVERLAY_ID;
    let errorHTML = '<div style="background: black; opacity: 0.85; font-size: 16px; color: white; position: fixed; height: 100%; width: 100%; top: 0px; left: 0px; padding: 30px; font-family: Menlo, Consolas, monospace; z-index: 9999;">';
    for (let diagnostic of diagnostics){
        let stack = diagnostic.frames.length ? diagnostic.frames.reduce((p, frame)=>{
            return `${p}
<a href="${protocol === 'wss' ? 'https' : 'http'}://${hostname}:${port}/__parcel_launch_editor?file=${encodeURIComponent(frame.location)}" style="text-decoration: underline; color: #888" onclick="fetch(this.href); return false">${frame.location}</a>
${frame.code}`;
        }, '') : diagnostic.stack;
        errorHTML += `
      <div>
        <div style="font-size: 18px; font-weight: bold; margin-top: 20px;">
          \u{1F6A8} ${diagnostic.message}
        </div>
        <pre>${stack}</pre>
        <div>
          ${diagnostic.hints.map((hint)=>"<div>\uD83D\uDCA1 " + hint + '</div>').join('')}
        </div>
        ${diagnostic.documentation ? `<div>\u{1F4DD} <a style="color: violet" href="${diagnostic.documentation}" target="_blank">Learn more</a></div>` : ''}
      </div>
    `;
    }
    errorHTML += '</div>';
    overlay.innerHTML = errorHTML;
    return overlay;
}
function fullReload() {
    if (typeof location !== 'undefined' && 'reload' in location) location.reload();
    else if (typeof extCtx !== 'undefined' && extCtx && extCtx.runtime && extCtx.runtime.reload) extCtx.runtime.reload();
    else try {
        let { workerData, parentPort } = module.bundle.root('node:worker_threads') /*: any*/ ;
        if (workerData !== null && workerData !== void 0 && workerData.__parcel) parentPort.postMessage('restart');
    } catch (err) {
        console.error("[parcel] \u26A0\uFE0F An HMR update was not accepted. Please restart the process.");
    }
}
function getParents(bundle, id) /*: Array<[ParcelRequire, string]> */ {
    var modules = bundle.modules;
    if (!modules) return [];
    var parents = [];
    var k, d, dep;
    for(k in modules)for(d in modules[k][1]){
        dep = modules[k][1][d];
        if (dep === id || Array.isArray(dep) && dep[dep.length - 1] === id) parents.push([
            bundle,
            k
        ]);
    }
    if (bundle.parent) parents = parents.concat(getParents(bundle.parent, id));
    return parents;
}
function updateLink(link) {
    var href = link.getAttribute('href');
    if (!href) return;
    var newLink = link.cloneNode();
    newLink.onload = function() {
        if (link.parentNode !== null) // $FlowFixMe
        link.parentNode.removeChild(link);
    };
    newLink.setAttribute('href', // $FlowFixMe
    href.split('?')[0] + '?' + Date.now());
    // $FlowFixMe
    link.parentNode.insertBefore(newLink, link.nextSibling);
}
var cssTimeout = null;
function reloadCSS() {
    if (cssTimeout || typeof document === 'undefined') return;
    cssTimeout = setTimeout(function() {
        var links = document.querySelectorAll('link[rel="stylesheet"]');
        for(var i = 0; i < links.length; i++){
            // $FlowFixMe[incompatible-type]
            var href /*: string */  = links[i].getAttribute('href');
            var hostname = getHostname();
            var servedFromHMRServer = hostname === 'localhost' ? new RegExp('^(https?:\\/\\/(0.0.0.0|127.0.0.1)|localhost):' + getPort()).test(href) : href.indexOf(hostname + ':' + getPort());
            var absolute = /^https?:\/\//i.test(href) && href.indexOf(location.origin) !== 0 && !servedFromHMRServer;
            if (!absolute) updateLink(links[i]);
        }
        cssTimeout = null;
    }, 50);
}
function hmrDownload(asset) {
    if (asset.type === 'js') {
        if (typeof document !== 'undefined') {
            let script = document.createElement('script');
            script.src = asset.url + '?t=' + Date.now();
            if (asset.outputFormat === 'esmodule') script.type = 'module';
            return new Promise((resolve, reject)=>{
                var _document$head;
                script.onload = ()=>resolve(script);
                script.onerror = reject;
                (_document$head = document.head) === null || _document$head === void 0 || _document$head.appendChild(script);
            });
        } else if (typeof importScripts === 'function') {
            // Worker scripts
            if (asset.outputFormat === 'esmodule') return import(asset.url + '?t=' + Date.now());
            else return new Promise((resolve, reject)=>{
                try {
                    importScripts(asset.url + '?t=' + Date.now());
                    resolve();
                } catch (err) {
                    reject(err);
                }
            });
        }
    }
}
async function hmrApplyUpdates(assets) {
    global.parcelHotUpdate = Object.create(null);
    let scriptsToRemove;
    try {
        // If sourceURL comments aren't supported in eval, we need to load
        // the update from the dev server over HTTP so that stack traces
        // are correct in errors/logs. This is much slower than eval, so
        // we only do it if needed (currently just Safari).
        // https://bugs.webkit.org/show_bug.cgi?id=137297
        // This path is also taken if a CSP disallows eval.
        if (!supportsSourceURL) {
            let promises = assets.map((asset)=>{
                var _hmrDownload;
                return (_hmrDownload = hmrDownload(asset)) === null || _hmrDownload === void 0 ? void 0 : _hmrDownload.catch((err)=>{
                    // Web extension fix
                    if (extCtx && extCtx.runtime && extCtx.runtime.getManifest().manifest_version == 3 && typeof ServiceWorkerGlobalScope != 'undefined' && global instanceof ServiceWorkerGlobalScope) {
                        extCtx.runtime.reload();
                        return;
                    }
                    throw err;
                });
            });
            scriptsToRemove = await Promise.all(promises);
        }
        assets.forEach(function(asset) {
            hmrApply(module.bundle.root, asset);
        });
    } finally{
        delete global.parcelHotUpdate;
        if (scriptsToRemove) scriptsToRemove.forEach((script)=>{
            if (script) {
                var _document$head2;
                (_document$head2 = document.head) === null || _document$head2 === void 0 || _document$head2.removeChild(script);
            }
        });
    }
}
function hmrApply(bundle /*: ParcelRequire */ , asset /*:  HMRAsset */ ) {
    var modules = bundle.modules;
    if (!modules) return;
    if (asset.type === 'css') reloadCSS();
    else if (asset.type === 'js') {
        let deps = asset.depsByBundle[bundle.HMR_BUNDLE_ID];
        if (deps) {
            if (modules[asset.id]) {
                // Remove dependencies that are removed and will become orphaned.
                // This is necessary so that if the asset is added back again, the cache is gone, and we prevent a full page reload.
                let oldDeps = modules[asset.id][1];
                for(let dep in oldDeps)if (!deps[dep] || deps[dep] !== oldDeps[dep]) {
                    let id = oldDeps[dep];
                    let parents = getParents(module.bundle.root, id);
                    if (parents.length === 1) hmrDelete(module.bundle.root, id);
                }
            }
            if (supportsSourceURL) // Global eval. We would use `new Function` here but browser
            // support for source maps is better with eval.
            (0, eval)(asset.output);
            // $FlowFixMe
            let fn = global.parcelHotUpdate[asset.id];
            modules[asset.id] = [
                fn,
                deps
            ];
        }
        // Always traverse to the parent bundle, even if we already replaced the asset in this bundle.
        // This is required in case modules are duplicated. We need to ensure all instances have the updated code.
        if (bundle.parent) hmrApply(bundle.parent, asset);
    }
}
function hmrDelete(bundle, id) {
    let modules = bundle.modules;
    if (!modules) return;
    if (modules[id]) {
        // Collect dependencies that will become orphaned when this module is deleted.
        let deps = modules[id][1];
        let orphans = [];
        for(let dep in deps){
            let parents = getParents(module.bundle.root, deps[dep]);
            if (parents.length === 1) orphans.push(deps[dep]);
        }
        // Delete the module. This must be done before deleting dependencies in case of circular dependencies.
        delete modules[id];
        delete bundle.cache[id];
        // Now delete the orphans.
        orphans.forEach((id)=>{
            hmrDelete(module.bundle.root, id);
        });
    } else if (bundle.parent) hmrDelete(bundle.parent, id);
}
function hmrAcceptCheck(bundle /*: ParcelRequire */ , id /*: string */ , depsByBundle /*: ?{ [string]: { [string]: string } }*/ ) {
    checkedAssets = {};
    if (hmrAcceptCheckOne(bundle, id, depsByBundle)) return true;
    // Traverse parents breadth first. All possible ancestries must accept the HMR update, or we'll reload.
    let parents = getParents(module.bundle.root, id);
    let accepted = false;
    while(parents.length > 0){
        let v = parents.shift();
        let a = hmrAcceptCheckOne(v[0], v[1], null);
        if (a) // If this parent accepts, stop traversing upward, but still consider siblings.
        accepted = true;
        else if (a !== null) {
            // Otherwise, queue the parents in the next level upward.
            let p = getParents(module.bundle.root, v[1]);
            if (p.length === 0) {
                // If there are no parents, then we've reached an entry without accepting. Reload.
                accepted = false;
                break;
            }
            parents.push(...p);
        }
    }
    return accepted;
}
function hmrAcceptCheckOne(bundle /*: ParcelRequire */ , id /*: string */ , depsByBundle /*: ?{ [string]: { [string]: string } }*/ ) {
    var modules = bundle.modules;
    if (!modules) return;
    if (depsByBundle && !depsByBundle[bundle.HMR_BUNDLE_ID]) {
        // If we reached the root bundle without finding where the asset should go,
        // there's nothing to do. Mark as "accepted" so we don't reload the page.
        if (!bundle.parent) {
            bundleNotFound = true;
            return true;
        }
        return hmrAcceptCheckOne(bundle.parent, id, depsByBundle);
    }
    if (checkedAssets[id]) return null;
    checkedAssets[id] = true;
    var cached = bundle.cache[id];
    if (!cached) return true;
    assetsToDispose.push([
        bundle,
        id
    ]);
    if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
        assetsToAccept.push([
            bundle,
            id
        ]);
        return true;
    }
    return false;
}
function hmrDisposeQueue() {
    // Dispose all old assets.
    for(let i = 0; i < assetsToDispose.length; i++){
        let id = assetsToDispose[i][1];
        if (!disposedAssets[id]) {
            hmrDispose(assetsToDispose[i][0], id);
            disposedAssets[id] = true;
        }
    }
    assetsToDispose = [];
}
function hmrDispose(bundle /*: ParcelRequire */ , id /*: string */ ) {
    var cached = bundle.cache[id];
    bundle.hotData[id] = {};
    if (cached && cached.hot) cached.hot.data = bundle.hotData[id];
    if (cached && cached.hot && cached.hot._disposeCallbacks.length) cached.hot._disposeCallbacks.forEach(function(cb) {
        cb(bundle.hotData[id]);
    });
    delete bundle.cache[id];
}
function hmrAccept(bundle /*: ParcelRequire */ , id /*: string */ ) {
    // Execute the module.
    bundle(id);
    // Run the accept callbacks in the new version of the module.
    var cached = bundle.cache[id];
    if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
        let assetsToAlsoAccept = [];
        cached.hot._acceptCallbacks.forEach(function(cb) {
            let additionalAssets = cb(function() {
                return getParents(module.bundle.root, id);
            });
            if (Array.isArray(additionalAssets) && additionalAssets.length) assetsToAlsoAccept.push(...additionalAssets);
        });
        if (assetsToAlsoAccept.length) {
            let handled = assetsToAlsoAccept.every(function(a) {
                return hmrAcceptCheck(a[0], a[1]);
            });
            if (!handled) return fullReload();
            hmrDisposeQueue();
        }
    }
}

},{}],"gH3Lb":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
var _indexCss = require("./index.css");
var _app = require("./App");
var _appDefault = parcelHelpers.interopDefault(_app);
document.body.append((0, _appDefault.default)());

},{"./index.css":"irmnC","./App":"4Ye6C","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"irmnC":[function() {},{}],"4Ye6C":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "BASE_PATH", ()=>BASE_PATH);
parcelHelpers.export(exports, "default", ()=>App);
var _oxidizer = require("oxidizer");
var _oxidizerRouter = require("oxidizer-router");
var _oxidizerRouterDefault = parcelHelpers.interopDefault(_oxidizerRouter);
var _sidebar = require("./layout/Sidebar");
var _homePage = require("./views/HomePage");
var _homePageDefault = parcelHelpers.interopDefault(_homePage);
var _sectionPage = require("./views/SectionPage");
var _sectionPageDefault = parcelHelpers.interopDefault(_sectionPage);
var _notFoundPage = require("./views/NotFoundPage");
var _notFoundPageDefault = parcelHelpers.interopDefault(_notFoundPage);
const BASE_PATH = (()=>{
    const match = location.pathname.match(/^(\/x\.sh)(?:\/|$)/);
    return match ? match[1] : "";
})();
function App() {
    const shell = (0, _oxidizer.createProps)({
        path: (0, _oxidizerRouter.getPathname)(),
        open: false,
        routeKey: window.location.search
    }, [
        (0, _oxidizer.createEffect)([
            'path'
        ], ($)=>{
            console.log('path', $.path);
            console.log('path starts with base path', $.path.startsWith(BASE_PATH));
            if (!$.path.startsWith(BASE_PATH)) (0, _oxidizerRouter.navigate)(BASE_PATH + $.path);
        })
    ]);
    window.addEventListener("popstate", ()=>(0, _sidebar.syncRoute)(shell));
    return (0, _oxidizer.DIV)({
        id: "app"
    }, (0, _oxidizer.DIV)(shell, (p)=>[
            (0, _oxidizer.DIV)({
                className: `sidebar-overlay${p.open ? " visible" : ""}`,
                onclick: ()=>{
                    p.open = false;
                }
            }),
            (0, _sidebar.Sidebar)(shell)
        ]), (0, _oxidizer.DIV)({
        className: "main-wrap"
    }, (0, _oxidizer.DIV)({
        className: "topbar",
        role: "banner"
    }, (0, _oxidizer.BUTTON)({
        className: "menu-btn",
        type: "button",
        "aria-label": "Open menu",
        onclick: ()=>{
            shell.open = true;
        }
    }, "\u2630"), (0, _oxidizer.SPAN)({
        className: "topbar-title"
    }, "x.sh docs")), (0, _oxidizerRouterDefault.default)({
        "/": ()=>(0, _homePageDefault.default)(shell),
        "/docs": {
            ":sectionId": ()=>(0, _sectionPageDefault.default)(shell)
        },
        "/x.sh": {
            "index": ()=>(0, _homePageDefault.default)(shell),
            "/docs": {
                ":sectionId": ()=>(0, _sectionPageDefault.default)(shell)
            }
        },
        "*": ()=>(0, _notFoundPageDefault.default)(shell)
    }), (0, _oxidizer.DIV)({
        className: "page-footer"
    }, (0, _oxidizer.P)({}, "Built with ", (0, _oxidizer.A)({
        href: "https://www.npmjs.com/package/oxidizer",
        className: "inline-link",
        target: "_blank",
        rel: "noopener noreferrer"
    }, "oxidizer"), " & ", (0, _oxidizer.A)({
        href: "https://www.npmjs.com/package/oxidizer-router",
        className: "inline-link",
        target: "_blank",
        rel: "noopener noreferrer"
    }, "oxidizer-router"), " \xb7 ", (0, _oxidizer.A)({
        href: "https://github.com/michaelmunson/x.sh",
        className: "inline-link",
        target: "_blank",
        rel: "noopener noreferrer"
    }, "x.sh on GitHub")))));
}

},{"oxidizer":"hnuU9","oxidizer-router":"6kk03","./layout/Sidebar":"hNbmf","./views/HomePage":"e9AAl","./views/SectionPage":"3AN0D","./views/NotFoundPage":"b6FlI","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"hnuU9":[function(require,module,exports,__globalThis) {
"use strict";
var __createBinding = this && this.__createBinding || (Object.create ? function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) desc = {
        enumerable: true,
        get: function() {
            return m[k];
        }
    };
    Object.defineProperty(o, k2, desc);
} : function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
});
var __exportStar = this && this.__exportStar || function(m, exports1) {
    for(var p in m)if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports1, p)) __createBinding(exports1, m, p);
};
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.ox = exports.Configuration = exports.Component = exports.createShadowComponent = exports.createComponentExtension = exports.createComponent = exports.html = exports.css = exports.createEffect = exports.createProps = void 0;
var props_1 = require("37539d85963ce572");
Object.defineProperty(exports, "createProps", {
    enumerable: true,
    get: function() {
        return props_1.createProps;
    }
});
Object.defineProperty(exports, "createEffect", {
    enumerable: true,
    get: function() {
        return props_1.createEffect;
    }
});
var utils_1 = require("93816504fd2d5b92");
Object.defineProperty(exports, "css", {
    enumerable: true,
    get: function() {
        return utils_1.css;
    }
});
Object.defineProperty(exports, "html", {
    enumerable: true,
    get: function() {
        return utils_1.html;
    }
});
var components_1 = require("2bc990817a4daacf");
Object.defineProperty(exports, "createComponent", {
    enumerable: true,
    get: function() {
        return components_1.createComponent;
    }
});
Object.defineProperty(exports, "createComponentExtension", {
    enumerable: true,
    get: function() {
        return components_1.createComponentExtension;
    }
});
Object.defineProperty(exports, "createShadowComponent", {
    enumerable: true,
    get: function() {
        return components_1.createShadowComponent;
    }
});
Object.defineProperty(exports, "Component", {
    enumerable: true,
    get: function() {
        return components_1.Component;
    }
});
var config_1 = require("4c05f08f626eecf1");
Object.defineProperty(exports, "Configuration", {
    enumerable: true,
    get: function() {
        return config_1.Configuration;
    }
});
__exportStar(require("9d75a45195803233"), exports);
var utils_2 = require("9c2addbba5e78953");
Object.defineProperty(exports, "ox", {
    enumerable: true,
    get: function() {
        return utils_2.ox;
    }
});
__exportStar(require("8e5fd0539dce2685"), exports);

},{"37539d85963ce572":"24Dw0","93816504fd2d5b92":"9yM9v","2bc990817a4daacf":"gDb1y","4c05f08f626eecf1":"7rQgM","9d75a45195803233":"7krqs","9c2addbba5e78953":"jlkVB","8e5fd0539dce2685":"fkPEc"}],"24Dw0":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.createEffect = void 0;
exports.createProps = createProps;
const renderMap_1 = require("76255f71da09bd90");
const utils_1 = require("67d70da7b10d342e");
var effects_1 = require("c1d416d8f0ca1e43");
Object.defineProperty(exports, "createEffect", {
    enumerable: true,
    get: function() {
        return effects_1.createEffect;
    }
});
function handleModifiers(props, key, modifiers) {
    if (!modifiers) return;
    for (const modifier of modifiers)if ((0, utils_1.isModifier)(modifier) && modifier.isRun(key)) {
        const returnObject = modifier.run(props);
        if (returnObject) for(const k in returnObject)props[k] = returnObject[k];
    }
}
/**
 * @description creates a props proxy that is used to update the state of a component
 * @example
 * ```typescript
const props = createProps({count: 0});
// update state by simply setting a property value
props.count = 1;
```
 */ function createProps(input, modifiers) {
    // CREATE PROPS
    const props = new Proxy(input, {
        set (target, property, newValue) {
            // set target value
            target[property] = newValue;
            // add to RenderMap
            const renderMap = renderMap_1.__PROPS_RENDER_MAP__.get(props);
            // handle modifiers
            handleModifiers(props, property, modifiers);
            // render attached elements
            if (renderMap) renderMap.renderEach(props);
            return true;
        },
        get (target, key) {
            if (key === utils_1.isProxySymbol) return true;
            else return target[key];
        }
    });
    // REASSIGN TO RUN ALL EFFECTS
    Object.assign(props, input);
    // ADD TO RENDER MAP
    renderMap_1.__PROPS_RENDER_MAP__.set(props, new renderMap_1.RenderMap());
    return props;
}

},{"76255f71da09bd90":"kYBh0","67d70da7b10d342e":"7JVjX","c1d416d8f0ca1e43":"5yZDQ"}],"kYBh0":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.__PROPS_RENDER_MAP__ = exports.RenderMap = void 0;
const utils_1 = require("ff37f281805fb1e6");
class RenderMap extends Map {
    constructor(){
        super();
    }
    renderEach(props) {
        for (const [element, renderFn] of this){
            const elementProperties = renderFn.call(element, props);
            (0, utils_1.setElementProperties)(element, ...elementProperties);
        }
    }
}
exports.RenderMap = RenderMap;
exports.__PROPS_RENDER_MAP__ = new Map();

},{"ff37f281805fb1e6":"jlkVB"}],"jlkVB":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.setElementAttributes = setElementAttributes;
exports.setElementChildren = setElementChildren;
exports.setElementProperties = setElementProperties;
exports.createElement = createElement;
exports.createIntrinsicElement = createIntrinsicElement;
exports.createFragment = createFragment;
exports.createIntrinsicElementComponent = createIntrinsicElementComponent;
exports.createShadowElement = createShadowElement;
exports.createElementFactory = createElementFactory;
exports.ox = ox;
const _helpers_1 = require("8c48f51a93868967");
const types_1 = require("4ca20562447d3ef4");
const utils_1 = require("df4b87fbf3829d57");
const renderMap_1 = require("cfdc0feee95a4647");
const config_1 = require("654fbc3102858824");
const utils_2 = require("d81f9e71a17c5834");
class RenderError extends TypeError {
    constructor(message){
        super(message);
        this.name = "RenderError";
    }
}
/** GENERAL EXPORTED UTILITIES */ /****** HTML ELEMENTS */ function setElementAttributes(element, attrs) {
    let attr;
    for(attr in attrs)try {
        let attrValue = attrs[attr];
        if (!(attr in element) && typeof attrValue === "string") element.setAttribute(attr, attrValue);
        else if (attr === "style" && typeof attrValue === "object") {
            const style = Object.entries(attrValue).map(([key, val])=>`${(0, _helpers_1.camelToDashed)(key)}:${val};`).join("\n  ");
            element.style = style;
        } else element[attr] = attrValue;
    } catch (e) {
        console.warn(e);
    }
    return element;
}
function setElementChildren(element, ...children) {
    for (const _children of children){
        if ((0, types_1.isHTMLChildren)(_children)) for (const child of [
            ..._children
        ])setElementChildren(element, child);
        else if ((0, types_1.isHTMLPrimitive)(_children)) {
            const htmlString = typeof _children === "boolean" || !_children ? "" : _children.toString();
            const node = (0, utils_2.html)`${htmlString}`;
            element.appendChild(node);
        } else if ((0, types_1.isDOMNode)(_children) || (0, types_1.isDocumentFragment)(_children)) element.appendChild(_children);
        else throw new RenderError('child not of type HTMLChildren');
    }
    return element;
}
function setElementProperties(element, ...params) {
    const [arg0, ...arg1] = params;
    if ((0, types_1.isAttributes)(arg0)) {
        setElementAttributes(element, arg0);
        if ((0, types_1.isHTMLChildren)(arg1)) {
            element.innerHTML = "";
            setElementChildren(element, ...arg1);
        } else if ((0, types_1.isHTMLChild)(arg1)) {
            element.innerHTML = "";
            setElementChildren(element, arg1);
        }
    } else if (params.length >= 0 && (0, types_1.isHTMLChildren)(params)) {
        element.innerHTML = "";
        setElementChildren(element, ...params);
    } else if ((0, types_1.isHTMLChildren)(arg0)) {
        element.innerHTML = "";
        setElementChildren(element, ...arg0);
    } else if ((0, types_1.isHTMLChild)(arg0)) {
        element.innerHTML = "";
        setElementChildren(element, arg0);
    }
    return element;
}
function createElement(tagName, customElementTagName) {
    const element = customElementTagName ? document.createElement(tagName, {
        is: customElementTagName
    }) : document.createElement(tagName);
    if (config_1.Configuration.get().components.autoUpgrade) {
        if (customElements.get(tagName)) customElements.upgrade(element);
    }
    return element;
}
function createIntrinsicElement(tagName, ...params) {
    const element = createElement(tagName);
    ox(element)(...params);
    return element;
}
/****** DOCUMENT FRAGMENTS */ function createFragment(...children) {
    const fragment = document.createDocumentFragment();
    setElementChildren(fragment, ...children);
    return fragment;
}
/** CUSTOM ELEMENTS */ function createIntrinsicElementComponent(tagName, customElementTagName, ...params) {
    const element = createElement(tagName, customElementTagName);
    if ((0, utils_1.isProps)(params[0]) && typeof params[1] === "function") {
        const [props, renderFn] = params;
        renderMap_1.__PROPS_RENDER_MAP__.get(props)?.set(element, renderFn);
        const elementProperties = renderFn.call(element, props);
        setElementProperties(element, ...elementProperties);
    } else setElementProperties(element, ...params);
    return element;
}
function createShadowElement(tagName, options, ...params) {
    const element = createElement(tagName);
    const shadow = element.attachShadow(options);
    if ((0, utils_1.isProps)(params[0]) && typeof params[1] === "function") {
        const [props, renderFn] = params;
        renderMap_1.__PROPS_RENDER_MAP__.get(props)?.set(shadow, renderFn);
        const elementProperties = renderFn.call(shadow, props);
        setElementProperties(shadow, ...elementProperties);
    } else setElementProperties(shadow, ...params);
    return element;
}
function createElementFactory(tagName) {
    return (...params)=>createIntrinsicElement(tagName, ...params);
}
/** LIB EXPORTS */ function ox(elem) {
    const element = typeof elem === "string" ? (0, _helpers_1.strictQuery)(elem) : elem;
    return (...params)=>{
        if ((0, utils_1.isProps)(params[0]) && typeof params[1] === "function") {
            const [props, renderFn] = params;
            renderMap_1.__PROPS_RENDER_MAP__.get(props)?.set(element, renderFn);
            const elementProperties = renderFn.call(element, props);
            setElementProperties(element, ...elementProperties);
        } else setElementProperties(element, ...params);
        return element;
    };
}

},{"8c48f51a93868967":"dCGQm","4ca20562447d3ef4":"9kytZ","df4b87fbf3829d57":"7JVjX","cfdc0feee95a4647":"kYBh0","654fbc3102858824":"7rQgM","d81f9e71a17c5834":"9yM9v"}],"dCGQm":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.camelToDashed = camelToDashed;
exports.dashedToCamel = dashedToCamel;
exports.strictQuery = strictQuery;
function camelToDashed(input) {
    return input.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase();
}
function dashedToCamel(input) {
    return input.replace(/-([a-z])/g, (match, group)=>group.toUpperCase());
}
function strictQuery(selector) {
    if (typeof selector !== 'string') throw new TypeError('Selector must be of type string');
    const element = document.querySelector(selector);
    if (!element) throw new ReferenceError(`No element found matching selector "${selector}"`);
    return element;
}

},{}],"9kytZ":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.isAttributes = isAttributes;
exports.isHTMLPrimitive = isHTMLPrimitive;
exports.isHTMLElement = isHTMLElement;
exports.isDocumentFragment = isDocumentFragment;
exports.isHTMLNode = isHTMLNode;
exports.isDOMNode = isDOMNode;
exports.isHTMLChild = isHTMLChild;
exports.isHTMLChildArray = isHTMLChildArray;
exports.isHTMLChildren = isHTMLChildren;
const utils_1 = require("2c9f74bb17a231e3");
function isAttributes(params) {
    return !!params && !isHTMLChildren(params) && !isHTMLChild(params) && !(0, utils_1.isProps)(params);
}
function isHTMLPrimitive(child) {
    return typeof child === "string" || typeof child === "undefined" || typeof child === "number" || typeof child === "boolean" || child === null;
}
function isHTMLElement(element) {
    return element instanceof HTMLElement;
}
function isDocumentFragment(fragment) {
    return fragment instanceof DocumentFragment;
}
function isHTMLNode(node) {
    return isHTMLElement(node) || isDocumentFragment(node);
}
function isDOMNode(node) {
    return isHTMLElement(node) || node instanceof Text || node instanceof Element;
}
function isHTMLChild(params) {
    return isHTMLPrimitive(params) || isDOMNode(params) || isHTMLChildren(params) || isDocumentFragment(params);
}
function isHTMLChildArray(params) {
    return Array.isArray(params) && params.reduce((acc, curr)=>acc && isHTMLChild(curr), true);
}
function isHTMLChildren(params) {
    return isHTMLChildArray(params) || params instanceof NodeList || params instanceof HTMLCollection;
}

},{"2c9f74bb17a231e3":"7JVjX"}],"7JVjX":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.isProps = exports.isProxySymbol = exports.isModifier = void 0;
const effects_1 = require("35ea7e27fcd62929");
const isModifier = (anything)=>{
    return anything instanceof effects_1.Effect;
};
exports.isModifier = isModifier;
exports.isProxySymbol = Symbol("isProxy");
const isProps = (props)=>{
    return props && props[exports.isProxySymbol];
};
exports.isProps = isProps;

},{"35ea7e27fcd62929":"5yZDQ"}],"5yZDQ":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.Effect = void 0;
exports.createEffect = createEffect;
class Effect {
    callback;
    dependencies;
    constructor(dependencies, callback){
        this.callback = callback;
        this.dependencies = !Array.isArray(dependencies) ? [
            dependencies
        ] : dependencies;
    }
    isRun(key) {
        return this.dependencies.includes(key);
    }
    run(props) {
        return this.callback.call({}, props);
    }
}
exports.Effect = Effect;
/**
 * @description
 * * creates an "effect", or a function that will be ran when a props property has been changed.
 * * effect callbacks will be ran immediately upon creation
 * * the return value of the callback will be applied to the props
 * @example
 * ```typescript
    const props = createProps({count: 0}, [
        // keep count >= 0 and <= 100
        createEffect(['count'], $ => {
            if ($.count < 0){
                $.count = 0;
            }
            if ($.count > 100) return {
                count: 100
            }
        })
    ]);
    // props will be set to 0
    props.count = -10;
    // props will be set to 100
    props.count = 110;
```
 */ function createEffect(...params) {
    return new Effect(...params);
}

},{}],"7rQgM":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.Configuration = void 0;
const CONFIG = {
    components: {
        autoUpgrade: false,
        shadowInit: {
            mode: 'open'
        }
    }
};
var Configuration;
(function(Configuration) {
    Configuration.configure = (config)=>{
        let key;
        for(key in config)Object.assign(CONFIG, config[key]);
        return CONFIG;
    };
    Configuration.get = ()=>CONFIG;
})(Configuration || (exports.Configuration = Configuration = {}));

},{}],"9yM9v":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.html = exports.css = void 0;
var css_1 = require("41fdc41949450d57");
Object.defineProperty(exports, "css", {
    enumerable: true,
    get: function() {
        return css_1.css;
    }
});
var html_1 = require("2f07ea2f0a04811b");
Object.defineProperty(exports, "html", {
    enumerable: true,
    get: function() {
        return html_1.html;
    }
});

},{"41fdc41949450d57":"fLo2c","2f07ea2f0a04811b":"2RU4m"}],"fLo2c":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.css = css;
function css(strs, ...values) {
    return strs.map((s, i)=>s + (values[i] ?? "")).join('');
}
(function(css) {
    // export type Properties = CSSProperties;
    function assign(...parameters) {
        const [assignee, styles] = parameters;
        if (assignee instanceof HTMLElement) {
            if (typeof styles === "string") assignee.style = styles;
            else Object.assign(assignee.style, styles);
        } else if (assignee instanceof CSSStyleDeclaration) {
            if (typeof styles === "string") throw new TypeError('assigned styles must be intersection of CSSStyleDeclaration');
            Object.assign(assignee, styles);
        } else throw new TypeError('Invalid assignee parameter. Assignee must be of type "HTMLElement" or "CSSStyleDeclaration"');
    }
    css.assign = assign;
})(css || (exports.css = css = {}));

},{}],"2RU4m":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.html = html;
class HTMLParsingError extends Error {
    constructor(message){
        super(message.join('\n'));
        this.name = 'HTMLParsingError';
    }
}
const parseHTMLString = (htmlString)=>{
    const doc = new DOMParser().parseFromString(htmlString, 'text/html');
    const errorNode = doc.querySelector("parsererror");
    if (errorNode) throw new HTMLParsingError([
        ...errorNode.children
    ].map((child)=>child.textContent ?? ""));
    const nodes = [
        ...doc.body.childNodes
    ];
    if (nodes.length === 1) return nodes[0];
    else {
        const fragment = document.createDocumentFragment();
        for (const node of nodes)fragment.appendChild(node);
        return fragment;
    }
};
function html(strs, ...values) {
    const htmlString = strs.map((s, i)=>s + (values[i] ?? "")).join('');
    return parseHTMLString(htmlString);
}

},{}],"gDb1y":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.Component = void 0;
exports.createComponent = createComponent;
exports.createComponentExtension = createComponentExtension;
exports.createShadowComponent = createShadowComponent;
const config_1 = require("affc35982935b57b");
const utils_1 = require("bb1bfd74942a925f");
class Component extends HTMLElement {
    /**
     * @description Attributes whose change in value triggers `attributeChangedCallback`
     * @see  */ static observedAttributes;
}
exports.Component = Component;
/**
 * @description A factory class used for creating a rendering function for you custom elements.
 * @example
    const TestApp = createComponent(
        'test-app',
        class extends HTMLElement implements Component {
            connectedCallback(): void {
                this.style.color = "red";
                this.style.background = "blue";
            }
        }
    );

    document.body.append(
        TestApp(
            {id: "root"},
            H1('Hello!'),
            P('How do you like my app?')
        )
    )
 */ function createComponent(tagName, classDefinition) {
    customElements.define(tagName, classDefinition);
    return (...params)=>(0, utils_1.createIntrinsicElement)(tagName, ...params);
}
function createComponentExtension(tagName, extension, classDefinition) {
    customElements.define(tagName, classDefinition, {
        extends: extension
    });
    return (...params)=>(0, utils_1.createIntrinsicElementComponent)(extension, tagName, ...params);
}
function createShadowComponent(tagName, classDefinition, options) {
    const shadowInit = options ?? config_1.Configuration.get().components.shadowInit;
    customElements.define(tagName, classDefinition);
    return (...params)=>(0, utils_1.createShadowElement)(tagName, shadowInit, ...params);
}

},{"affc35982935b57b":"7rQgM","bb1bfd74942a925f":"jlkVB"}],"7krqs":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.LEGEND = exports.LABEL = exports.LI = exports.INPUT = exports.IMG = exports.IFRAME = exports.I = exports.INS = exports.KBD = exports.HTML = exports.H6 = exports.H5 = exports.H4 = exports.H3 = exports.H2 = exports.H1 = exports.HEAD = exports.HR = exports.HGROUP = exports.FORM = exports.FOOTER = exports.FIGURE = exports.FIELDSET = exports.EMBED = exports.EM = exports.DIV = exports.DIALOG = exports.DETAILS = exports.DATALIST = exports.DATA = exports.DL = exports.DD = exports.COLGROUP = exports.CODE = exports.CITE = exports.CANVAS = exports.BUTTON = exports.BODY = exports.BASE = exports.BDO = exports.BDI = exports.B = exports.BR = exports.ASIDE = exports.ARTICLE = exports.ADDRESS = exports.ABBR = exports.AUDIO = exports.AREA = exports.A = void 0;
exports.U = exports.TRACK = exports.TITLE = exports.TIME = exports.TEXTAREA = exports.TEMPLATE = exports.TBODY = exports.TFOOT = exports.THEAD = exports.TR = exports.TABLE = exports.COL = exports.TD = exports.TH = exports.CAPTION = exports.SECTION = exports.SEARCH = exports.SUMMARY = exports.SUB = exports.STYLE = exports.SPAN = exports.SOURCE = exports.SLOT = exports.SELECT = exports.SCRIPT = exports.BLOCKQUOTE = exports.Q = exports.SMALL = exports.RUBY = exports.RP = exports.RT = exports.PROGRESS = exports.PRE = exports.PICTURE = exports.P = exports.OUTPUT = exports.OPTION = exports.OPTGROUP = exports.OBJECT = exports.OL = exports.DEL = exports.METER = exports.META = exports.NOSCRIPT = exports.NAV = exports.MARK = exports.MENU = exports.MAP = exports.MAIN = exports.LINK = void 0;
exports.FRAGMENT = exports.WBR = exports.VAR = exports.VIDEO = exports.UL = void 0;
const utils_1 = require("ce6b4a056d7b02cf");
/**
 * #### HTMLAnchorElement
 * ```html
 * <a>
 * ```
 * @description creates an `HTMLAnchorElement` instance.
 * @example
 * ```typescript
 * const a:HTMLAnchorElement =
 *     A(
 *         {className: 'a'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.A = (0, utils_1.createElementFactory)("a");
/**
 * #### HTMLAreaElement
 * ```html
 * <area>
 * ```
 * @description creates an `HTMLAreaElement` instance.
 * @example ```typescript
 * const areaElement = AREA({className: 'area'});
 * ```
 */ exports.AREA = (0, utils_1.createElementFactory)("area");
/**
 * #### HTMLAudioElement
 * ```html
 * <audio>
 * ```
 * @description creates an `HTMLAudioElement` instance.
 * @example
 * ```typescript
 * const audio:HTMLAudioElement =
 *     AUDIO(
 *         {className: 'audio'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.AUDIO = (0, utils_1.createElementFactory)("audio");
/**
 * #### HTMLElement
 * ```html
 * <abbr>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const abbr:HTMLElement =
 *     ABBR(
 *         {className: 'abbr'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.ABBR = (0, utils_1.createElementFactory)("abbr");
/**
 * #### HTMLElement
 * ```html
 * <address>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const address:HTMLElement =
 *     ADDRESS(
 *         {className: 'address'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.ADDRESS = (0, utils_1.createElementFactory)("address");
/**
 * #### HTMLElement
 * ```html
 * <article>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const article:HTMLElement =
 *     ARTICLE(
 *         {className: 'article'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.ARTICLE = (0, utils_1.createElementFactory)("article");
/**
 * #### HTMLElement
 * ```html
 * <aside>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const aside:HTMLElement =
 *     ASIDE(
 *         {className: 'aside'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.ASIDE = (0, utils_1.createElementFactory)("aside");
/**
 * #### HTMLBRElement
 * ```html
 * <br>
 * ```
 * @description creates an `HTMLBRElement` instance.
 * @example ```typescript
 * const brElement = BR({className: 'br'});
 * ```
 */ exports.BR = (0, utils_1.createElementFactory)("br");
/**
 * #### HTMLElement
 * ```html
 * <b>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const b:HTMLElement =
 *     B(
 *         {className: 'b'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.B = (0, utils_1.createElementFactory)("b");
/**
 * #### HTMLElement
 * ```html
 * <bdi>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const bdi:HTMLElement =
 *     BDI(
 *         {className: 'bdi'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.BDI = (0, utils_1.createElementFactory)("bdi");
/**
 * #### HTMLElement
 * ```html
 * <bdo>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const bdo:HTMLElement =
 *     BDO(
 *         {className: 'bdo'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.BDO = (0, utils_1.createElementFactory)("bdo");
/**
 * #### HTMLBaseElement
 * ```html
 * <base>
 * ```
 * @description creates an `HTMLBaseElement` instance.
 * @example ```typescript
 * const baseElement = BASE({className: 'base'});
 * ```
 */ exports.BASE = (0, utils_1.createElementFactory)("base");
/**
 * #### HTMLBodyElement
 * ```html
 * <body>
 * ```
 * @description creates an `HTMLBodyElement` instance.
 * @example
 * ```typescript
 * const body:HTMLBodyElement =
 *     BODY(
 *         {className: 'body'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.BODY = (0, utils_1.createElementFactory)("body");
/**
 * #### HTMLButtonElement
 * ```html
 * <button>
 * ```
 * @description creates an `HTMLButtonElement` instance.
 * @example
 * ```typescript
 * const button:HTMLButtonElement =
 *     BUTTON(
 *         {className: 'button'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.BUTTON = (0, utils_1.createElementFactory)("button");
/**
 * #### HTMLCanvasElement
 * ```html
 * <canvas>
 * ```
 * @description creates an `HTMLCanvasElement` instance.
 * @example
 * ```typescript
 * const canvas:HTMLCanvasElement =
 *     CANVAS(
 *         {className: 'canvas'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.CANVAS = (0, utils_1.createElementFactory)("canvas");
/**
 * #### HTMLElement
 * ```html
 * <cite>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const cite:HTMLElement =
 *     CITE(
 *         {className: 'cite'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.CITE = (0, utils_1.createElementFactory)("cite");
/**
 * #### HTMLElement
 * ```html
 * <code>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const code:HTMLElement =
 *     CODE(
 *         {className: 'code'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.CODE = (0, utils_1.createElementFactory)("code");
/**
 * #### HTMLElement
 * ```html
 * <colgroup>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const colgroup:HTMLElement =
 *     COLGROUP(
 *         {className: 'colgroup'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.COLGROUP = (0, utils_1.createElementFactory)("colgroup");
/**
 * #### HTMLElement
 * ```html
 * <dd>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const dd:HTMLElement =
 *     DD(
 *         {className: 'dd'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DD = (0, utils_1.createElementFactory)("dd");
/**
 * #### HTMLDListElement
 * ```html
 * <dl>
 * ```
 * @description creates an `HTMLDListElement` instance.
 * @example
 * ```typescript
 * const dl:HTMLDListElement =
 *     DL(
 *         {className: 'dl'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DL = (0, utils_1.createElementFactory)("dl");
/**
 * #### HTMLDataElement
 * ```html
 * <data>
 * ```
 * @description creates an `HTMLDataElement` instance.
 * @example
 * ```typescript
 * const data:HTMLDataElement =
 *     DATA(
 *         {className: 'data'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DATA = (0, utils_1.createElementFactory)("data");
/**
 * #### HTMLDataListElement
 * ```html
 * <datalist>
 * ```
 * @description creates an `HTMLDataListElement` instance.
 * @example
 * ```typescript
 * const datalist:HTMLDataListElement =
 *     DATALIST(
 *         {className: 'datalist'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DATALIST = (0, utils_1.createElementFactory)("datalist");
/**
 * #### HTMLDetailsElement
 * ```html
 * <details>
 * ```
 * @description creates an `HTMLDetailsElement` instance.
 * @example
 * ```typescript
 * const details:HTMLDetailsElement =
 *     DETAILS(
 *         {className: 'details'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DETAILS = (0, utils_1.createElementFactory)("details");
/**
 * #### HTMLDialogElement
 * ```html
 * <dialog>
 * ```
 * @description creates an `HTMLDialogElement` instance.
 * @example
 * ```typescript
 * const dialog:HTMLDialogElement =
 *     DIALOG(
 *         {className: 'dialog'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DIALOG = (0, utils_1.createElementFactory)("dialog");
/**
 * #### HTMLDivElement
 * ```html
 * <div>
 * ```
 * @description creates an `HTMLDivElement` instance.
 * @example
 * ```typescript
 * const div:HTMLDivElement =
 *     DIV(
 *         {className: 'div'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DIV = (0, utils_1.createElementFactory)("div");
/**
 * #### HTMLElement
 * ```html
 * <em>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const em:HTMLElement =
 *     EM(
 *         {className: 'em'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.EM = (0, utils_1.createElementFactory)("em");
/**
 * #### HTMLEmbedElement
 * ```html
 * <embed>
 * ```
 * @description creates an `HTMLEmbedElement` instance.
 * @example ```typescript
 * const embedElement = EMBED({className: 'embed'});
 * ```
 */ exports.EMBED = (0, utils_1.createElementFactory)("embed");
/**
 * #### HTMLFieldSetElement
 * ```html
 * <fieldset>
 * ```
 * @description creates an `HTMLFieldSetElement` instance.
 * @example
 * ```typescript
 * const fieldset:HTMLFieldSetElement =
 *     FIELDSET(
 *         {className: 'fieldset'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.FIELDSET = (0, utils_1.createElementFactory)("fieldset");
/**
 * #### HTMLElement
 * ```html
 * <figure>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const figure:HTMLElement =
 *     FIGURE(
 *         {className: 'figure'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.FIGURE = (0, utils_1.createElementFactory)("figure");
/**
 * #### HTMLElement
 * ```html
 * <footer>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const footer:HTMLElement =
 *     FOOTER(
 *         {className: 'footer'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.FOOTER = (0, utils_1.createElementFactory)("footer");
/**
 * #### HTMLFormElement
 * ```html
 * <form>
 * ```
 * @description creates an `HTMLFormElement` instance.
 * @example
 * ```typescript
 * const form:HTMLFormElement =
 *     FORM(
 *         {className: 'form'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.FORM = (0, utils_1.createElementFactory)("form");
/**
 * #### HTMLElement
 * ```html
 * <hgroup>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const hgroup:HTMLElement =
 *     HGROUP(
 *         {className: 'hgroup'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.HGROUP = (0, utils_1.createElementFactory)("hgroup");
/**
 * #### HTMLHRElement
 * ```html
 * <hr>
 * ```
 * @description creates an `HTMLHRElement` instance.
 * @example ```typescript
 * const hrElement = HR({className: 'hr'});
 * ```
 */ exports.HR = (0, utils_1.createElementFactory)("hr");
/**
 * #### HTMLHeadElement
 * ```html
 * <head>
 * ```
 * @description creates an `HTMLHeadElement` instance.
 * @example
 * ```typescript
 * const head:HTMLHeadElement =
 *     HEAD(
 *         {className: 'head'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.HEAD = (0, utils_1.createElementFactory)("head");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h1>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h1:HTMLHeadingElement =
 *     H1(
 *         {className: 'h1'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H1 = (0, utils_1.createElementFactory)("h1");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h2>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h2:HTMLHeadingElement =
 *     H2(
 *         {className: 'h2'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H2 = (0, utils_1.createElementFactory)("h2");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h3>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h3:HTMLHeadingElement =
 *     H3(
 *         {className: 'h3'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H3 = (0, utils_1.createElementFactory)("h3");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h4>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h4:HTMLHeadingElement =
 *     H4(
 *         {className: 'h4'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H4 = (0, utils_1.createElementFactory)("h4");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h5>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h5:HTMLHeadingElement =
 *     H5(
 *         {className: 'h5'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H5 = (0, utils_1.createElementFactory)("h5");
/**
 * #### HTMLHeadingElement
 * ```html
 * <h6>
 * ```
 * @description creates an `HTMLHeadingElement` instance.
 * @example
 * ```typescript
 * const h6:HTMLHeadingElement =
 *     H6(
 *         {className: 'h6'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.H6 = (0, utils_1.createElementFactory)("h6");
/**
 * #### HTMLHtmlElement
 * ```html
 * <html>
 * ```
 * @description creates an `HTMLHtmlElement` instance.
 * @example
 * ```typescript
 * const html:HTMLHtmlElement =
 *     HTML(
 *         {className: 'html'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.HTML = (0, utils_1.createElementFactory)("html");
/**
 * #### HTMLElement
 * ```html
 * <kbd>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const kbd:HTMLElement =
 *     KBD(
 *         {className: 'kbd'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.KBD = (0, utils_1.createElementFactory)("kbd");
/**
 * #### HTMLModElement
 * ```html
 * <ins>
 * ```
 * @description creates an `HTMLModElement` instance.
 * @example
 * ```typescript
 * const ins:HTMLModElement =
 *     INS(
 *         {className: 'ins'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.INS = (0, utils_1.createElementFactory)("ins");
/**
 * #### HTMLElement
 * ```html
 * <i>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const i:HTMLElement =
 *     I(
 *         {className: 'i'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.I = (0, utils_1.createElementFactory)("i");
/**
 * #### HTMLIFrameElement
 * ```html
 * <iframe>
 * ```
 * @description creates an `HTMLIFrameElement` instance.
 * @example
 * ```typescript
 * const iframe:HTMLIFrameElement =
 *     IFRAME(
 *         {className: 'iframe'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.IFRAME = (0, utils_1.createElementFactory)("iframe");
/**
 * #### HTMLImageElement
 * ```html
 * <img>
 * ```
 * @description creates an `HTMLImageElement` instance.
 * @example ```typescript
 * const imgElement = IMG({className: 'img'});
 * ```
 */ exports.IMG = (0, utils_1.createElementFactory)("img");
/**
 * #### HTMLInputElement
 * ```html
 * <input>
 * ```
 * @description creates an `HTMLInputElement` instance.
 * @example ```typescript
 * const inputElement = INPUT({className: 'input'});
 * ```
 */ exports.INPUT = (0, utils_1.createElementFactory)("input");
/**
 * #### HTMLLIElement
 * ```html
 * <li>
 * ```
 * @description creates an `HTMLLIElement` instance.
 * @example
 * ```typescript
 * const li:HTMLLIElement =
 *     LI(
 *         {className: 'li'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.LI = (0, utils_1.createElementFactory)("li");
/**
 * #### HTMLLabelElement
 * ```html
 * <label>
 * ```
 * @description creates an `HTMLLabelElement` instance.
 * @example
 * ```typescript
 * const label:HTMLLabelElement =
 *     LABEL(
 *         {className: 'label'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.LABEL = (0, utils_1.createElementFactory)("label");
/**
 * #### HTMLLegendElement
 * ```html
 * <legend>
 * ```
 * @description creates an `HTMLLegendElement` instance.
 * @example
 * ```typescript
 * const legend:HTMLLegendElement =
 *     LEGEND(
 *         {className: 'legend'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.LEGEND = (0, utils_1.createElementFactory)("legend");
/**
 * #### HTMLLinkElement
 * ```html
 * <link>
 * ```
 * @description creates an `HTMLLinkElement` instance.
 * @example ```typescript
 * const linkElement = LINK({className: 'link'});
 * ```
 */ exports.LINK = (0, utils_1.createElementFactory)("link");
/**
 * #### HTMLElement
 * ```html
 * <main>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const main:HTMLElement =
 *     MAIN(
 *         {className: 'main'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.MAIN = (0, utils_1.createElementFactory)("main");
/**
 * #### HTMLMapElement
 * ```html
 * <map>
 * ```
 * @description creates an `HTMLMapElement` instance.
 * @example
 * ```typescript
 * const map:HTMLMapElement =
 *     MAP(
 *         {className: 'map'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.MAP = (0, utils_1.createElementFactory)("map");
/**
 * #### HTMLMenuElement
 * ```html
 * <menu>
 * ```
 * @description creates an `HTMLMenuElement` instance.
 * @example
 * ```typescript
 * const menu:HTMLMenuElement =
 *     MENU(
 *         {className: 'menu'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.MENU = (0, utils_1.createElementFactory)("menu");
/**
 * #### HTMLElement
 * ```html
 * <mark>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const mark:HTMLElement =
 *     MARK(
 *         {className: 'mark'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.MARK = (0, utils_1.createElementFactory)("mark");
/**
 * #### HTMLElement
 * ```html
 * <nav>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const nav:HTMLElement =
 *     NAV(
 *         {className: 'nav'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.NAV = (0, utils_1.createElementFactory)("nav");
/**
 * #### HTMLElement
 * ```html
 * <noscript>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const noscript:HTMLElement =
 *     NOSCRIPT(
 *         {className: 'noscript'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.NOSCRIPT = (0, utils_1.createElementFactory)("noscript");
/**
 * #### HTMLMetaElement
 * ```html
 * <meta>
 * ```
 * @description creates an `HTMLMetaElement` instance.
 * @example ```typescript
 * const metaElement = META({className: 'meta'});
 * ```
 */ exports.META = (0, utils_1.createElementFactory)("meta");
/**
 * #### HTMLMeterElement
 * ```html
 * <meter>
 * ```
 * @description creates an `HTMLMeterElement` instance.
 * @example
 * ```typescript
 * const meter:HTMLMeterElement =
 *     METER(
 *         {className: 'meter'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.METER = (0, utils_1.createElementFactory)("meter");
/**
 * #### HTMLModElement
 * ```html
 * <del>
 * ```
 * @description creates an `HTMLModElement` instance.
 * @example
 * ```typescript
 * const del:HTMLModElement =
 *     DEL(
 *         {className: 'del'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.DEL = (0, utils_1.createElementFactory)("del");
/**
 * #### HTMLOListElement
 * ```html
 * <ol>
 * ```
 * @description creates an `HTMLOListElement` instance.
 * @example
 * ```typescript
 * const ol:HTMLOListElement =
 *     OL(
 *         {className: 'ol'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.OL = (0, utils_1.createElementFactory)("ol");
/**
 * #### HTMLObjectElement
 * ```html
 * <object>
 * ```
 * @description creates an `HTMLObjectElement` instance.
 * @example
 * ```typescript
 * const object:HTMLObjectElement =
 *     OBJECT(
 *         {className: 'object'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.OBJECT = (0, utils_1.createElementFactory)("object");
/**
 * #### HTMLOptGroupElement
 * ```html
 * <optgroup>
 * ```
 * @description creates an `HTMLOptGroupElement` instance.
 * @example
 * ```typescript
 * const optgroup:HTMLOptGroupElement =
 *     OPTGROUP(
 *         {className: 'optgroup'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.OPTGROUP = (0, utils_1.createElementFactory)("optgroup");
/**
 * #### HTMLOptionElement
 * ```html
 * <option>
 * ```
 * @description creates an `HTMLOptionElement` instance.
 * @example
 * ```typescript
 * const option:HTMLOptionElement =
 *     OPTION(
 *         {className: 'option'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.OPTION = (0, utils_1.createElementFactory)("option");
/**
 * #### HTMLOutputElement
 * ```html
 * <output>
 * ```
 * @description creates an `HTMLOutputElement` instance.
 * @example
 * ```typescript
 * const output:HTMLOutputElement =
 *     OUTPUT(
 *         {className: 'output'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.OUTPUT = (0, utils_1.createElementFactory)("output");
/**
 * #### HTMLParagraphElement
 * ```html
 * <p>
 * ```
 * @description creates an `HTMLParagraphElement` instance.
 * @example
 * ```typescript
 * const p:HTMLParagraphElement =
 *     P(
 *         {className: 'p'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.P = (0, utils_1.createElementFactory)("p");
/**
 * #### HTMLPictureElement
 * ```html
 * <picture>
 * ```
 * @description creates an `HTMLPictureElement` instance.
 * @example
 * ```typescript
 * const picture:HTMLPictureElement =
 *     PICTURE(
 *         {className: 'picture'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.PICTURE = (0, utils_1.createElementFactory)("picture");
/**
 * #### HTMLPreElement
 * ```html
 * <pre>
 * ```
 * @description creates an `HTMLPreElement` instance.
 * @example
 * ```typescript
 * const pre:HTMLPreElement =
 *     PRE(
 *         {className: 'pre'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.PRE = (0, utils_1.createElementFactory)("pre");
/**
 * #### HTMLProgressElement
 * ```html
 * <progress>
 * ```
 * @description creates an `HTMLProgressElement` instance.
 * @example
 * ```typescript
 * const progress:HTMLProgressElement =
 *     PROGRESS(
 *         {className: 'progress'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.PROGRESS = (0, utils_1.createElementFactory)("progress");
/**
 * #### HTMLElement
 * ```html
 * <rt>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const rt:HTMLElement =
 *     RT(
 *         {className: 'rt'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.RT = (0, utils_1.createElementFactory)("rt");
/**
 * #### HTMLElement
 * ```html
 * <rp>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const rp:HTMLElement =
 *     RP(
 *         {className: 'rp'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.RP = (0, utils_1.createElementFactory)("rp");
/**
 * #### HTMLElement
 * ```html
 * <ruby>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const ruby:HTMLElement =
 *     RUBY(
 *         {className: 'ruby'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.RUBY = (0, utils_1.createElementFactory)("ruby");
/**
 * #### HTMLElement
 * ```html
 * <small>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const small:HTMLElement =
 *     SMALL(
 *         {className: 'small'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SMALL = (0, utils_1.createElementFactory)("small");
/**
 * #### HTMLQuoteElement
 * ```html
 * <q>
 * ```
 * @description creates an `HTMLQuoteElement` instance.
 * @example
 * ```typescript
 * const q:HTMLQuoteElement =
 *     Q(
 *         {className: 'q'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.Q = (0, utils_1.createElementFactory)("q");
/**
 * #### HTMLQuoteElement
 * ```html
 * <blockquote>
 * ```
 * @description creates an `HTMLQuoteElement` instance.
 * @example
 * ```typescript
 * const blockquote:HTMLQuoteElement =
 *     BLOCKQUOTE(
 *         {className: 'blockquote'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.BLOCKQUOTE = (0, utils_1.createElementFactory)("blockquote");
/**
 * #### HTMLScriptElement
 * ```html
 * <script>
 * ```
 * @description creates an `HTMLScriptElement` instance.
 * @example
 * ```typescript
 * const script:HTMLScriptElement =
 *     SCRIPT(
 *         {className: 'script'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SCRIPT = (0, utils_1.createElementFactory)("script");
/**
 * #### HTMLSelectElement
 * ```html
 * <select>
 * ```
 * @description creates an `HTMLSelectElement` instance.
 * @example
 * ```typescript
 * const select:HTMLSelectElement =
 *     SELECT(
 *         {className: 'select'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SELECT = (0, utils_1.createElementFactory)("select");
/**
 * #### HTMLSlotElement
 * ```html
 * <slot>
 * ```
 * @description creates an `HTMLSlotElement` instance.
 * @example
 * ```typescript
 * const slot:HTMLSlotElement =
 *     SLOT(
 *         {className: 'slot'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SLOT = (0, utils_1.createElementFactory)("slot");
/**
 * #### HTMLSourceElement
 * ```html
 * <source>
 * ```
 * @description creates an `HTMLSourceElement` instance.
 * @example ```typescript
 * const sourceElement = SOURCE({className: 'source'});
 * ```
 */ exports.SOURCE = (0, utils_1.createElementFactory)("source");
/**
 * #### HTMLSpanElement
 * ```html
 * <span>
 * ```
 * @description creates an `HTMLSpanElement` instance.
 * @example
 * ```typescript
 * const span:HTMLSpanElement =
 *     SPAN(
 *         {className: 'span'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SPAN = (0, utils_1.createElementFactory)("span");
/**
 * #### HTMLStyleElement
 * ```html
 * <style>
 * ```
 * @description creates an `HTMLStyleElement` instance.
 * @example
 * ```typescript
 * const stylesheet = STYLE(css`
 *     .my-class {
 *         color: red;
 *     }
 * `);

 * document.head.append(stylesheet)
 * ```
 */ exports.STYLE = (0, utils_1.createElementFactory)("style");
/**
 * #### HTMLElement
 * ```html
 * <sub>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const sub:HTMLElement =
 *     SUB(
 *         {className: 'sub'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SUB = (0, utils_1.createElementFactory)("sub");
/**
 * #### HTMLElement
 * ```html
 * <summary>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const summary:HTMLElement =
 *     SUMMARY(
 *         {className: 'summary'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SUMMARY = (0, utils_1.createElementFactory)("summary");
/**
 * #### HTMLElement
 * ```html
 * <search>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const search:HTMLElement =
 *     SEARCH(
 *         {className: 'search'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SEARCH = (0, utils_1.createElementFactory)("search");
/**
 * #### HTMLElement
 * ```html
 * <section>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const section:HTMLElement =
 *     SECTION(
 *         {className: 'section'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.SECTION = (0, utils_1.createElementFactory)("section");
/**
 * #### HTMLTableCaptionElement
 * ```html
 * <caption>
 * ```
 * @description creates an `HTMLTableCaptionElement` instance.
 * @example
 * ```typescript
 * const caption:HTMLTableCaptionElement =
 *     CAPTION(
 *         {className: 'caption'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.CAPTION = (0, utils_1.createElementFactory)("caption");
/**
 * #### HTMLTableCellElement
 * ```html
 * <th>
 * ```
 * @description creates an `HTMLTableCellElement` instance.
 * @example
 * ```typescript
 * const th:HTMLTableCellElement =
 *     TH(
 *         {className: 'th'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TH = (0, utils_1.createElementFactory)("th");
/**
 * #### HTMLTableCellElement
 * ```html
 * <td>
 * ```
 * @description creates an `HTMLTableCellElement` instance.
 * @example
 * ```typescript
 * const td:HTMLTableCellElement =
 *     TD(
 *         {className: 'td'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TD = (0, utils_1.createElementFactory)("td");
/**
 * #### HTMLTableColElement
 * ```html
 * <col>
 * ```
 * @description creates an `HTMLTableColElement` instance.
 * @example ```typescript
 * const colElement = COL({className: 'col'});
 * ```
 */ exports.COL = (0, utils_1.createElementFactory)("col");
/**
 * #### HTMLTableElement
 * ```html
 * <table>
 * ```
 * @description creates an `HTMLTableElement` instance.
 * @example
 * ```typescript
 * const table:HTMLTableElement =
 *     TABLE(
 *         {className: 'table'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TABLE = (0, utils_1.createElementFactory)("table");
/**
 * #### HTMLTableRowElement
 * ```html
 * <tr>
 * ```
 * @description creates an `HTMLTableRowElement` instance.
 * @example
 * ```typescript
 * const tr:HTMLTableRowElement =
 *     TR(
 *         {className: 'tr'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TR = (0, utils_1.createElementFactory)("tr");
/**
 * #### HTMLTableSectionElement
 * ```html
 * <thead>
 * ```
 * @description creates an `HTMLTableSectionElement` instance.
 * @example
 * ```typescript
 * const thead:HTMLTableSectionElement =
 *     THEAD(
 *         {className: 'thead'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.THEAD = (0, utils_1.createElementFactory)("thead");
/**
 * #### HTMLTableSectionElement
 * ```html
 * <tfoot>
 * ```
 * @description creates an `HTMLTableSectionElement` instance.
 * @example
 * ```typescript
 * const tfoot:HTMLTableSectionElement =
 *     TFOOT(
 *         {className: 'tfoot'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TFOOT = (0, utils_1.createElementFactory)("tfoot");
/**
 * #### HTMLTableSectionElement
 * ```html
 * <tbody>
 * ```
 * @description creates an `HTMLTableSectionElement` instance.
 * @example
 * ```typescript
 * const tbody:HTMLTableSectionElement =
 *     TBODY(
 *         {className: 'tbody'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TBODY = (0, utils_1.createElementFactory)("tbody");
/**
 * #### HTMLTemplateElement
 * ```html
 * <template>
 * ```
 * @description creates an `HTMLTemplateElement` instance.
 * @example
 * ```typescript
 * const template:HTMLTemplateElement =
 *     TEMPLATE(
 *         {className: 'template'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TEMPLATE = (0, utils_1.createElementFactory)("template");
/**
 * #### HTMLTextAreaElement
 * ```html
 * <textarea>
 * ```
 * @description creates an `HTMLTextAreaElement` instance.
 * @example
 * ```typescript
 * const textarea:HTMLTextAreaElement =
 *     TEXTAREA(
 *         {className: 'textarea'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TEXTAREA = (0, utils_1.createElementFactory)("textarea");
/**
 * #### HTMLTimeElement
 * ```html
 * <time>
 * ```
 * @description creates an `HTMLTimeElement` instance.
 * @example
 * ```typescript
 * const time:HTMLTimeElement =
 *     TIME(
 *         {className: 'time'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TIME = (0, utils_1.createElementFactory)("time");
/**
 * #### HTMLTitleElement
 * ```html
 * <title>
 * ```
 * @description creates an `HTMLTitleElement` instance.
 * @example
 * ```typescript
 * const title:HTMLTitleElement =
 *     TITLE(
 *         {className: 'title'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.TITLE = (0, utils_1.createElementFactory)("title");
/**
 * #### HTMLTrackElement
 * ```html
 * <track>
 * ```
 * @description creates an `HTMLTrackElement` instance.
 * @example ```typescript
 * const trackElement = TRACK({className: 'track'});
 * ```
 */ exports.TRACK = (0, utils_1.createElementFactory)("track");
/**
 * #### HTMLElement
 * ```html
 * <u>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const u:HTMLElement =
 *     U(
 *         {className: 'u'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.U = (0, utils_1.createElementFactory)("u");
/**
 * #### HTMLUListElement
 * ```html
 * <ul>
 * ```
 * @description creates an `HTMLUListElement` instance.
 * @example
 * ```typescript
 * const ul:HTMLUListElement =
 *     UL(
 *         {className: 'ul'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.UL = (0, utils_1.createElementFactory)("ul");
/**
 * #### HTMLVideoElement
 * ```html
 * <video>
 * ```
 * @description creates an `HTMLVideoElement` instance.
 * @example
 * ```typescript
 * const video:HTMLVideoElement =
 *     VIDEO(
 *         {className: 'video'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.VIDEO = (0, utils_1.createElementFactory)("video");
/**
 * #### HTMLElement
 * ```html
 * <var>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example
 * ```typescript
 * const var:HTMLElement =
 *     VAR(
 *         {className: 'var'},
 *         'Hello',
 *         SPAN({style:{color:'red'}}, 'World')
 *     );
 * ```
 */ exports.VAR = (0, utils_1.createElementFactory)("var");
/**
 * #### HTMLElement
 * ```html
 * <wbr>
 * ```
 * @description creates an `HTMLElement` instance.
 * @example ```typescript
 * const wbrElement = WBR({className: 'wbr'});
 * ```
 */ exports.WBR = (0, utils_1.createElementFactory)("wbr");
exports.FRAGMENT = utils_1.createFragment;

},{"ce6b4a056d7b02cf":"jlkVB"}],"fkPEc":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.isProps = void 0;
var utils_1 = require("c775d0cb4f8a2784");
Object.defineProperty(exports, "isProps", {
    enumerable: true,
    get: function() {
        return utils_1.isProps;
    }
});

},{"c775d0cb4f8a2784":"7JVjX"}],"6kk03":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.navigate = exports.setHash = exports.setSearch = exports.getPathname = exports.getHash = exports.getSearch = exports.getParams = void 0;
exports.default = Router;
const oxidizer_1 = require("4a5c4b6c022c5436");
const utils_1 = require("acf308670dc2778b");
const props = (0, oxidizer_1.createProps)({
    path: window.location.pathname
}, [
    (0, oxidizer_1.createEffect)('path', ({ path })=>{
        try {
            window.history.pushState('page', 'title', path);
        } catch (e) {
            throw new utils_1.RouterError(`Failed to navigate path "${path}"`);
        }
    })
]);
let routeParams = {};
const walkRoute = (routes, route)=>{
    route = utils_1.Url.getPathname(route);
    routeParams = {};
    const routeArr = route.split('/').filter((x)=>x).map((r)=>`/${r}`);
    let currRoutes = routes;
    if (routeArr.length === 0) routeArr.push('/');
    for(const i in routeArr){
        if (!currRoutes) throw new utils_1.RouterError('Router object must not be null or undefined');
        const r = routeArr[i];
        if (r in currRoutes || `${r}/` in currRoutes) currRoutes = currRoutes[r];
        else {
            const [param, paramRoute] = (0, utils_1.getParamRoute)(currRoutes);
            if (paramRoute) {
                currRoutes = paramRoute;
                routeParams[param] = r.replace('/', '');
            } else {
                const starRoute = (0, utils_1.getStarRoute)(currRoutes);
                if (starRoute) currRoutes = starRoute;
            }
        }
        // break if function
        if (typeof currRoutes === "function") break;
    }
    if (typeof currRoutes === "function") return currRoutes;
    else if (!currRoutes) throw new utils_1.RouterError('Router object must not be null or undefined');
    else {
        const indexRoute = (0, utils_1.getIndexRoute)(currRoutes);
        if (indexRoute) {
            if (typeof indexRoute === "function") return indexRoute;
            else throw new utils_1.RouterError('index route must be of type function');
        } else throw new utils_1.RouterError(`No routes matched current route "${route}". Consider adding a dynamic (/:), star (/*) or index (/%) route.`);
    }
};
const RouterComponent = (0, oxidizer_1.createComponent)('oxidizer-router', class extends HTMLElement {
});
const getParams = ()=>routeParams;
exports.getParams = getParams;
const getSearch = ()=>utils_1.SearchParams.stringToRecord(window.location.search);
exports.getSearch = getSearch;
const getHash = ()=>window.location.hash.replace('#', '');
exports.getHash = getHash;
const getPathname = ()=>utils_1.Url.getPathname(props.path);
exports.getPathname = getPathname;
const setSearch = (search)=>{
    const searchString = typeof search === "string" ? search : utils_1.SearchParams.recordToString(search);
    const url = utils_1.Url.get(props.path);
    url.search = searchString;
    props.path = utils_1.Url.removeOrigin(url);
};
exports.setSearch = setSearch;
const setHash = (hash)=>{
    const url = utils_1.Url.get(props.path);
    url.hash = hash;
    props.path = utils_1.Url.removeOrigin(url);
};
exports.setHash = setHash;
const navigate = (route, { hash, search } = {})=>{
    if (!route.startsWith('/')) route = props.path + '/' + route;
    const url = utils_1.Url.get(route);
    if (search) url.search = typeof search === "string" ? search : utils_1.SearchParams.recordToString(search);
    if (hash) url.hash = hash;
    props.path = utils_1.Url.removeOrigin(url);
};
exports.navigate = navigate;
function Router(routes) {
    return RouterComponent(props, ($)=>[
            walkRoute(routes, $.path)()
        ]);
}

},{"4a5c4b6c022c5436":"hnuU9","acf308670dc2778b":"ixfvL"}],"ixfvL":[function(require,module,exports,__globalThis) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.RouterError = exports.Url = exports.SearchParams = exports.getIndexRoute = exports.getStarRoute = exports.getParamRoute = void 0;
const routePrefixes = {
    star: '*',
    index: 'index',
    parameter: ':'
};
const getParamRoute = (routes)=>{
    const paramRoute = Object.entries(routes).filter(([k, v])=>k.startsWith(routePrefixes.parameter))[0];
    if (paramRoute) {
        paramRoute[0] = paramRoute[0].replaceAll(':', '').replace('/', '');
        return paramRoute;
    } else return [
        '/:',
        null
    ];
};
exports.getParamRoute = getParamRoute;
const getStarRoute = (routes)=>{
    const starRoute = Object.entries(routes).filter(([k, v])=>k.startsWith(routePrefixes.star))[0];
    return starRoute ? starRoute[1] : null;
};
exports.getStarRoute = getStarRoute;
const getIndexRoute = (routes)=>{
    const indexRoute = Object.entries(routes).filter(([k, v])=>k.startsWith(routePrefixes.index))[0];
    return indexRoute ? indexRoute[1] : null;
};
exports.getIndexRoute = getIndexRoute;
exports.SearchParams = {
    stringToRecord: (searchString)=>{
        return Object.fromEntries(new URLSearchParams(searchString));
    },
    recordToString: (searchRecord)=>{
        return new URLSearchParams(Object.entries(searchRecord)).toString();
    }
};
exports.Url = {
    get origin () {
        return window.location.origin;
    },
    get (suburl) {
        return new URL(this.origin + suburl);
    },
    getPathname (suburl) {
        const url = new URL(this.origin + suburl);
        return url.pathname;
    },
    getSubUrl (suburl) {
        const url = this.get(suburl);
        return url.href.replace(url.origin, '');
    },
    removeOrigin (url) {
        return url.href.replace(url.origin, '');
    }
};
class RouterError extends Error {
    constructor(message){
        super(message);
        this.name = 'RouterError';
    }
}
exports.RouterError = RouterError;

},{}],"hNbmf":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "syncRoute", ()=>syncRoute);
parcelHelpers.export(exports, "goTo", ()=>goTo);
parcelHelpers.export(exports, "Sidebar", ()=>Sidebar);
var _oxidizer = require("oxidizer");
var _oxidizerRouter = require("oxidizer-router");
var _content = require("../content");
var _components = require("../components");
var _app = require("../App");
function activeSection(path) {
    const match = path.match(/^\/docs\/([^/]+)/);
    return match?.[1] ?? "";
}
function syncRoute(shell) {
    shell.path = (0, _oxidizerRouter.getPathname)();
    shell.routeKey = window.location.search;
}
function goTo(path, shell) {
    const { q } = (0, _oxidizerRouter.getSearch)();
    (0, _oxidizerRouter.navigate)(path.startsWith((0, _app.BASE_PATH)) ? path : (0, _app.BASE_PATH) + path, q ? {
        search: {
            q
        }
    } : undefined);
    syncRoute(shell);
    shell.open = false;
}
function setSearchQuery(shell, q) {
    (0, _oxidizerRouter.setSearch)(q ? {
        q
    } : {});
    shell.routeKey = window.location.search;
}
function Sidebar(shell) {
    return (0, _oxidizer.ASIDE)(shell, (p)=>{
        const query = ((0, _oxidizerRouter.getSearch)().q ?? "").toLowerCase().trim();
        const current = activeSection(p.path);
        const groups = [
            ...new Set((0, _content.NAV_ITEMS).map((n)=>n.group ?? ""))
        ];
        const filtered = (0, _content.NAV_ITEMS).filter((item)=>!query || item.label.toLowerCase().includes(query) || item.id.includes(query));
        return [
            {
                className: `sidebar${p.open ? " open" : ""}`
            },
            (0, _oxidizer.DIV)({
                className: "sidebar-header"
            }, (0, _oxidizer.BUTTON)({
                className: "logo",
                type: "button",
                onclick: ()=>goTo("/", p)
            }, (0, _oxidizer.SPAN)({
                className: "logo-icon"
            }, "x"), (0, _oxidizer.SPAN)({
                className: "logo-text"
            }, ".sh")), (0, _oxidizer.BUTTON)({
                className: "sidebar-close",
                type: "button",
                "aria-label": "Close menu",
                onclick: ()=>{
                    p.open = false;
                }
            }, "\xd7")),
            (0, _oxidizer.DIV)({
                className: "sidebar-search"
            }, (0, _oxidizer.INPUT)({
                type: "search",
                className: "search-input",
                placeholder: "Search docs\u2026",
                value: (0, _oxidizerRouter.getSearch)().q ?? "",
                oninput (e) {
                    setSearchQuery(p, e.target.value);
                },
                onload () {
                    console.log("focusing");
                    this.focus();
                },
                onloadstart () {
                    console.log("focusing");
                    this.focus();
                }
            })),
            (0, _oxidizer.NAV)({
                className: "sidebar-nav"
            }, (0, _oxidizer.UL)({
                className: "nav-list"
            }, (0, _oxidizer.LI)({}, (0, _oxidizer.BUTTON)({
                className: `nav-link${p.path === "/" ? " active" : ""}`,
                type: "button",
                onclick: ()=>goTo("/", p)
            }, "Home"))), ...groups.map((group)=>{
                const items = filtered.filter((i)=>(i.group ?? "") === group);
                if (!items.length) return null;
                return (0, _oxidizer.DIV)({
                    className: "nav-group"
                }, group ? (0, _oxidizer.SPAN)({
                    className: "nav-group-label"
                }, group) : null, (0, _oxidizer.UL)({
                    className: "nav-list"
                }, ...items.map((item)=>(0, _oxidizer.LI)({}, (0, _oxidizer.BUTTON)({
                        className: `nav-link${current === item.id ? " active" : ""}`,
                        type: "button",
                        onclick: ()=>goTo(`/docs/${item.id}`, p)
                    }, ...(0, _components.renderInline)(item.label))))));
            })),
            (0, _oxidizer.DIV)({
                className: "sidebar-footer"
            }, (0, _oxidizer.A)({
                href: "https://github.com/michaelmunson/x.sh",
                className: "github-link",
                target: "_blank",
                rel: "noopener noreferrer"
            }, "GitHub \u2192"))
        ];
    });
}

},{"oxidizer":"hnuU9","oxidizer-router":"6kk03","../content":"hqNCP","../components":"ip12w","../App":"4Ye6C","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"hqNCP":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "NAV_ITEMS", ()=>NAV_ITEMS);
parcelHelpers.export(exports, "SECTIONS", ()=>SECTIONS);
var _examples = require("./examples");
const NAV_ITEMS = [
    {
        id: "overview",
        label: "Overview",
        group: "Getting Started"
    },
    {
        id: "installation",
        label: "Installation",
        group: "Getting Started"
    },
    {
        id: "quickstart",
        label: "Quick Start",
        group: "Getting Started"
    },
    {
        id: "architecture",
        label: "Architecture",
        group: "Getting Started"
    },
    {
        id: "commands",
        label: "Commands",
        group: "Reference"
    },
    {
        id: "global-scripts",
        label: "Global Scripts",
        group: "Reference"
    },
    {
        id: "project-local",
        label: "Local Scripts",
        group: "Reference"
    },
    {
        id: "app-framework",
        label: "App Framework",
        group: "Apps"
    },
    {
        id: "app-format",
        label: "App File Format",
        group: "Apps"
    },
    {
        id: "synopsis",
        label: "Synopsis DSL",
        group: "Apps"
    },
    {
        id: "builtins",
        label: "Built-in Helpers",
        group: "Apps"
    },
    {
        id: "validation-help",
        label: "Validation & Help",
        group: "Apps"
    },
    {
        id: "examples",
        label: "Examples",
        group: "Apps"
    },
    {
        id: "configuration",
        label: "Configuration",
        group: "Advanced"
    },
    {
        id: "ai",
        label: "AI Integration",
        group: "Advanced"
    },
    {
        id: "languages",
        label: "Supported Languages",
        group: "Advanced"
    },
    {
        id: "faq",
        label: "FAQ",
        group: "Advanced"
    }
];
const SECTIONS = [
    {
        id: "overview",
        title: "Overview",
        subtitle: "A personal script manager and lightweight CLI app framework for your shell.",
        blocks: [
            {
                type: "p",
                text: "**x** is a Rust-powered CLI that helps you create, organize, and run scripts across projects. Think of it as a script registry with metadata, editor integration, and optional symlinks into your PATH \u2014 plus two YAML-based layers for project commands and full multi-command apps."
            },
            {
                type: "h3",
                text: "What makes x different"
            },
            {
                type: "ul",
                items: [
                    "**Global scripts** live in `~/.x.sh/scripts/` with per-script metadata (description, language, groups) and usage tracking.",
                    "**Project-local commands** in `./x.yml` let you easily define project specific scripts and commands.",
                    "**Apps** (`&lt;name&gt;.x.yml`) turn YAML into validated CLIs with options, arguments, nested subcommands, and bash handlers \u2014 before any bash runs, x parses and validates the user's input."
                ]
            },
            {
                type: "h3",
                text: "Three ways to run a command"
            },
            {
                type: "code",
                title: "Resolution order when you run `x &lt;name&gt; \u2026`",
                code: `1. ./x.yml          \u{2192} project-local inline script (CWD only)
2. ./&lt;name&gt;.x.yml    \u{2192} local app (CWD, then parent dirs)
3. ~/.x.sh/apps/     \u{2192} global app
4. ~/.x.sh/scripts/  \u{2192} installed global script`
            },
            {
                type: "callout",
                variant: "tip",
                text: "Use `x --src &lt;name&gt;` to print the absolute path of a script or app file without executing it \u2014 handy for piping into editors or CI."
            }
        ]
    },
    {
        id: "installation",
        title: "Installation",
        subtitle: "Get the `x` binary on your system in minutes.",
        blocks: [
            {
                type: "h3",
                text: "Automated install (recommended)"
            },
            {
                type: "p",
                text: "The install script clones the repo, builds in release mode, and places the binary in `~/.local/bin/`."
            },
            {
                type: "code",
                title: "One-liner",
                code: `curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash`
            },
            {
                type: "p",
                text: "Or download and run manually:"
            },
            {
                type: "code",
                code: `wget https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh
bash install.sh`
            },
            {
                type: "h4",
                text: "Requirements"
            },
            {
                type: "ul",
                items: [
                    "**Rust / Cargo** \u2014 the script prompts you to install via rustup if missing.",
                    "**Git** \u2014 used to clone the repository (skipped if you run the script from inside the repo).",
                    "An internet connection for the initial clone and build."
                ]
            },
            {
                type: "h4",
                text: "What the script does"
            },
            {
                type: "ol",
                items: [
                    "Checks for Rust/Cargo.",
                    "Clones `github.com/michaelmunson/x.sh` (or uses the current directory if already in the repo).",
                    "Runs `cargo build --release`.",
                    "Copies `target/release/x` to `~/.local/bin/x`.",
                    "Verifies the installation by running `x --help`."
                ]
            },
            {
                type: "callout",
                variant: "warn",
                title: "PATH",
                text: "Ensure `~/.local/bin` is on your PATH. Add `export PATH=\"$HOME/.local/bin:$PATH\"` to `~/.bashrc`, `~/.zshrc`, or `~/.profile`, then restart your shell."
            },
            {
                type: "h3",
                text: "Manual build"
            },
            {
                type: "code",
                code: `git clone https://github.com/michaelmunson/x.sh.git
cd x.sh
cargo build --release
# Binary at target/release/x \u{2014} copy or symlink into your PATH`
            },
            {
                type: "h3",
                text: "First-run directory layout"
            },
            {
                type: "p",
                text: "On first use, x creates `~/.x.sh/` with the following structure:"
            },
            {
                type: "code",
                title: "~/.x.sh/",
                code: `~/.x.sh/
\u{251C}\u{2500}\u{2500} scripts/          # your global script files
\u{251C}\u{2500}\u{2500} apps/             # global app YAML files (&lt;name&gt;.x.yml)
\u{251C}\u{2500}\u{2500} metadata/         # per-script TOML metadata
\u{2502}   \u{2514}\u{2500}\u{2500} &lt;name&gt;.toml
\u{251C}\u{2500}\u{2500} config.json       # default_program and other settings
\u{251C}\u{2500}\u{2500} metadata.json     # usage / activity tracking
\u{2514}\u{2500}\u{2500} config/
    \u{2514}\u{2500}\u{2500} llm.sh        # LLM provider script (after x --config)`
            }
        ]
    },
    {
        id: "quickstart",
        title: "Quick Start",
        subtitle: "From zero to a runnable script in under a minute.",
        blocks: [
            {
                type: "h3",
                text: "1. Create your first script"
            },
            {
                type: "code",
                code: `x --init hello-world
# or: x -i hello-world`
            },
            {
                type: "p",
                text: "Interactive prompts ask for the script name (alphanumeric + dashes), an optional description, and a programming language. Your `$EDITOR` opens so you can write the script body."
            },
            {
                type: "h3",
                text: "2. Run it"
            },
            {
                type: "code",
                code: `x hello-world
x hello-world arg1 arg2   # args pass through to the script`
            },
            {
                type: "h3",
                text: "3. List installed scripts"
            },
            {
                type: "code",
                code: `x --ls
# or: x -l`
            },
            {
                type: "p",
                text: "Shows a formatted table with name, description, language, groups, and last-run timestamps."
            },
            {
                type: "h3",
                text: "4. Add to PATH (optional)"
            },
            {
                type: "code",
                code: `x --ln hello-world
# Now run directly: hello-world`
            },
            {
                type: "h3",
                text: "5. Project-local commands (optional)"
            },
            {
                type: "p",
                text: "Drop an `x.yml` in your project root to define repo-specific tasks:"
            },
            {
                type: "code",
                title: "x.yml",
                code: `test: npm test
build: |
  npm run build
  echo "Built $1"   # $1 = first arg after 'build'`
            },
            {
                type: "code",
                code: `x test
x build v1.0.0`
            }
        ]
    },
    {
        id: "architecture",
        title: "Architecture",
        subtitle: "How x resolves, stores, and executes commands.",
        blocks: [
            {
                type: "h3",
                text: "Command resolution flow"
            },
            {
                type: "p",
                text: "When you invoke `x &lt;name&gt; [args\u2026]`, x walks a fixed resolution chain. The first match wins and the process exits after execution."
            },
            {
                type: "h3",
                text: "Local app discovery"
            },
            {
                type: "p",
                text: "Unlike `x.yml` (current directory only), **local apps** are discovered by walking from the current working directory up through every parent directory. The nearest `&lt;name&gt;.x.yml` wins. This lets monorepos keep a shared app at the repo root while you work in subdirectories."
            },
            {
                type: "h3",
                text: "Global script lookup"
            },
            {
                type: "p",
                text: "Scripts in `~/.x.sh/scripts/` are matched by exact filename first, then by basename without extension \u2014 so `x c-test` finds `c-test.c` automatically."
            },
            {
                type: "h3",
                text: "Metadata model"
            },
            {
                type: "p",
                text: "Each global script can have a companion TOML file at `~/.x.sh/metadata/&lt;name&gt;.toml`:"
            },
            {
                type: "code",
                title: "Example metadata",
                code: `description = "Fetch a URL with curl"
groups = ["network", "utils"]
program = "bash"`
            },
            {
                type: "p",
                text: "Activity data (created, updated, last_executed) is stored separately in `~/.x.sh/metadata.json` and shown in `x --ls`."
            },
            {
                type: "h3",
                text: "Inline script execution"
            },
            {
                type: "p",
                text: "Project-local `x.yml` entries and certain inline paths run via your configured default program (typically `bash`):"
            },
            {
                type: "code",
                code: `&lt;program&gt; -c '&lt;script body&gt;' x &lt;remaining-args\u{2026}&gt;`
            },
            {
                type: "p",
                text: "Positional parameters in the script body (`$1`, `$2`, \u2026) refer to arguments after the command name."
            }
        ]
    },
    {
        id: "commands",
        title: "Commands",
        subtitle: "Complete CLI reference for every flag and subcommand.",
        blocks: [
            {
                type: "h3",
                text: "`x --init [name] [script]` / `x -i`"
            },
            {
                type: "p",
                text: "Create a new global script or re-open an existing one in your editor."
            },
            {
                type: "ul",
                items: [
                    "`x -i` \u2014 fully interactive (name, description, language, editor).",
                    "`x -i my-script` \u2014 prompts for metadata, opens editor. If the script exists, you can rename it first; metadata and history follow the new name.",
                    '`x -i my-script "echo hello"` \u2014 creates the script with inline content using the default or existing program.'
                ]
            },
            {
                type: "callout",
                variant: "info",
                text: "Script names must be non-empty and contain only letters, numbers, and dashes."
            },
            {
                type: "h3",
                text: "`x &lt;script_name&gt; [args\u2026]`"
            },
            {
                type: "p",
                text: "Execute a command using the resolution order described in Architecture. Arguments are forwarded verbatim."
            },
            {
                type: "h3",
                text: "`x --ls` / `x -l`"
            },
            {
                type: "p",
                text: "List all global scripts in a table with description, program, groups, and activity timestamps."
            },
            {
                type: "h3",
                text: "`x --delete &lt;name&gt;` / `x -d`"
            },
            {
                type: "p",
                text: "Remove a global script and its metadata. Does **not** remove symlinks created with `x --ln` \u2014 delete those separately."
            },
            {
                type: "h3",
                text: "`x --ln &lt;name&gt; [link_name]`"
            },
            {
                type: "p",
                text: "Symlink a script into `~/.local/bin/` so you can invoke it without the `x` prefix."
            },
            {
                type: "code",
                code: `x --ln my-script          # \u{2192} ~/.local/bin/my-script
x --ln my-script alias    # \u{2192} ~/.local/bin/alias
x -d --ln my-script       # remove the symlink`
            },
            {
                type: "h3",
                text: "`x --src &lt;name&gt;`"
            },
            {
                type: "p",
                text: "Print the absolute path of a script or app file. Resolution: local app \u2192 global app \u2192 global script. `x.yml` keys are not matched."
            },
            {
                type: "h3",
                text: "`x -i --app [--local | --global] [&lt;name&gt;]`"
            },
            {
                type: "p",
                text: "Create a new app YAML file. Scope flags control where the file is written; without them, x prompts interactively."
            },
            {
                type: "h3",
                text: "`x --config`"
            },
            {
                type: "p",
                text: "Interactive configuration for default script language and LLM provider. See the Configuration section."
            },
            {
                type: "h3",
                text: "`x --ai` and `x -i --ai [name]`"
            },
            {
                type: "p",
                text: "LLM-powered command and script generation. Requires `~/.x.sh/config/llm.sh`. See AI Integration."
            }
        ]
    },
    {
        id: "global-scripts",
        title: "Global Scripts",
        subtitle: "Personal scripts available from any directory.",
        blocks: [
            {
                type: "p",
                text: "Global scripts are plain files in `~/.x.sh/scripts/`. Each script is executed with the program stored in its metadata (or the global default from `x --config`)."
            },
            {
                type: "h3",
                text: "Creating and editing"
            },
            {
                type: "p",
                text: "`x -i` is the primary workflow. x opens your `$EDITOR`, and on save, writes the file and metadata. &lt;br&gt; Re-running `x -i &lt;name&gt;` on an existing script lets you edit in place or rename."
            },
            {
                type: "h3",
                text: "Passing arguments"
            },
            {
                type: "code",
                code: `# Script: curl-google (bash)
curl -s "https://google.com$1"

$ x curl-google /search
# \u{2192} curl -s "https://google.com/search"`
            },
            {
                type: "h3",
                text: "Program / interpreter"
            },
            {
                type: "p",
                text: "The `program` field in metadata determines how x invokes the file \u2014 `bash`, `python`, `node`, `go run`, etc. Set per-script during `x -i` or rely on the global default."
            },
            {
                type: "h3",
                text: "Groups"
            },
            {
                type: "p",
                text: "Metadata `groups` are free-form tags for organization. They appear in `x --ls` output to help you scan large script collections."
            },
            {
                type: "h3",
                text: "Symlinks vs scripts"
            },
            {
                type: "p",
                text: "`x --ln` creates a symlink in `~/.local/bin/`, not a copy. The symlink points at the script file in `~/.x.sh/scripts/`, so edits via `x -i` are reflected immediately."
            }
        ]
    },
    {
        id: "project-local",
        title: "Project-local Scripts (`x.yml`)",
        subtitle: "Version-controlled task runners in the current project.",
        blocks: [
            {
                type: "p",
                text: "Place a file named `x.yml` in your **current working directory**. When you run `x &lt;name&gt;`, x loads it and looks for a top-level key matching `&lt;name&gt;`. If found, the entry runs; otherwise x falls back to global scripts."
            },
            {
                type: "callout",
                variant: "warn",
                text: "Only the CWD is checked for `x.yml` \u2014 not parent directories. Invalid YAML fails immediately; x does not fall back to global scripts on parse errors."
            },
            {
                type: "h3",
                text: "String entries (inline scripts)"
            },
            {
                type: "code",
                title: "x.yml",
                code: `lint: npm run lint

build: |
  VERSION="$1"
  npm run build
  npm version "$VERSION"
  git push --tags`
            },
            {
                type: "p",
                text: "Extra CLI arguments after the command name become `$1`, `$2`, \u2026 in the script body."
            },
            {
                type: "h3",
                text: "Mapping entries (nested subcommands)"
            },
            {
                type: "code",
                title: "Nested commands",
                code: `deploy:
  $: |
    echo "Deploying (default)"
    x deploy dev
  dev: npm run deploy:dev
  prod: npm run deploy:prod
  test:
    unit: npx jest unit
    integration: npx jest integration`
            },
            {
                type: "ul",
                items: [
                    "`$` \u2014 default handler when you run the parent with no further arguments (`x deploy`).",
                    "Any other key \u2014 nested subcommand; mappings can nest arbitrarily deep.",
                    "`x deploy prod` \u2192 `deploy.prod`",
                    "`x deploy test unit` \u2192 `deploy.test.unit`"
                ]
            },
            {
                type: "callout",
                variant: "info",
                text: "If a mapping has nested keys but no `$` handler, invoking the parent alone (e.g. `x deploy`) produces an error asking you to add `$` or pass a subcommand."
            },
            {
                type: "h3",
                text: "Execution details"
            },
            {
                type: "p",
                text: "Local scripts use your configured **default program** (`x --config` \u2192 `default_program` in `~/.x.sh/config.json`), typically `bash`. No per-entry metadata is supported."
            },
            {
                type: "h3",
                text: "Activity tracking"
            },
            {
                type: "p",
                text: "Runs from `x.yml` are **not** recorded in `x --ls` activity data \u2014 only global scripts are tracked."
            }
        ]
    },
    {
        id: "app-framework",
        title: "App Framework",
        subtitle: "YAML-defined, validated multi-command CLIs.",
        blocks: [
            {
                type: "p",
                text: "Apps turn a single `&lt;name&gt;.x.yml` file into a structured CLI with options, positional arguments, nested subcommands, and bash handlers. Unlike `x.yml` inline scripts, apps get **parse-time validation** \u2014 x checks required args, defaults, choice membership, `requires:` chains, and unknown flags before any handler bash executes."
            },
            {
                type: "h3",
                text: "Where apps live"
            },
            {
                type: "ul",
                items: [
                    "**Local:** `./&lt;name&gt;.x.yml` (discovered from CWD up through parents).",
                    "**Global:** `~/.x.sh/apps/&lt;name&gt;.x.yml`."
                ]
            },
            {
                type: "h3",
                text: "Creating an app"
            },
            {
                type: "code",
                code: `x -i --app my-app             # prompts for local vs global
x -i --app --local my-app     # ./my-app.x.yml
x -i --app --global my-app    # ~/.x.sh/apps/my-app.x.yml`
            },
            {
                type: "p",
                text: "After the editor closes, x validates the YAML structure, parses every synopsis string, and checks handler coverage. On failure you can **edit** (re-open) or **revert** (restore prior file or delete the new one)."
            },
            {
                type: "h3",
                text: "App vs project-local"
            },
            {
                type: "table",
                headers: [
                    "Feature",
                    "`x.yml`",
                    "App (`&lt;name&gt;.x.yml`)"
                ],
                rows: [
                    [
                        "Scope",
                        "CWD only",
                        "CWD + parent walk / global"
                    ],
                    [
                        "CLI parsing",
                        "None (raw args as $1\u2026)",
                        "Full synopsis DSL + validation"
                    ],
                    [
                        "Options / flags",
                        "No",
                        "Yes"
                    ],
                    [
                        "Help generation",
                        "No",
                        "Auto `-h` / `--help` per command"
                    ],
                    [
                        "Handlers",
                        "Inline YAML strings",
                        "Bash in `$:` block or `$.import`"
                    ],
                    [
                        "Metadata",
                        "None",
                        "name, version, description"
                    ]
                ]
            },
            {
                type: "h3",
                text: "Handler imports"
            },
            {
                type: "p",
                text: "Large apps can split handlers into external YAML files via `$.import`. This is mutually exclusive with an inline `$:` block."
            },
            {
                type: "code",
                code: `$.import:
  - ./handlers/build.yml
  - ./handlers/deploy.yml`
            }
        ]
    },
    {
        id: "app-format",
        title: "App File Format",
        subtitle: "Anatomy of a `&lt;name&gt;.x.yml` app definition.",
        blocks: [
            {
                type: "code",
                title: "Complete example",
                code: `# metadata
name: my-app
version: 0.0.0
description: an example app

# top-level options (apply to the root command)
options:
  - "[-v | --version]"

# nested commands; each may have its own options/arguments/commands
commands:
  build:
    description: build the project
    options:
      - "[--input=&lt;file&gt; [--output=&lt;dir&gt;]]"
      - "[--mode={fast|safe|deep}]"
    arguments:
      - "&lt;assets&gt;..."

  create:
    description: create things
    commands:
      file:
        description: create a file
        arguments: "&lt;path&gt; [&lt;content='empty'&gt;]"
      folder:
        arguments: "&lt;path&gt;"

# bash handlers, keyed by dotted command path
$:
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
  create: x-usage create        # default: print help when called bare
  create.file: |
    echo "creating $(x-arg path) with $(x-arg content)"
  create.folder: |
    mkdir -p "$(x-arg path)"`
            },
            {
                type: "h3",
                text: "Top-level fields"
            },
            {
                type: "ul",
                items: [
                    "`name`, `version`, `description` \u2014 metadata shown in help output.",
                    "`options` \u2014 flags available on the root command.",
                    "`arguments` \u2014 positional args on the root command.",
                    "`commands` \u2014 nested subcommand tree.",
                    "`$` or `$.import` \u2014 bash handler map (mutually exclusive)."
                ]
            },
            {
                type: "h3",
                text: "Command nodes"
            },
            {
                type: "p",
                text: "Each entry under `commands` can have `description`, `options`, `arguments`, nested `commands`, or any combination. Leaf commands need a matching handler key in `$` (e.g. `build` \u2192 `$: build:`)."
            },
            {
                type: "h3",
                text: "Non-leaf groups"
            },
            {
                type: "p",
                text: "A command with subcommands but no direct handler can use a one-liner like `create: x-usage create` to print help when invoked without a subcommand."
            },
            {
                type: "h3",
                text: "Imported handlers"
            },
            {
                type: "p",
                text: "External files referenced by `$.import` have the same flat `dotted.path: bash body` shape as the inline `$:` block."
            }
        ]
    },
    {
        id: "synopsis",
        title: "Synopsis DSL",
        subtitle: "The mini-language for declaring options and arguments.",
        blocks: [
            {
                type: "p",
                text: "Every string in `options` and `arguments` arrays is a **synopsis** that x parses into a structured CLI spec. All constraints are enforced before handlers run."
            },
            {
                type: "table",
                headers: [
                    "Synopsis",
                    "Meaning"
                ],
                rows: [
                    [
                        "`&lt;name&gt;`",
                        "Required positional"
                    ],
                    [
                        "`[&lt;name&gt;]`",
                        "Optional positional"
                    ],
                    [
                        "`[&lt;name='val'&gt;]`",
                        "Optional positional with default literal"
                    ],
                    [
                        "`&lt;name&gt;...` / `[&lt;name&gt;...]`",
                        "Repeating positional (greedy, must be last)"
                    ],
                    [
                        "`[-s | --long]`",
                        "Optional bool flag (alias pair)"
                    ],
                    [
                        "`[-s | --long &lt;arg&gt;]`",
                        "Flag with one required value"
                    ],
                    [
                        "`[-s | --long &lt;arg='v'&gt;]`",
                        "Flag value with default"
                    ],
                    [
                        "`[-s | --long &lt;arg&gt; ...]`",
                        "Flag value repeats"
                    ],
                    [
                        "`[--long={a|b|c}]`",
                        "Choice from set"
                    ],
                    [
                        "`[--long=&lt;arg='v'&gt;]`",
                        "`=` form with default value"
                    ],
                    [
                        "`[--input=&lt;a&gt; [--output=&lt;b&gt;]]`",
                        "Nesting = dependency (`--output` requires `--input`)"
                    ],
                    [
                        "`(a|b|c)`",
                        "Required choice (becomes positional `choice`)"
                    ],
                    [
                        "`--long`",
                        "Required option (bare, no brackets)"
                    ]
                ]
            },
            {
                type: "h3",
                text: "Validation guarantees"
            },
            {
                type: "ul",
                items: [
                    "Required arguments and options must be present.",
                    "Defaults are applied when optional values are omitted.",
                    "Choice values must be members of the declared set.",
                    "`requires:` chains from nested bracket groups are enforced.",
                    "Repeating positionals must be the last argument in the list.",
                    "Unknown options are rejected with a clear error."
                ]
            },
            {
                type: "h3",
                text: "YAML anchors"
            },
            {
                type: "p",
                text: "Standard YAML anchors and aliases work for sharing option sets across commands \u2014 see the [example app](https://github.com/michaelmunson/x.sh/blob/main/docs/examples/app/exapp.x.yml) for `&demo_verbose` / `*demo_verbose` usage."
            }
        ]
    },
    {
        id: "builtins",
        title: "Built-in Helpers",
        subtitle: "Bash functions injected before every app handler.",
        blocks: [
            {
                type: "p",
                text: "x prepends a small bash preamble to each handler so you can read parsed values without manual environment-variable wrangling. All helpers are available in the handler's scope and exported for `x-run` subprocesses."
            },
            {
                type: "table",
                headers: [
                    "Builtin",
                    "Behavior"
                ],
                rows: [
                    [
                        "`x-opt &lt;name&gt;`",
                        "Print option value. Bool flags \u2192 `true`. Repeats join with newline."
                    ],
                    [
                        "`x-arg &lt;name&gt;`",
                        "Print positional argument value."
                    ],
                    [
                        "`x-opts &lt;assoc-name&gt;`",
                        "Populate a caller-named bash associative array with all options."
                    ],
                    [
                        "`x-args &lt;assoc-name&gt;`",
                        "Populate a caller-named bash associative array with all arguments."
                    ],
                    [
                        "`x-run &lt;cmd&gt; [args\u2026]`",
                        "Run a command with `x-*` helpers still in scope."
                    ],
                    [
                        "`x-usage &lt;cmd-path&gt;`",
                        "Print auto-generated help (e.g. `x-usage create.file`)."
                    ],
                    [
                        "`x-io-read \u2026`",
                        "Prompt for a line. `-v` / `--var` assigns to a global scalar."
                    ],
                    [
                        "`x-io-confirm \u2026`",
                        "Yes/no prompt (`--default yes|no`). `-v` assigns `true`/`false`."
                    ],
                    [
                        "`x-io-select \u2026`",
                        "Menu of `id=label` pairs. `--multi` for multiple. `-v` assigns result."
                    ]
                ]
            },
            {
                type: "h3",
                text: "Reading options and arguments"
            },
            {
                type: "code",
                code: `echo "mode=$(x-opt mode)"
echo "path=$(x-arg path)"

declare -A opts
x-opts opts
echo "\${opts[mode]}"

declare -A args
x-args args
for k in "\${!args[@]}"; do echo "$k=\${args[$k]}"; done`
            },
            {
                type: "h3",
                text: "Repeating options"
            },
            {
                type: "code",
                code: `mapfile -t dirs &lt; &lt;(x-opt dir)
for d in "\${dirs[@]}"; do echo "dir: $d"; done`
            },
            {
                type: "h3",
                text: "Interactive I/O"
            },
            {
                type: "code",
                code: `x-io-read "Name:" -v name
echo "$name"

x-io-confirm "Proceed?" --default no -v ok
[[ "$ok" == "true" ]] && echo "go"

x-io-select -v color "Pick a color" "red=Red" "green=Green" "blue=Blue"
echo "$color"

x-io-select --multi -v colors "Colors" "r=Red" "g=Green"
echo "\${colors[0]}"`
            },
            {
                type: "callout",
                variant: "tip",
                text: "With `-v` / `--var`, results land in the caller's global shell variable \u2014 not stdout. Multi-select stores an indexed array."
            }
        ]
    },
    {
        id: "examples",
        title: "App Examples",
        subtitle: "Real-world references in the repository.",
        blocks: [
            {
                type: "h3",
                text: "Comprehensive example app (`exapp`)"
            },
            {
                type: "p",
                text: "The app definition imports handlers from two sidecar files:"
            },
            {
                type: "code",
                title: "exapp.x.yml",
                code: (0, _examples.EXAPP_X)
            },
            {
                type: "code",
                title: "exapp.handlers-a.yml",
                code: (0, _examples.EXAPP_HANDLERS_A)
            },
            {
                type: "code",
                title: "exapp.handlers-b.yml",
                code: (0, _examples.EXAPP_HANDLERS_B)
            },
            {
                type: "link",
                href: "https://github.com/michaelmunson/x.sh/tree/main/docs/examples/app",
                text: "View source on GitHub",
                external: true
            },
            {
                type: "h3",
                text: "Typical workflows"
            },
            {
                type: "ul",
                items: [
                    "**Dotfiles scripts** \u2014 `x -i` for personal utilities, `x --ln` for PATH access.",
                    "**Monorepo tasks** \u2014 root `x.yml` for `build`, `test`, `lint`; no global install needed.",
                    "**Team CLI tool** \u2014 commit `&lt;tool&gt;.x.yml` to the repo; teammates run `x tool subcmd` from any subdirectory.",
                    "**Complex internal CLI** \u2014 global app in `~/.x.sh/apps/` with `$.import` for large handler sets."
                ]
            }
        ]
    },
    {
        id: "validation-help",
        title: "Validation & Help",
        subtitle: "How x keeps apps correct and self-documenting.",
        blocks: [
            {
                type: "h3",
                text: "Automatic help"
            },
            {
                type: "p",
                text: "Every command supports `-h` / `--help` out of the box \u2014 no extra configuration."
            },
            {
                type: "code",
                code: `x my-app --help
x my-app build --help
x my-app create file --help`
            },
            {
                type: "p",
                text: "The same renderer powers `x-usage &lt;cmd-path&gt;` inside handlers, so you can print context-appropriate help from bash."
            },
            {
                type: "h3",
                text: "Save-time validation (`x -i --app`)"
            },
            {
                type: "p",
                text: "When you finish editing an app file, x performs a full structural check:"
            },
            {
                type: "ul",
                items: [
                    "YAML parses cleanly.",
                    "Every synopsis string parses into a valid spec.",
                    "Leaf commands have corresponding `$` handlers.",
                    "No orphan handler keys.",
                    "No duplicate options within a command.",
                    "No dangling `requires:` references.",
                    "`$` and `$.import` are not both present."
                ]
            },
            {
                type: "p",
                text: "On failure, x lists all errors and offers **edit** (re-open the editor) or **revert**."
            },
            {
                type: "h3",
                text: "Runtime validation"
            },
            {
                type: "p",
                text: "A lighter validation pass runs on every `x &lt;app&gt;` invocation so hand-edited or stale files fail fast with readable errors before any bash executes."
            }
        ]
    },
    {
        id: "configuration",
        title: "Configuration",
        subtitle: "Defaults, metadata, and persistent settings.",
        blocks: [
            {
                type: "h3",
                text: "`x --config`"
            },
            {
                type: "p",
                text: "Interactive menu with two options:"
            },
            {
                type: "h4",
                text: "Default Script Language"
            },
            {
                type: "p",
                text: "Sets `default_program` in `~/.x.sh/config.json`. Pre-selected during `x -i` and used for `x.yml` inline execution."
            },
            {
                type: "code",
                title: "config.json",
                code: `{
  "default_program": "bash"
}`
            },
            {
                type: "h4",
                text: "LLM Provider"
            },
            {
                type: "p",
                text: "Opens your editor with a template script saved to `~/.x.sh/config/llm.sh`. The script must accept the prompt as `$1` and print the completion to stdout."
            },
            {
                type: "code",
                title: "llm.sh example",
                code: `#!/bin/bash
PROMPT="$1"
ollama run llama2 "$PROMPT"`
            },
            {
                type: "h3",
                text: "Environment"
            },
            {
                type: "ul",
                items: [
                    "`$EDITOR` \u2014 used by `x -i`, `x -i --app`, and LLM config.",
                    "`$PATH` \u2014 must include `~/.local/bin` for `x --ln` symlinks."
                ]
            }
        ]
    },
    {
        id: "ai",
        title: "AI Integration",
        subtitle: "Generate commands and scripts with your LLM of choice.",
        blocks: [
            {
                type: "p",
                text: "x does not bundle an LLM. Instead, you provide a shell script that forwards prompts to any backend \u2014 Ollama, OpenAI CLI, a custom API wrapper, etc."
            },
            {
                type: "h3",
                text: "`x --ai`"
            },
            {
                type: "p",
                text: "Prompts for natural-language instructions, generates a shell command, displays it, and optionally executes after confirmation."
            },
            {
                type: "code",
                code: `x --ai
# instructions&gt; aws command to list parameters
#
# Generated Command
# aws ssm describe-parameters
# Run command? [y/N]`
            },
            {
                type: "h3",
                text: "`x -i --ai [name]`"
            },
            {
                type: "p",
                text: "Generates a complete script via LLM and saves it through the normal `x -i` workflow (metadata + file)."
            },
            {
                type: "callout",
                variant: "info",
                text: "Configure your provider first with `x --config` \u2192 LLM Provider. Without `~/.x.sh/config/llm.sh`, AI commands will fail."
            }
        ]
    },
    {
        id: "languages",
        title: "Supported Languages",
        subtitle: "Interpreters and compilers available during `x -i`.",
        blocks: [
            {
                type: "p",
                text: "Each language maps to a `program` value stored in script metadata. x invokes `&lt;program&gt; &lt;script-file&gt; [args\u2026]` for global scripts."
            },
            {
                type: "table",
                headers: [
                    "Language",
                    "Program",
                    "Notes"
                ],
                rows: [
                    [
                        "Bash",
                        "bash",
                        "Default for most shell scripts"
                    ],
                    [
                        "Zsh",
                        "zsh",
                        ""
                    ],
                    [
                        "POSIX Shell",
                        "sh",
                        ""
                    ],
                    [
                        "Node.js",
                        "node",
                        "JavaScript / TypeScript"
                    ],
                    [
                        "Python 3",
                        "python",
                        ""
                    ],
                    [
                        "Python 2",
                        "python2",
                        ""
                    ],
                    [
                        "Ruby",
                        "ruby",
                        ""
                    ],
                    [
                        "Perl",
                        "perl",
                        ""
                    ],
                    [
                        "Go",
                        "go",
                        "Compiled \u2014 use `go run` style in script"
                    ],
                    [
                        "Rust",
                        "rust",
                        ""
                    ],
                    [
                        "PHP",
                        "php",
                        ""
                    ],
                    [
                        "Lua",
                        "lua",
                        ""
                    ],
                    [
                        "Deno",
                        "deno",
                        "JS/TS runtime"
                    ],
                    [
                        "Swift",
                        "swift",
                        ""
                    ],
                    [
                        "C",
                        "c",
                        ""
                    ],
                    [
                        "C++",
                        "cpp",
                        ""
                    ],
                    [
                        "Java",
                        "java",
                        ""
                    ],
                    [
                        "R",
                        "r",
                        ""
                    ],
                    [
                        "AWK",
                        "awk",
                        ""
                    ],
                    [
                        "Elixir",
                        "elixir",
                        ""
                    ],
                    [
                        "Clojure",
                        "clj",
                        ""
                    ],
                    [
                        "Scala",
                        "scala",
                        ""
                    ],
                    [
                        "Haskell",
                        "haskell",
                        ""
                    ],
                    [
                        "PowerShell",
                        "powershell",
                        ""
                    ],
                    [
                        "Kotlin",
                        "kotlin",
                        ""
                    ]
                ]
            }
        ]
    },
    {
        id: "faq",
        title: "FAQ",
        subtitle: "Common questions and troubleshooting.",
        blocks: [
            {
                type: "h3",
                text: "Why does `x my-script` not find my `x.yml` entry?"
            },
            {
                type: "p",
                text: "`x.yml` is only read from the **current working directory**, not parent directories. `cd` to the directory containing `x.yml`, or use a global script / app instead."
            },
            {
                type: "h3",
                text: "Why does a local app work from a subdirectory but `x.yml` does not?"
            },
            {
                type: "p",
                text: "By design. Apps walk up the directory tree; `x.yml` does not. Put shared project commands in a root-level app or keep `x.yml` usage to single-directory workflows."
            },
            {
                type: "h3",
                text: "`x --ln` created a symlink but the command is not found"
            },
            {
                type: "p",
                text: "Ensure `~/.local/bin` is on your `PATH`. Run `echo $PATH` and verify. You may need to open a new shell after updating your rc file."
            },
            {
                type: "h3",
                text: "My app fails validation after editing"
            },
            {
                type: "p",
                text: "Run `x -i --app --local &lt;name&gt;` (or `--global`) to re-enter the edit/revert loop. Common issues: missing handler for a leaf command, invalid synopsis syntax, or both `$` and `$.import` present."
            },
            {
                type: "h3",
                text: "Can I use extensions in script names?"
            },
            {
                type: "p",
                text: "Script **names** you pass to `x` are extensionless (e.g. `x c-test`). The file on disk can include an extension (`c-test.c`); x resolves by basename match."
            },
            {
                type: "h3",
                text: "Does `x --delete` remove symlinks?"
            },
            {
                type: "p",
                text: "No. Use `x -d --ln &lt;name&gt;` to remove symlinks separately."
            },
            {
                type: "h3",
                text: "Where is the source code?"
            },
            {
                type: "link",
                href: "https://github.com/michaelmunson/x.sh",
                text: "github.com/michaelmunson/x.sh",
                external: true
            }
        ]
    }
];

},{"@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT","./examples":"8FCOW"}],"jnFvT":[function(require,module,exports,__globalThis) {
exports.interopDefault = function(a) {
    return a && a.__esModule ? a : {
        default: a
    };
};
exports.defineInteropFlag = function(a) {
    Object.defineProperty(a, '__esModule', {
        value: true
    });
};
exports.exportAll = function(source, dest) {
    Object.keys(source).forEach(function(key) {
        if (key === 'default' || key === '__esModule' || Object.prototype.hasOwnProperty.call(dest, key)) return;
        Object.defineProperty(dest, key, {
            enumerable: true,
            get: function() {
                return source[key];
            }
        });
    });
    return dest;
};
exports.export = function(dest, destName, get) {
    Object.defineProperty(dest, destName, {
        enumerable: true,
        get: get
    });
};

},{}],"8FCOW":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "EXAPP_X", ()=>EXAPP_X);
parcelHelpers.export(exports, "EXAPP_HANDLERS_A", ()=>EXAPP_HANDLERS_A);
parcelHelpers.export(exports, "EXAPP_HANDLERS_B", ()=>EXAPP_HANDLERS_B);
const EXAPP_X = `
# Comprehensive sample app for the \`x\` app framework (\`x exapp \u{2026}\`).
# Demonstrates: metadata, root options/arguments, nested commands, YAML anchors,
# every synopsis form from the cheat-sheet, and split \`$.import\` handler files.

name: exapp
version: 0.1.0
description: >-
  Exercise app \u{2014} exercises options, positionals, nesting, validation,
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
          Option forms \u{2014} bool pair, value, defaulted value, repeating values,
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
          Positional forms \u{2014} required, optional, optional-with-default,
          and greedy repeat (must be last).
        arguments:
          - "<one> [<two>] [<three='3'>] [<rest>...]"

      pick:
        description: required choice \u{2014} \`(alt1|alt2|alt3)\` becomes positional name \`choice\`
        arguments:
          - "(north|south|east|west)"

      tail:
        description: optional repeating positional \`[<items>...]\`
        arguments:
          - "[<items>...]"

      group:
        description: >-
          Non-leaf group \u{2014} invoke \`x exapp demo group\` with no subcommand to see auto-help.
        commands:
          alpha:
            description: nested leaf \u{3B1} \u{2014} shares options via YAML anchor (see \`reflect\`)
            options: &demo_verbose
              - "[-V | --verbose]"
            arguments:
              - "<msg>"
          beta:
            description: nested leaf \u{3B2} \u{2014} same shared options as \`alpha\`
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
          One-line handler \u{2014} forwards to x-usage (same idea as README create \u{2192} x-usage create).

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
const EXAPP_HANDLERS_A = `
# Handlers merged via \`$.import\` (first file). Keys are dotted command paths.

"":
  |
    topic=$(x-arg topic)
    note=$(x-opt global)
    echo "exapp \u{2014} topic=\${topic:-overview}  global-note=\${note:-\u{2205}}"
    echo "Try:  x exapp --help"
    echo "       x exapp demo --help"

demo.opts:
  |
    echo "dry-run=$(x-opt dry-run)"
    echo "out=$(x-opt out)"
    echo "count=$(x-opt count)"
    mapfile -t defs < <(x-opt define)
    printf 'define[%s]
' "\${defs[@]:-}"
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
    printf 'rest[%s]
' "\${rest[@]:-}"

demo.pick:
  |
    echo "direction=$(x-arg choice)"

demo.tail:
  |
    mapfile -t items < <(x-arg items)
    if ((\${#items[@]})); then
      printf 'items: %s
' "\${items[@]}"
    else
      echo "(no items)"
    fi

demo.sparse:
  |
    mapfile -t parts < <(x-arg parts)
    echo "parts=\${parts[*]}"

demo.usage-tip: x-usage demo.opts
`;
const EXAPP_HANDLERS_B = `
# Second import file \u{2014} merged keys must not duplicate the first import.

demo.group.alpha:
  |
    msg=$(x-arg msg)
    if [[ $(x-opt verbose) == true ]]; then
      echo "[verbose] alpha \u{2190} $msg"
    else
      echo "alpha \u{2190} $msg"
    fi

demo.group.beta:
  |
    msg=$(x-arg msg)
    if [[ $(x-opt verbose) == true ]]; then
      echo "[verbose] beta \u{2190} $msg"
    else
      echo "beta \u{2190} $msg"
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

},{"@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"ip12w":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "renderInline", ()=>(0, _inline.renderInline));
parcelHelpers.export(exports, "CodeBlock", ()=>(0, _codeBlock.CodeBlock));
parcelHelpers.export(exports, "Callout", ()=>(0, _callout.Callout));
parcelHelpers.export(exports, "DataTable", ()=>(0, _dataTable.DataTable));
parcelHelpers.export(exports, "Paragraph", ()=>(0, _paragraph.Paragraph));
parcelHelpers.export(exports, "Heading", ()=>(0, _heading.Heading));
parcelHelpers.export(exports, "ResolutionPipeline", ()=>(0, _resolutionPipeline.ResolutionPipeline));
parcelHelpers.export(exports, "renderBlock", ()=>(0, _render.renderBlock));
parcelHelpers.export(exports, "renderBlocks", ()=>(0, _render.renderBlocks));
var _inline = require("./inline");
var _codeBlock = require("./CodeBlock");
var _callout = require("./Callout");
var _dataTable = require("./DataTable");
var _paragraph = require("./Paragraph");
var _heading = require("./Heading");
var _resolutionPipeline = require("./ResolutionPipeline");
var _render = require("./render");

},{"./inline":"7qopj","./CodeBlock":"9jFRS","./Callout":"km6ax","./DataTable":"6YT4j","./Paragraph":"kodbe","./Heading":"3cLgy","./ResolutionPipeline":"1mf7y","./render":"6Az6e","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"7qopj":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "renderInline", ()=>renderInline);
var _oxidizer = require("oxidizer");
function renderInline(text) {
    const parts = [];
    const regex = /`([^`]+)`|\*\*([^*]+)\*\*|\[([^\]]+)\]\(([^)]+)\)/g;
    let last = 0;
    let match;
    while((match = regex.exec(text)) !== null){
        if (match.index > last) parts.push(text.slice(last, match.index));
        if (match[1]) parts.push((0, _oxidizer.CODE)({
            className: "inline-code"
        }, match[1]));
        else if (match[2]) parts.push((0, _oxidizer.B)({}, match[2]));
        else if (match[3] && match[4]) parts.push((0, _oxidizer.A)({
            href: match[4],
            className: "inline-link"
        }, match[3]));
        last = match.index + match[0].length;
    }
    if (last < text.length) parts.push(text.slice(last));
    return parts.length ? parts : [
        text
    ];
}

},{"oxidizer":"hnuU9","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"9jFRS":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "CodeBlock", ()=>CodeBlock);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
function CodeBlock(code, title) {
    return (0, _oxidizer.DIV)({
        className: "code-block"
    }, title ? (0, _oxidizer.DIV)({
        className: "code-block-header"
    }, (0, _oxidizer.SPAN)({
        className: "code-block-title"
    }, ...(0, _inline.renderInline)(title)), (0, _oxidizer.BUTTON)({
        className: "copy-btn",
        type: "button",
        onclick (e) {
            const btn = e.currentTarget;
            navigator.clipboard.writeText(code).then(()=>{
                const prev = btn.textContent;
                btn.textContent = "Copied!";
                setTimeout(()=>{
                    btn.textContent = prev;
                }, 1500);
            });
        }
    }, "Copy")) : (0, _oxidizer.BUTTON)({
        className: "copy-btn copy-btn-float",
        type: "button",
        onclick (e) {
            const btn = e.currentTarget;
            navigator.clipboard.writeText(code).then(()=>{
                const prev = btn.textContent;
                btn.textContent = "Copied!";
                setTimeout(()=>{
                    btn.textContent = prev;
                }, 1500);
            });
        }
    }, "Copy"), (0, _oxidizer.PRE)({
        className: "code-pre"
    }, (0, _oxidizer.CODE)({
        className: "code-content"
    }, code)));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"km6ax":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "Callout", ()=>Callout);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
function Callout(variant, text, title) {
    const labels = {
        info: "Note",
        warn: "Warning",
        tip: "Tip"
    };
    return (0, _oxidizer.DIV)({
        className: `callout callout-${variant}`
    }, (0, _oxidizer.B)({}, title ?? labels[variant]), (0, _oxidizer.P)({
        className: "prose-p"
    }, ...(0, _inline.renderInline)(text)));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"6YT4j":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "DataTable", ()=>DataTable);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
function DataTable(headers, rows) {
    return (0, _oxidizer.DIV)({
        className: "table-wrap"
    }, (0, _oxidizer.TABLE)({
        className: "data-table"
    }, (0, _oxidizer.THEAD)({}, (0, _oxidizer.TR)({}, ...headers.map((h)=>(0, _oxidizer.TH)({}, ...(0, _inline.renderInline)(h))))), (0, _oxidizer.TBODY)({}, ...rows.map((row)=>(0, _oxidizer.TR)({}, ...row.map((cell)=>(0, _oxidizer.TD)({}, ...(0, _inline.renderInline)(cell))))))));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"kodbe":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "Paragraph", ()=>Paragraph);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
function Paragraph(text) {
    return (0, _oxidizer.P)({
        className: "prose-p"
    }, ...(0, _inline.renderInline)(text));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"3cLgy":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "Heading", ()=>Heading);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
function Heading(level, text) {
    const cls = level === 2 ? "section-h2" : level === 3 ? "section-h3" : "section-h4";
    const Tag = level === 2 ? (0, _oxidizer.H2) : level === 3 ? (0, _oxidizer.H3) : (0, _oxidizer.H4);
    return Tag({
        className: cls
    }, ...(0, _inline.renderInline)(text));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"1mf7y":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "ResolutionPipeline", ()=>ResolutionPipeline);
var _oxidizer = require("oxidizer");
function FlowStep(source, sourceNote, action, actionNote) {
    return (0, _oxidizer.DIV)({
        className: "flow-step"
    }, (0, _oxidizer.DIV)({
        className: "flow-step-row"
    }, (0, _oxidizer.DIV)({
        className: "flow-box flow-source"
    }, (0, _oxidizer.CODE)({
        className: "flow-box-code"
    }, source), sourceNote ? (0, _oxidizer.SPAN)({
        className: "flow-box-note"
    }, sourceNote) : null), (0, _oxidizer.DIV)({
        className: "flow-connector"
    }, (0, _oxidizer.SPAN)({
        className: "flow-connector-label"
    }, "match?"), (0, _oxidizer.SPAN)({
        className: "flow-connector-yes"
    }, "yes \u2192")), (0, _oxidizer.DIV)({
        className: "flow-box flow-action"
    }, (0, _oxidizer.SPAN)({
        className: "flow-box-text"
    }, action), actionNote ? (0, _oxidizer.SPAN)({
        className: "flow-box-note"
    }, actionNote) : null)), (0, _oxidizer.DIV)({
        className: "flow-fallback"
    }, (0, _oxidizer.SPAN)({
        className: "flow-fallback-label"
    }, "no"), (0, _oxidizer.SPAN)({
        className: "flow-arrow-down"
    }, "\u2193")));
}
function ResolutionPipeline() {
    return (0, _oxidizer.DIV)({
        className: "flow-diagram"
    }, (0, _oxidizer.P)({
        className: "flow-diagram-title"
    }, (0, _oxidizer.CODE)({
        className: "inline-code"
    }, "x my-cmd arg1 arg2")), (0, _oxidizer.DIV)({
        className: "flow-entry"
    }, (0, _oxidizer.SPAN)({
        className: "flow-arrow-down"
    }, "\u2193")), FlowStep("./x.yml", null, "run inline bash", null), FlowStep("<name>.x.yml", "CWD \u2192 parents", "parse & validate", "run $: handler"), FlowStep("~/.x.sh/apps/", null, "same as local app", null), FlowStep("~/.x.sh/scripts/", null, "run with program", "from metadata"), (0, _oxidizer.DIV)({
        className: "flow-step flow-step-terminal"
    }, (0, _oxidizer.DIV)({
        className: "flow-fallback flow-fallback-terminal"
    }, (0, _oxidizer.SPAN)({
        className: "flow-fallback-label"
    }, "no"), (0, _oxidizer.SPAN)({
        className: "flow-arrow-down"
    }, "\u2193")), (0, _oxidizer.DIV)({
        className: "flow-box flow-error"
    }, (0, _oxidizer.SPAN)({
        className: "flow-box-text"
    }, "error: command not found"))));
}

},{"oxidizer":"hnuU9","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"6Az6e":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "renderBlock", ()=>renderBlock);
parcelHelpers.export(exports, "renderBlocks", ()=>renderBlocks);
var _oxidizer = require("oxidizer");
var _inline = require("./inline");
var _codeBlock = require("./CodeBlock");
var _callout = require("./Callout");
var _dataTable = require("./DataTable");
var _paragraph = require("./Paragraph");
var _heading = require("./Heading");
var _resolutionPipeline = require("./ResolutionPipeline");
function renderDiagram(variant) {
    switch(variant.variant){
        case "resolution-pipeline":
            return (0, _resolutionPipeline.ResolutionPipeline)();
    }
}
function renderBlock(block) {
    switch(block.type){
        case "p":
            return (0, _paragraph.Paragraph)(block.text);
        case "h3":
            return (0, _heading.Heading)(3, block.text);
        case "h4":
            return (0, _heading.Heading)(4, block.text);
        case "code":
            return (0, _codeBlock.CodeBlock)(block.code, block.title);
        case "ul":
            return (0, _oxidizer.UL)({
                className: "prose-ul"
            }, ...block.items.map((item)=>(0, _oxidizer.LI)({}, ...(0, _inline.renderInline)(item))));
        case "ol":
            return (0, _oxidizer.OL)({
                className: "prose-ol"
            }, ...block.items.map((item)=>(0, _oxidizer.LI)({}, ...(0, _inline.renderInline)(item))));
        case "table":
            return (0, _dataTable.DataTable)(block.headers, block.rows);
        case "callout":
            return (0, _callout.Callout)(block.variant, block.text, block.title);
        case "hr":
            return (0, _oxidizer.HR)({
                className: "section-hr"
            });
        case "link":
            return (0, _oxidizer.P)({
                className: "prose-p"
            }, (0, _oxidizer.A)({
                href: block.href,
                className: "inline-link",
                ...block.external ? {
                    target: "_blank",
                    rel: "noopener noreferrer"
                } : {}
            }, block.text));
        case "diagram":
            return renderDiagram(block);
    }
}
function renderBlocks(blocks) {
    return blocks.map((b)=>renderBlock(b));
}

},{"oxidizer":"hnuU9","./inline":"7qopj","./CodeBlock":"9jFRS","./Callout":"km6ax","./DataTable":"6YT4j","./Paragraph":"kodbe","./Heading":"3cLgy","./ResolutionPipeline":"1mf7y","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"e9AAl":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "default", ()=>HomePage);
var _oxidizer = require("oxidizer");
var _content = require("../content");
var _components = require("../components");
var _sidebar = require("../layout/Sidebar");
function HomePage(shell) {
    console.log('HomePage', shell.path);
    const groups = [
        ...new Set((0, _content.NAV_ITEMS).map((n)=>n.group ?? ""))
    ];
    return (0, _oxidizer.DIV)({
        className: "page home-page"
    }, (0, _oxidizer.DIV)({
        className: "hero"
    }, (0, _oxidizer.H1)({
        className: "hero-title"
    }, (0, _oxidizer.SPAN)({
        className: "hero-x"
    }, "x"), ".sh"), (0, _oxidizer.P)({
        className: "hero-tagline"
    }, "Script management and CLI App Framework."), (0, _oxidizer.DIV)({
        className: "hero-actions"
    }, (0, _oxidizer.BUTTON)({
        className: "btn btn-primary",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)("/docs/quickstart", shell)
    }, "Get Started"), (0, _oxidizer.BUTTON)({
        className: "btn btn-secondary",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)("/docs/commands", shell)
    }, "Command Reference"), (0, _oxidizer.A)({
        href: "https://github.com/michaelmunson/x.sh",
        className: "btn btn-ghost",
        target: "_blank",
        rel: "noopener noreferrer"
    }, "View Source")), (0, _oxidizer.DIV)({
        className: "hero-install"
    }, (0, _oxidizer.SPAN)({
        className: "hero-install-label"
    }, "Install"), (0, _oxidizer.CODE)({
        className: "hero-install-code"
    }, "curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash"))), (0, _oxidizer.DIV)({
        className: "home-sections"
    }, (0, _oxidizer.H2)({
        className: "home-sections-title"
    }, "Browse by topic"), ...groups.map((group)=>(0, _oxidizer.DIV)({
            className: "section-card-group"
        }, (0, _oxidizer.SPAN)({
            className: "section-card-group-label"
        }, group), (0, _oxidizer.DIV)({
            className: "section-cards"
        }, ...(0, _content.NAV_ITEMS).filter((item)=>item.group === group).map((item)=>(0, _oxidizer.BUTTON)({
                className: "section-card",
                type: "button",
                onclick: ()=>(0, _sidebar.goTo)(`/docs/${item.id}`, shell)
            }, (0, _oxidizer.SPAN)({
                className: "section-card-title"
            }, ...(0, _components.renderInline)(item.label)), (0, _oxidizer.SPAN)({
                className: "section-card-arrow"
            }, "\u2192"))))))));
}

},{"oxidizer":"hnuU9","../content":"hqNCP","../components":"ip12w","../layout/Sidebar":"hNbmf","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"3AN0D":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "default", ()=>SectionPage);
var _oxidizer = require("oxidizer");
var _oxidizerRouter = require("oxidizer-router");
var _content = require("../content");
var _components = require("../components");
var _sidebar = require("../layout/Sidebar");
var _notFoundPage = require("./NotFoundPage");
var _notFoundPageDefault = parcelHelpers.interopDefault(_notFoundPage);
function SectionPage(shell) {
    console.log('SectionPage', shell.path);
    const { sectionId } = (0, _oxidizerRouter.getParams)();
    const section = (0, _content.SECTIONS).find((s)=>s.id === sectionId);
    if (!section) return (0, _notFoundPageDefault.default)(shell);
    const idx = (0, _content.SECTIONS).findIndex((s)=>s.id === sectionId);
    const prev = (0, _content.SECTIONS)[idx - 1];
    const next = (0, _content.SECTIONS)[idx + 1];
    return (0, _oxidizer.MAIN)({
        className: "page doc-page"
    }, (0, _oxidizer.DIV)({
        className: "page-header"
    }, (0, _oxidizer.DIV)({
        className: "breadcrumb"
    }, (0, _oxidizer.BUTTON)({
        className: "breadcrumb-link",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)("/", shell)
    }, "Home"), (0, _oxidizer.SPAN)({
        className: "breadcrumb-sep"
    }, "/"), (0, _oxidizer.SPAN)({
        className: "breadcrumb-current"
    }, section.title)), (0, _oxidizer.H1)({
        className: "page-title"
    }, ...(0, _components.renderInline)(section.title)), section.subtitle ? (0, _oxidizer.P)({
        className: "page-subtitle"
    }, section.subtitle) : null), (0, _oxidizer.DIV)({
        className: "page-body"
    }, ...(0, _components.renderBlocks)(section.blocks)), (0, _oxidizer.DIV)({
        className: "page-nav"
    }, prev ? (0, _oxidizer.BUTTON)({
        className: "page-nav-btn page-nav-prev",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)(`/docs/${prev.id}`, shell)
    }, (0, _oxidizer.SPAN)({
        className: "page-nav-label"
    }, "Previous"), (0, _oxidizer.SPAN)({
        className: "page-nav-title"
    }, prev.title)) : (0, _oxidizer.SPAN)({
        className: "page-nav-spacer"
    }), next ? (0, _oxidizer.BUTTON)({
        className: "page-nav-btn page-nav-next",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)(`/docs/${next.id}`, shell)
    }, (0, _oxidizer.SPAN)({
        className: "page-nav-label"
    }, "Next"), (0, _oxidizer.SPAN)({
        className: "page-nav-title"
    }, next.title)) : (0, _oxidizer.SPAN)({
        className: "page-nav-spacer"
    })));
}

},{"oxidizer":"hnuU9","oxidizer-router":"6kk03","../content":"hqNCP","../components":"ip12w","../layout/Sidebar":"hNbmf","./NotFoundPage":"b6FlI","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}],"b6FlI":[function(require,module,exports,__globalThis) {
var parcelHelpers = require("@parcel/transformer-js/src/esmodule-helpers.js");
parcelHelpers.defineInteropFlag(exports);
parcelHelpers.export(exports, "default", ()=>NotFoundPage);
var _oxidizer = require("oxidizer");
var _sidebar = require("../layout/Sidebar");
function NotFoundPage(shell) {
    return (0, _oxidizer.MAIN)({
        className: "page not-found-page"
    }, (0, _oxidizer.DIV)({
        className: "not-found"
    }, (0, _oxidizer.H1)({
        className: "not-found-code"
    }, "404"), (0, _oxidizer.P)({
        className: "not-found-msg"
    }, "This documentation page doesn't exist."), (0, _oxidizer.BUTTON)({
        className: "btn btn-primary",
        type: "button",
        onclick: ()=>(0, _sidebar.goTo)("/", shell)
    }, "Back to Home")));
}

},{"oxidizer":"hnuU9","../layout/Sidebar":"hNbmf","@parcel/transformer-js/src/esmodule-helpers.js":"jnFvT"}]},["3dtlh","gH3Lb"], "gH3Lb", "parcelRequire56c8", {})

//# sourceMappingURL=pages.34df32e0.js.map
