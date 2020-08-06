# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > estree-compat > shorthand-ambient-module`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "typescript/estree-compat/shorthand-ambient-module/input.ts"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array ["ts"]
	loc: Object {
		filename: "typescript/estree-compat/shorthand-ambient-module/input.ts"
		end: Object {
			column: 32
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		TSModuleDeclaration {
			id: JSStringLiteral {
				value: "hot-new-module"
				loc: Object {
					filename: "typescript/estree-compat/shorthand-ambient-module/input.ts"
					end: Object {
						column: 31
						line: 1
					}
					start: Object {
						column: 15
						line: 1
					}
				}
			}
			body: undefined
			declare: true
			global: undefined
			loc: Object {
				filename: "typescript/estree-compat/shorthand-ambient-module/input.ts"
				end: Object {
					column: 32
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```