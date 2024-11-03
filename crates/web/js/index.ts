// js wrapper functions that are exposed to wasm

import { basicSetup } from "codemirror";
import { indentWithTab } from "@codemirror/commands";
import { EditorView, keymap } from "@codemirror/view";
import { linter } from "@codemirror/lint";
import type { Diagnostic } from "@codemirror/lint";
import { ScamperSupport } from "./codemirror/language.js";
import Split from "split.js";
import { initPlayer, getPlayer, Player } from "./webaudiofont/webaudiofont.js";

declare module "@codemirror/view" {
	interface EditorView {
		get_doc(): string;
		set_doc(content: string): void;
	}
}

export function createEditor(
	doc: string,
	parent: HTMLElement,
	onUpdate: (view: EditorView) => void,
	onLinting: (view: EditorView) => Diagnostic[],
): EditorView {
	// const debounce = debouncer({ onUpdate, onUpdating });
	const update = EditorView.updateListener.of((update) => {
		if (update.docChanged) {
			onUpdate(update.view);
		}
	});

	const view = new EditorView({
		doc,
		parent,
		extensions: [
			basicSetup,
			keymap.of([indentWithTab]),
			ScamperSupport(),
			linter((view) => onLinting(view)),
			update,
		],
	});

	view.get_doc = function (this: EditorView): string {
		return this.state.doc.toString();
	};

	view.set_doc = function (this: EditorView, content: string): void {
		this.dispatch({
			changes: {
				from: 0,
				to: this.state.doc.length,
				insert: content,
			},
		});
	};

	return view;
}

export function createDiagnostic(
	from: number,
	to: number,
	severity: "error" | "warning" | "info",
	message: string,
): Diagnostic {
	return {
		from,
		to,
		severity: severity,
		message: message,
	};
}

export function createSplit(elements: HTMLElement[], sizes: number[]): void {
	Split(elements, {
		sizes: sizes,
	});
}

// todo: why does re-exporting the functions not work?
export function initPlayer2() {
	initPlayer();
}

export function getPlayer2(): Player {
	return getPlayer();
}
