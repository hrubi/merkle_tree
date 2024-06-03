perf-data: fixtures/input-perf.txt

fixtures/input-perf.txt: record_count = 100000
fixtures/input-perf.txt:
	for i in $$(seq $(record_count)); do \
		cat /dev/urandom | tr -dc 'a-f0-9' | fold -w $${1:-64} | head -n 1 >> "$@"; \
	done
