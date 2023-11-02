init:

build:
	@ mkdir -p $(repository_root)/.www/ikasoba/preact-example
	deno run -A build.js
	@ cp .dist/* $(repository_root)/.www/ikasoba/preact-example
	@ cp src/index.html $(repository_root)/.www/ikasoba/preact-example