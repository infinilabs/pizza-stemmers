
define compile_algorithm
	snowball ./algorithms/$(1).sbl -rust -o ./src/snowball/algorithms/$(1)

	if sed --version 2>/dev/null | grep -q GNU; then \
		sed -i "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	else \
		sed -i "" "s/use snowball::SnowballEnv;/use super::super::snowball_env::SnowballEnv;/g" ./src/snowball/algorithms/$(1).rs; \
		sed -i "" "s/use snowball::Among;/use super::super::among::Among;/g" ./src/snowball/algorithms/$(1).rs; \
	fi

	echo "pub mod $(1);" >> ./src/snowball/algorithms/mod.rs
endef

.PHONY: default algorithms

default:

algorithms:
	rm -rf ./src/snowball/algorithms
	mkdir ./src/snowball/algorithms

	$(call compile_algorithm,arabic)
	$(call compile_algorithm,armenian_mkrtchyan)
	$(call compile_algorithm,basque)
	$(call compile_algorithm,catalan)
	$(call compile_algorithm,czech_dolamic_aggressive)
	$(call compile_algorithm,czech_dolamic_light)
	$(call compile_algorithm,danish)
	$(call compile_algorithm,dutch)
	$(call compile_algorithm,english_lovins)
	$(call compile_algorithm,english_porter)
	$(call compile_algorithm,english_porter_2)
	$(call compile_algorithm,estonian_freienthal)
	$(call compile_algorithm,finnish)
	$(call compile_algorithm,french)
	$(call compile_algorithm,german)
	$(call compile_algorithm,greek)
	$(call compile_algorithm,hindi_lightweight)
	$(call compile_algorithm,hungarian)
	$(call compile_algorithm,indonesian_tala)
	$(call compile_algorithm,irish_gaelic)
	$(call compile_algorithm,italian)
	$(call compile_algorithm,lithuanian_jocas)
	$(call compile_algorithm,norwegian_bokmal)
	$(call compile_algorithm,portuguese)
	$(call compile_algorithm,romanian_heidelberg)
	$(call compile_algorithm,romanian_tirdea)
	$(call compile_algorithm,romanian)
	$(call compile_algorithm,russian)
	$(call compile_algorithm,spanish)
	$(call compile_algorithm,swedish)
	$(call compile_algorithm,turkish_cilden)
	$(call compile_algorithm,yiddish_urieli)
