
SRC_DIR ?= ./src
OBJ_DIR ?= ./build
LIB_DIR ?= ./lib

SRC = $(wildcard $(SRC_DIR)/*.c)
OBJ = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRC))

VPATH = $(SRC_DIR)
vpath %.o $(OBJ_DIR)

###########################################################

COMPILER_LANGUAGE ?= c
CC ?= gcc
CONFIG ?= config.h
TARGETPLATFORM := $(shell LANG=en_US.UTF-8 $(CC) -v 2>&1 | grep '^Target: ' | cut -f 2 -d ' ')

BASECFLAGS = -DVLMCSD_COMPILER=\"$(notdir $(CC))\" -DVLMCSD_PLATFORM=\"$(TARGETPLATFORM)\" -DCONFIG=\"$(CONFIG)\" -DBUILD_TIME=$(shell date '+%s') -g -Os -fno-strict-aliasing -fomit-frame-pointer -ffunction-sections -fdata-sections

ifneq (,$(findstring cygwin,$(TARGETPLATFORM)))
  TARGET ?= $(LIB_DIR)/libkms.dll
else ifneq (,$(findstring cygnus,$(TARGETPLATFORM)))
  TARGET ?= $(LIB_DIR)/libkms.dll
else ifneq (,$(findstring mingw,$(TARGETPLATFORM)))
  TARGET ?= $(LIB_DIR)/libkms.dll
else ifneq (,$(findstring darwin,$(TARGETPLATFORM)))
  TARGET ?= $(LIB_DIR)/libkms.dylib
  BASECFLAGS += -Wno-deprecated-declarations
else ifneq (,$(findstring linux,$(TARGETPLATFORM)))
  TARGET ?= $(LIB_DIR)/libkms.so
  PICFLAGS += -fPIC
endif

LIBRARY_CFLAGS = -DSIMPLE_SOCKETS -DNO_TIMEOUT -DNO_SIGHUP -DNO_CL_PIDS -DNO_LOG -DNO_RANDOM_EPID -DNO_INI_FILE -DNO_HELP -DNO_CUSTOM_INTERVALS -DNO_PID_FILE -DNO_USER_SWITCH -DNO_VERBOSE_LOG -DNO_LIMIT -DNO_VERSION_INFORMATION -DNO_PRIVATE_IP_DETECT -DNO_STRICT_MODES -DNO_CLIENT_LIST -DNO_TAP -UNO_SOCKETS -USIMPLE_RPC

###########################################################

all: $(TARGET)

$(TARGET) : $(OBJ)
	+@(test -d lib || mkdir lib) & (test -d build || mkdir build)
	+@echo "	LD	$@ <- $(notdir $^)"
	+@$(CC) $(PICFLAGS) $(PLATFORMFLAGS) $(BASECFLAGS) $(CFLAGS) $(LDFLAGS) -fvisibility=hidden -shared -DIS_LIBRARY=1 $(LIBRARY_CFLAGS) -UNO_SOCKETS -UUSE_MSRPC -o $@ $^

$(OBJ_DIR)/%.o : %.c
	+@(test -d lib || mkdir lib) & (test -d build || mkdir build)
	+@echo "	CC	$(notdir $@) <- $<"
	+@$(CC) -x$(COMPILER_LANGUAGE) $(PICFLAGS) $(PLATFORMFLAGS) $(BASECFLAGS) $(CFLAGS) -fvisibility=hidden -c -DIS_LIBRARY=1 $(LIBRARY_CFLAGS) -UNO_SOCKETS -UUSE_MSRPC -Wno-single-bit-bitfield-constant-conversion -o $@ -c $<

.PHONY: clean all

clean:
	rm -f $(OBJ) $(TARGET) && rmdir $(OBJ_DIR) $(LIB_DIR)
