#!/bin/bash

#simple util
case $1 in
	push)
		git add .
		git commit -m "Updated"
		git push origin master
		;;
	get)
		cd pkg
		rm * .*
		read TOKEN
		curl -H "Circle-Token: $TOKEN" https://circleci.com/api/v1.1/project/github/rev1912/wasm_module/latest/artifacts \
		| grep -o 'https://[^"]*' \
		| wget --verbose --header "Circle-Token: $TOKEN" --input-file -
		cd ..
		;;
	parse)
		./wabt/bin/wasm2wat pkg/*.wasm -o wasm_module.wat
		;;
esac