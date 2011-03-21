generate-js:
	coffee -c -o lib src

remove-js:
	@rm -rf lib/

publish: generate-js
	npm publish
	@remove-js

install: generate-js
	npm install
	@remove-js

.PHONY: all

