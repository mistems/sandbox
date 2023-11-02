init:
	asdf plugin add deno https://github.com/asdf-community/asdf-deno.git
	asdf install deno latest
	asdf global deno latest

build:
	mkdir -p .www/ikasoba/ddsk
	deno run -A build.js
	cp dist/* $(repository_root)/.www/ikasoba/ddsk
	cp src/index.html $(repository_root)/.www/ikasoba/ddsk