dev:
	cargo tauri dev

build:
	cargo tauri build

css:
	npx tailwindcss -i ./tailwind.css -o ./styles.css --watch