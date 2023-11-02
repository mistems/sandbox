init:

build:
	mkdir -p .www/ikasoba/ddsk
	deno run -A build.js
	cp dist/* $(repository_root)/.www/ikasoba/ddsk
	cp src/index.html $(repository_root)/.www/ikasoba/ddsk