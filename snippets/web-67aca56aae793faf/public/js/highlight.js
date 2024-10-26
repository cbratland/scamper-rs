import hljs from "https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/highlight.min.js";
import scheme from "https://unpkg.com/@highlightjs/cdn-assets@11.9.0/es/languages/scheme.min.js";

hljs.registerLanguage("scheme", scheme);

export function highlightElement(element) {
	hljs.highlightElement(element);
}
