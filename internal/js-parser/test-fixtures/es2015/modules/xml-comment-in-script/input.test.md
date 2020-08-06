# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > modules > xml-comment-in-script`

### `ast`

```javascript
JSRoot {
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "es2015/modules/xml-comment-in-script/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "es2015/modules/xml-comment-in-script/input.js"
		end: Object {
			column: 0
			line: 2
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	comments: Array [
		CommentLine {
			id: "0"
			value: "bar"
			loc: Object {
				filename: "es2015/modules/xml-comment-in-script/input.js"
				end: Object {
					column: 11
					line: 1
				}
				start: Object {
					column: 4
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			trailingComments: Array ["0"]
			loc: Object {
				filename: "es2015/modules/xml-comment-in-script/input.js"
				end: Object {
					column: 3
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSReferenceIdentifier {
				name: "foo"
				trailingComments: undefined
				loc: Object {
					filename: "es2015/modules/xml-comment-in-script/input.js"
					identifierName: "foo"
					end: Object {
						column: 3
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
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