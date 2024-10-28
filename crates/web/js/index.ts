import { basicSetup } from "codemirror";
import { indentWithTab } from "@codemirror/commands";
import { EditorView, keymap } from "@codemirror/view";
import { ScamperSupport } from "./codemirror/language.js";
import Split from "split.js";

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

export function createSplit(elements: HTMLElement[], sizes: number[]): void {
	Split(elements, {
		sizes: sizes,
	});
}
