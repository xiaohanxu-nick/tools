# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/transforms/compile/index.test.ts --update-snapshots` to update.

## `css-handler > prefix > appearance`

### `Diagnostics`

```

```

### `Input`

```css
.style {
	appearance: none;
}

.style {
	appearance: auto;
}

.style {
	appearance: button;
}

```

### `Output`

```css
.style {
	-webkit-appearance: none;
	-moz-appearance: none;
	appearance: none;
}

.style {
	-webkit-appearance: auto;
	-moz-appearance: auto;
	appearance: auto;
}

.style {
	-webkit-appearance: button;
	-moz-appearance: button;
	appearance: button;
}

```