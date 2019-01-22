
.PHONY: extract_cci clean

PHEONIX_VERISON := 5.12.1
CCI_PREFIX := http://devsite.ctr-electronics.com/maven/release/com/ctre/phoenix/cci/$(PHEONIX_VERISON)/cci-$(PHEONIX_VERISON)
CORE_PREFIX := http://devsite.ctr-electronics.com/maven/release/com/ctre/phoenix/core/$(PHEONIX_VERISON)/core-$(PHEONIX_VERISON)


generate: extract_cci
	cd sys-gen; cargo run;

extract_cci:
	mkdir -p tmp
	mkdir -p include
	mkdir -p lib/athena
	cd tmp; wget $(CCI_PREFIX)-headers.zip
	unzip -o tmp/cci-$(PHEONIX_VERISON)-headers.zip -d include

	cd tmp; wget $(CCI_PREFIX)-linuxathenastatic.zip
	unzip -o -j tmp/cci-$(PHEONIX_VERISON)-linuxathenastatic.zip 'linux/athena/static/libCTRE_PhoenixCCI.a' -d lib/athena

	cd tmp; wget $(CORE_PREFIX)-headers.zip
	unzip -o tmp/core-$(PHEONIX_VERISON)-headers.zip -d include

clean:
	rm -r tmp/*
	rm -r include/*
	rm -r lib/*
