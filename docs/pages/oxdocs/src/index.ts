import "./index.css";
import "./prism";
import App from "./App";
import { iconSvgBase64 } from "./assets";

const favicon = document.querySelector<HTMLLinkElement>('link[rel="icon"]');
console.log('favicon', favicon);
if (favicon) favicon.src = iconSvgBase64;

document.body.append(App());
