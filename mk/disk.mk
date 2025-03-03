# Asurada 镜像配置文件

$(BUILD)/harddrive.img: $(INSTALLER) $(FSTOOL_TAG) $(REPO_TAG)
		mkdir -p $(BUILD)
		rm -rf $@ $@.partial
		-$(FUMOUNT) /tmp/asurada_installer || true
		FILESYSTEM_SIZE=$(FILESYSTEM_SIZE) && \
		if [ -z "$$FILESYSTEM_SIZE" ]; then \
			FILESYSTEM_SIZE=$(shell $(INSTALLER) --filesystem-size -c $(FILESYSTEM_CONFIG)); \
	  	fi && \
	  		truncate -s "$$FILESYSTEM_SIZE"m $@.partial
	  		umask 002 && $(INSTALLER) $(INSTALLER_OPTS) -c $(FILESYSTEM_CONFIG) $@.partial
	  		mv $@partial $@